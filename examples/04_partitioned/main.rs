use std::{
    collections::HashMap,
    fs::{remove_dir_all, File},
    io::Read,
    sync::Arc,
};

use deltalake::{
    action::SaveMode,
    arrow::csv::Writer,
    datafusion::prelude::SessionContext,
    writer::{DeltaWriter, JsonWriter},
    DeltaOps, DeltaTableError, SchemaDataType, SchemaField,
};

use serde_json::{json, Value};
#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    let _ = remove_dir_all("./data/http_requests_partitioned");

    // Use [`DeltaOps`] to create a new table at the specified URI.
    // The result with create the first commit of to the tables
    // `_delta_log/` location.
    // This will error if the table already exists.

    // The `memory://` URI is a special URI that will create a table in memory.
    // This is useful for testing and examples, but you can use any URI that
    // is supported by the underlying storage backends (e.g. `s3://`)
    let mut table = DeltaOps::try_from_uri("./data/http_requests_partitioned")
        .await?
        .create()
        .with_table_name("http") // optional table name
        .with_comment("HTTP Request logs")
        .with_columns(vec![
            SchemaField::new(
                "timestamp".to_string(),
                SchemaDataType::primitive("timestamp".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "host".to_string(),
                SchemaDataType::primitive("string".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "method".to_string(),
                SchemaDataType::primitive("string".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "path".to_string(),
                SchemaDataType::primitive("string".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "status_code".to_string(),
                SchemaDataType::primitive("integer".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "size".to_string(),
                SchemaDataType::primitive("long".to_string()),
                true,
                HashMap::new(),
            ),
            SchemaField::new(
                "date".to_string(),
                SchemaDataType::primitive("date".to_string()),
                true,
                HashMap::new(),
            ),
        ])
        .with_save_mode(SaveMode::Append) // or `SaveMode::Overwrite`, `SaveMode::ErrorIfExists`, or `SaveMode::Ignore`
        .with_partition_columns(vec!["date".to_string(), "method".to_string()])
        .await?;

    // populate the table
    // Create a JsonWriter for the table.
    let mut writer = JsonWriter::for_table(&table)?;

    // Load the JSON data from a file.
    let mut buf = String::new();
    let _ = File::open("./http_requests.json")
        .unwrap()
        .read_to_string(&mut buf)
        .unwrap();

    writer
        .write(
            buf.lines()
                .map(|line| {
                    let mut json: Value = serde_json::from_str(line).unwrap();
                    let obj = json.as_object_mut().unwrap();

                    // Add date field from timestamp
                    let date = obj
                        .get("timestamp")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .split('T')
                        .next()
                        .unwrap()
                        .to_string();

                    obj.insert("date".to_string(), json!(date));

                    json
                })
                .collect(),
        )
        .await?;

    // Commit the changes to the table.
    writer.flush_and_commit(&mut table).await?;

    // Query the table by partition
    let ctx = SessionContext::new();

    ctx.register_table("http_requests", Arc::new(table))?;

    let df = ctx.sql("SELECT * FROM http_requests").await?;

    let batches = df.collect().await?;

    let mut writer = Writer::new(std::io::stdout());

    for batch in batches {
        writer.write(&batch)?;
    }

    Ok(())
}

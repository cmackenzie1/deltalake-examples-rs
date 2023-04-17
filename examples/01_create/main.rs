use std::collections::HashMap;

use deltalake::{action::SaveMode, DeltaOps, DeltaTableError, SchemaDataType, SchemaField};

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    // Use [`DeltaOps`] to create a new table at the specified URI.
    // The result with create the first commit of to the tables
    // `_delta_log/` location.
    // This will error if the table already exists.

    // The `memory://` URI is a special URI that will create a table in memory.
    // This is useful for testing and examples, but you can use any URI that
    // is supported by the underlying storage backends (e.g. `s3://`)
    let table = DeltaOps::try_from_uri("./data/http_requests")
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
        ])
        .with_save_mode(SaveMode::Append) // or `SaveMode::Overwrite`, `SaveMode::ErrorIfExists`, or `SaveMode::Ignore`
        .await?;

    println!("{}", table);

    Ok(())
}

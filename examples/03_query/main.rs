use std::sync::Arc;

use deltalake::{datafusion::prelude::SessionContext, DeltaTableError};

use deltalake::arrow::csv::writer::Writer;

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    // Open and load the latest table state from table_uri.
    // This function errors if the table does not already exist.
    let table = deltalake::open_table("./data/http_requests").await?;

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

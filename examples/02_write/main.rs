use std::{fs::File, io::Read};

use deltalake::{
    writer::{DeltaWriter, JsonWriter},
    DeltaTableError,
};

#[tokio::main]
async fn main() -> Result<(), DeltaTableError> {
    // Open the table at the specified URI.
    let mut table = deltalake::open_table("./data/http_requests").await?;

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
                .map(|line| serde_json::from_str(line).unwrap())
                .collect(),
        )
        .await?;

    // Commit the changes to the table.
    writer.flush_and_commit(&mut table).await?;

    Ok(())
}

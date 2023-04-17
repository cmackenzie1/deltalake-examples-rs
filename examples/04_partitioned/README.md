# Partitioned Example

This example shows how to create a partitoned Delta table with a schema. It is partitioned by `method` and a derived column, `date`, that is computed from `timestamp`. The schema is defined for HTTP Requests that look like this:

```json
{
  "timestamp": "2020-01-01T00:00:00Z",
  "host": "example.com",
  "method": "GET",
  "path": "/",
  "status_code": 200,
  "size": 1234
}
```

In order to partition the table, we need to derive a column from the `timestamp` column. We do this by creating a new column called `date` that is a `date` type. We then partition the table by `date` and `method`. Before inserting the JSON into the table, we need to add the `date` column to the JSON.

## Run the example

From the root of the repo, run:

```bash
cargo run --example partitioned
```

# Write Delta Table Example

This example shows how to write to an existing Delta table. The table is created in the [create example](../create/README.md).

The output of this example is a Delta table with the following schema:

```text
root
 |-- timestamp: timestamp (nullable = true)
 |-- host: string (nullable = true)
 |-- method: string (nullable = true)
 |-- path: string (nullable = true)
 |-- status_code: long (nullable = true)
 |-- size: long (nullable = true)
```

And the directory structure looks like this:

```text
http_requests
├── _delta_log
│   ├── 00000000000000000000.json
│   └── 00000000000000000001.json
└── part-00000-b79e213a-cb97-4612-9ffb-508e7cca0459-c000.snappy.parquet
```

## Run the example

From the root of the repo, run:

```bash
cargo run --example write
```

In the [next example](../03_query/README.md), you can see how to query your table.

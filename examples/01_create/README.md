# Create Delta Table Example

This example shows how to create a Delta table with a schema. The schema is defined for HTTP Requests that look like this:

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

The output of this example is a Delta table with the following schema:

```text
root
 |-- timestamp: timestamp (nullable = true)
 |-- host: string (nullable = true)
 |-- method: string (nullable = true)
 |-- path: string (nullable = true)
 |-- status_code: integer (nullable = true)
 |-- size: long (nullable = true)
```

And the directory structure looks like this:

```text
http_requests
└── _delta_log
    └── 00000000000000000000.json
```

## Run the example

From the root of the repo, run:

```bash
cargo run --example create
```

In the [next example](../02_write/README.md), you can see how to start adding data to your table.

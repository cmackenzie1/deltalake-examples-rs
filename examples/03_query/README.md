# Query Delta table using Datafusion

This example shows how to query a Delta table using Datafusion and output the results to the console in CSV format. The table is created in the [create example](../create/README.md) and data is written to it in the [write example](../write/README.md).

## Run the example

```bash
cargo run --example query
```

## Output

```text
timestamp,host,method,path,status_code,size
2023-04-16T10:00:00.000000000,example.com,GET,/,200,1234
2023-04-16T10:01:15.000000000,example.com,POST,/login,401,567
2023-04-16T10:02:30.000000000,example.com,GET,/products,200,4567
2023-04-16T10:03:45.000000000,example.com,PUT,/profile,204,0
2023-04-16T10:04:55.000000000,example.com,DELETE,/account,403,0
2023-04-16T10:05:30.000000000,example.com,GET,/search,200,234
2023-04-16T10:06:20.000000000,example.com,POST,/checkout,201,789
2023-04-16T10:07:10.000000000,example.com,GET,/cart,200,567
2023-04-16T10:08:05.000000000,example.com,PUT,/order,200,123
2023-04-16T10:09:15.000000000,example.com,DELETE,/product/123,204,0
```

In the [next example](../04_partitioned/README.md), you can see how to create a partitioned table for the same data.

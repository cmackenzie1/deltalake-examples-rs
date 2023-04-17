# DeltaLake examples in Rust

Examples showing some common uses of the `deltalake` Rust crate. For more information, see the [deltalake](https://docs.rs/deltalake) crate documentation or the [Delta Lake](https://delta.io/) project.

The examples are meant to illustrate how to use the `deltalake` crate. They are not meant to be production-ready code.
Each example is an iteration on the previous one, so you can see how to build up functionality, step by step.

1. [Create](./examples/01_create/README.md): Create a Delta table.
2. [Write](./examples/02_write/README.md): Write to an existing Delta table.
3. [Query](./examples/03_query/README.md): Read from a Delta table.
4. [Partioned Table](./examples/04_partitioned/README.md): Read/Write data from a partitioned Delta table.

## Run the examples

```bash
cargo run --example <create|write|query|partitioned>
```

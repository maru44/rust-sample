# rust-sample

Start Project

```shell
cargo new rust_sample
```

## Server

Add `axum`

```shell
cargo add axum
# https://crates.io/crates/axum
```

Add with feature

```shell
cargo add serde --features=derive
```

## DB

```shell
cargo install sqlx-cli
```

create migrate file

```shell
sqlx migrate add init
# pwd = infra/db
```

migration

```shell
sqlx migrate run --database-url postgres://root:password@127.0.0.1:5432
```

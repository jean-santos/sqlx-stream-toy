##### sqlx streaming based on sea-orm implementation

```
sqlx database create && sqlx migrate run
cargo run -- add
export DATABASE_URL="sqlite:basic_batched.db"
cargo run
```
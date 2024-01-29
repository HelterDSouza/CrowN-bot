run:
    cargo run
watch:
    cargo watch -q -c -w src -x run
makemigration:
    sqlx migrate add
migrate:
    sqlx migrate run
explode:
    sqlx database drop
db_reset:
    rm database.db
    sqlx database setup

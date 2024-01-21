run:
    cargo run
makemigration:
    sqlx migrate add
migrate:
    sqlx migrate run
explode:
    sqlx database drop
db_reset:
    sqlx database setup

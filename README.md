# Firelight. Rust Version

## Backend
Install [cargo.](https://doc.rust-lang.org/cargo/getting-started/installation.html) Install [diesel.](https://diesel.rs/guides/getting-started).

### Set up DB
Diesel setup will not run because there is a migrations directory already. Create the DB manually:
```
psql -U postgres -c 'CREATE DATABASE firelight-rust;
```
Then run diesel run migrations to create the DB structure.
# What this project this

This repository is a practical implementation of the motimate projects database
and backend layer, but in rust with respective frameworks.
As such this was done mostly as an exercise in learning an ORM for rust and
seeing how far and fast I can go in a language I personally feel more
comfortable with.

# Getting started

Make sure you have rustc installed with cargo. On linux this happens via the tool rustup, which you should install with the preferred method for your OS.
For more details see [The book of Rust - Installation Chapter](https://doc.rust-lang.org/stable/book/ch01-01-installation.html).

Furthermorethe database used is postgres, so make sure it is installed and running.
Resources:

- [Installation from source](https://www.postgresql.org/docs/17/installation.html)

Afterwards run `createdb moticlonedb` to create the postgres database.
To run the server, simply run `cargo run` in the directory.
If compilation fails at the linking step, make sure to have libraries such as
`libpq-dev` installed, though the name may differ on your system.

## Development

Any changes to the schema should be done with the cli tool `diesel-cli`([how to install and use](https://diesel.rs/guides/getting-started.html#installing-diesel-cli)).
To create a new table blueprint run `diesel migration generate`.
To change the database schema in code and postgres, it is recommended to amend
`up.sql` and `down.sql` files in migration/xyz/. The `up.sql` contains sql code to
create a new table, whereas `down.sql` contains code to undo what the former created,
so e.g. `CREATE TABLE foo` for the former and `DROP TABLE foo` for the latter.
Run `diesel migration run` to create the new tables and `diesel migration revert`
to remove the table.

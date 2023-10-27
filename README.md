# IDS706--Rust-CLI-Binary-YuhanXue

![format workflow](https://github.com/nogibjj/IDS706--Rust-CLI-Binary-YuhanXue/actions/workflows/rs_cicd.yml/badge.svg)

This individual project 2 contains Rust scripts/functions that perform CRUD operations in SQLite. 


## Formatting and Erorrs
Please run `make all` to ensure all codes are well-formatted and free of errors.

## Explanation of the Project
In general, this project contains Rust script that perform `load`, `create`, `read`, `update` and `delete` operations on a SQLite database.
Before running any commands, please run `cargo build` to install all dependencies listed under `Cargo.toml` and build the project.

After the project is built, we have the following commands that perform CRUD operations.

`cargo run load`: create a SQLite database named "diabetes.db", within which a table named "diabetes" is created. This command also extracts content from the `diabetes.csv` file and insert each entry in the csv file into the diabetes table as a row.

`cargo run create`: create a new record in the diabetes table in "diabetes.db". The new record is currently hardcoded as `(4, 94, 78, 31, 85, 33.1, 0.446, 22, 1)`.

`cargo run read`: Return the first 10 rows in the diabetes table.

`cargo run update`: Update the row with `id = 1` to be `( 4, 94, 78, 31, 85, 33.1, 0.446, 22, 1)`.

`cargo run delete`: Delete the row with `id = 10`.

Note that any other command will not be recognized and will trigger error message. If you would like to insert/update/delete a particular record, feel free to modify the `main.rs` file. All dependencies are listed under `Cargo.toml`.

If you execute command `load`, `create`, `update` and `delete` in order, between each command you should be able to run `read` command to see how a row in the table has been changed by the command. If any command fails to run, you will see a corresponding error message explaining the reason. 

## Optimized Rust Binary
In `.workflow/rs_cicd.yml`, feel free to change th path of Rust Binary as a GitHub Actions artifact.
```
name: Rust Binary
      uses: actions/upload-artifact@v2
      with:
        name: optimized-binary
        path: target/release/IDP2_yuhan_rust
```
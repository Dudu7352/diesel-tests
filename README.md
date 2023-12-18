# diesel-tests

This repository contains simple diesel code which uses sqlite database specified in .env file.

## How to run

### Setup

1. First make sure you have correctly installed rust toolchain installed
2. Install diesel_cli with command `cargo install diesel_cli --no-default-features --features sqlite`
3. Create .env file with database url specified in a DATABASE_URL variable
4. Run `diesel migration run` command. It will update src/schema.rs as well as prepare database file for us

### Running the code

After the setup process running the code is pretty straightforward. The only thing to do is to execute `cargo run` command.
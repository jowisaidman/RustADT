# Grep command line

This is a simple version of the grep command line written in Rust.

### Usage

To use this simple grep command run:

`cargo run <query> <file_name>`

Example:

`cargo run rust example.txt`

If you are using unix system you can enable the case sensitive grep by setting the env variable:

`export CASE_INSENSITIVE=true`
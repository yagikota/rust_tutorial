# About this command
This command is a simple grep command.
## How to use
* help command
    ```shell
    cargo run -- --help

    rsgrep --help`
    rsgrep 0.1.0

    USAGE:
        rsgrep <PATTERN> [FILE]...

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <PATTERN>
        <FILE>...
    ```

* example
    ```rs
    // main.rs
    fn main() {
        println!("Hello World!");
    }
    ```

    ```shell
    cargo run main ./main.rs
    ...
    ./main.rs: fn main() {
    ```

## Library I used
* rayon(data-parallelism library for Rust)
    * https://github.com/rayon-rs/rayon
* structopt(command line parser for Rust)
    * https://github.com/TeXitoi/structopt
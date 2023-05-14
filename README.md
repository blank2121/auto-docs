# auto-docs

Auto-docs is a command-line interface tool that generates documentation for Rust projects based on source code comments. It's named as a joke after Dixy, the famous documentarian. 

## Installation

Auto-docs is not yet available in any package manager, but you can install it by cloning this repository and running the following command:

```
cargo install --path .
```

## Usage

To use auto-docs, run the following command in your Rust project directory:

```
auto-docs [FLAGS] [OPTIONS]
```

Here are the available flags and options:

```
USAGE:
    auto-docs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -c, --config     Generate base auto_docs.yaml file
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    Changes working directory for auto-doc
```

## License

This project is licensed under the MIT License. 

## Author

This project was created by [blank2121](https://github.com/blank2121) on GitHub.

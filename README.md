# auto-docs

Auto-docs is a command-line interface tool that generates documentation for any code projects based on its source code and comments.

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

## How to use

The CLI has only two functions, but due to the flexible nature of gpt models, will mostlikely have more functionality added to it overtime and may even be renamed to a more 
accurate name in the future. The Primary function is to scan your code and make docs for it. There is one prerequisite and two ways to envoke the CLI.

To generate a base config file in the current directory run:

```
auto-docs --config
```

Or to specify the directory that the config will be generated in run:

```
auto-docs --config --path /path/of/choice
```

To run the core function, use no -c or --config flag. The path behavior is the same and defaults to the current directory and can be specified with the -p or --path option.

## License

This project is licensed under the MIT License. 

## Author

This project was created by [blank2121](https://github.com/blank2121) on GitHub.

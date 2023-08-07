# auto-docs
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

Auto-docs is a command-line interface tool that generates documentation for any code projects based on its source code and comments. It will then create documentation for each individual file that it scans meaning that if it reads a ```main.py``` it will generate a file called ```main.py.md``` containing the docs for that file. Also, this tool uses gpt3.5-turbo underneath the hood so make sure to fine tune the config file to your liking.
## Disclaimer

(you will need an openai api key)

This was originally a project that I personally wanted to make to use the openai api and also used for my school term final. If this idea is liked, then I will expand the functionality and make this project bigger. Also, note that currently the main purpose of this is to make documentation and this tool's demographic is library projects (aka ones with functions that will be used and need to be described).
## Installation

This project is currently not on any package manager, but it can still be installed by downloading the code and using cargo.

First copy this project in any directory of your choise via ```git clone https://github.com/blank2121/auto-docs.git```
then cd into the auto-docs folder and run:
```bash
  cargo install --path .
```
It will then install and can be used anywhere on your local machine.
## Usage

Usage
To use auto-docs, run the following command in your Rust project directory:

```bash
auto-docs [FLAGS] [OPTIONS]
```
Here are the available flags and options:

```bash
USAGE:
    auto-docs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -c, --config     Generate base auto_docs.yaml file
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    Changes working directory for auto-doc
```


## Config File

auto-docs will not work unless there is an ```auto_docs.yaml``` file present. One can be made by running ```auto-docs -c``` in the current directory
or add the ```-p /desired/path``` option to create the config in a specific directory.

```yaml
# Configuration file for My Documentation Project
# powered by chatgpt-3.5-turbo so promts are like asking a chat.openai.com
# the prompt will be exactly what is in the system prompt so be specific

system_prompt: "here is my definition of documentation and how it should be made if you are asked
  to make documentation. Firstly, you will only respond with the documentation and this is how you
  will produce the documentation. In markdown, make a h2 title for the function name and underneath
  write only the initialization line that definition of the function, underneath that write a brief
  description, finally write a code example (h4) for how to use the function and end that function with
  a line break."
lang_specific_information: "none"
ignore_files:
  - 

function_description_length: "max length of a paragraph and a half. try to keep it concise"
include_overall_summary: false

api_key: ""
```

There is a plan for auto-docs to utilize ```.gitignore``` files as well as the built-in option, and only current option, to ignore files and folders.

Take note of the ```lang_specific_information``` field. It is set to none by default but can be used as a second prompting message for language specific information. An example to use this is this: say you wanted to access version information in a rust project via the Cargo.toml file. What you could do is sent the prompt message to:

```yaml
lang_specific_information: "If you recieve a configuation toml file, write a brief
description of the app via the information provided in the file. Make sure to write in in markdown."
```

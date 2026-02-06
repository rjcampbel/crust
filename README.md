# crust

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**crust** is a C compiler written entirely in Rust.


## Table of contents

- [About](#about)
- [Features](#features)
- [Tech stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Quick start](#quick-start)
- [Usage](#usage)
- [License](#license)
- [Contact](#contact)
- [Acknowledgements](#acknowledgements)

## About

**Crust** is my (perhaps naive) attempt at writing a compiler in Rust. This is my first attempt at writing a C compiler, as well as my first substantial Rust program. I have two primary motivations for this project:
1. I've been interested in compilers for a long time now. I worked through "Crafting Interpreters", and found myself looking for my next challenge in this journey.
2. I want to learn Rust.

To facilitate my C compiler journey, I am following in the footsteps of many before and working through Nora Sandler's fantastic book, "Writing a C Compiler".

## Features

- [x] Chapter 1
- [x] Chapter 2
- [x] Chapter 3
- [x] Chapter 4
- [x] Chapter 5
- [x] Chapter 6
- [ ] Chapter 7
- [ ] Chapter 8
- [ ] Chapter 9

I've currently worked through Chapter 6 of the book. At this point, the only programs that can be compiled are ones with a single function, `main`, that contains any of the features listed above.

## Tech stack

- Language: Rust

## Prerequisites

- GCC/Clang
- Rust
- Git

## Quick start

### Clone the repo

```bash
git clone https://github.com/rjcampbel/crust.git
cd crust
```

### Setup (example: Node.js)

```bash
cargo build
cargo run
```

## Usage

```bash
cargo run <SOURCE>
```

```bash
Usage: crust [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Path to C source file to compile. Running without any additional arguments, or with only print_* arguments, will run all stages of the compiler and generate an executable in the same directory as the source file

Options:
  -l, --lex             Run only the lexer
  -p, --parse           Run only the lexer and parser
  -v, --validate        Run only the lexer, parser, and validator
  -t, --tacky           Run only the lexer, parser, validator, and tacky generation
  -c, --codegen         Run only the lexer, parser, validator, tacky generation, and assembly generation
      --print-tokens    Print all the scanned tokens
      --print-ast       Print the AST after parsing
      --print-tacky     Print the tacky ast
      --print-assembly  Print the assembly AST after code generation
  -h, --help            Print help
  -V, --version         Print version
```

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

## Contact

Project lead / maintainer: Ryan Campbell — [GitHub profile](https://github.com/rjcampbel)

Report issues at: [https://github.com/rjcampbel/crust/issues](https://github.com/rjcampbel/crust/issues)

## Acknowledgements


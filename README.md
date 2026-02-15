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

## About

**Crust** is my (perhaps naive) attempt at writing a compiler in Rust. This is my first attempt at writing a C compiler, as well as my first substantial Rust program. I have two primary motivations for this project:
1. I've been interested in compilers for a long time now. I worked through "Crafting Interpreters", and found myself looking for my next challenge in this journey.
2. I want to learn Rust.

To facilitate my C compiler journey, I am following in the footsteps of many before and working through Nora Sandler's fantastic book, "Writing a C Compiler".

## Features

- [x] Chapter 1 - Basics
- [x] Chapter 2 - Unary Operators
  - [x] Negation
  - [x] Bitwise Complement
- [x] Chapter 3 - Binary Operators
  - [x] Addition
  - [x] Subtraction
  - [x] Multiplication
  - [x] Division
  - [x] Remainder
  - [x] Extra Credit: Bitwise AND, OR, XOR, left shift, right shift
- [x] Chapter 4 - Logical and Relational Operators
  - [x] Logical Not, Logical And, Logical Or
  - [x] Equal, Not Equal
  - [x] Less Than, Greater Than, Less than or equal, Greater than or equal
- [ ] Chapter 5 - Local variables
  - [x] Local variable declarations
  - [x] Local variable assignments
  - [x] Local variable resolution
  - [x] Extra Credit: Compound assignment (+=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=)
  - [ ] Extra Credit: Prefix and postfix increment and decrement
- [ ] Chapter 6 - If Statements and Conditional Expressions
  - [x] If/Else statements without compound statements
  - [x] Conditional (ternary) operator
  - [x] Function `main` supports all previous chapters plus if and conditional statements and expressions
  - [ ] Extra Credit: `goto`
- [x] Chapter 7 - Compound statements
  - [x] Nested scopes 
  - [x] Variable scope resolutions
  - [x] If statements with compound statements
  - [x] Function `main` can now include arbitrary levels of nested scopes 
- [ ] Chapter 8 - Loops
  - [x] `while`, `do`...`while`, `for` loops
  - [x] `continue`, `break`
  - [ ] Extra Credit: `switch` statements
- [x] Chapter 9 - Functions
- [ ] Chapter 10 - File Scope Variable Declarations and Storage-Class Specifiers

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

### Setup

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
  -l, --lex               Run only the lexer
  -p, --parse             Run lexer and parser
  -v, --validate          Run lexer, parser, and validator
  -t, --tacky             Run lexer, parser, validator, and IR generation
  -c, --codegen           Run lexer, parser, validator, IR generation, and assembly generation
      --print-tokens      Print all the scanned tokens
      --print-ast         Print the AST after parsing and validating
      --print-tacky       Print the IR
      --print-assembly    Print the assembly
      --args [<ARGS>...]  Additional arguments to pass to the assembler
  -h, --help              Print help
  -V, --version           Print version
```

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

## Contact

Project lead / maintainer: Ryan Campbell — [GitHub profile](https://github.com/rjcampbel)

Report issues at: [https://github.com/rjcampbel/crust/issues](https://github.com/rjcampbel/crust/issues)


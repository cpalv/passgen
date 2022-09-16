# Password Generator in Python and Rust

Generate a complex password with python or rust!

## Installation

`git clone https://github.com/cpalv/passgen.git`

## Bring Your Own Interpreter

Copy/move `passgen.py` to one of your $PATH directories or run from passgen directory `./passgen.py`

## Put in the appropriate toolbox

Requires `cargo`

```
$ cd passgen/rustypass 
$ cargo build --release
```

Copy/move `target/release/rustypass` to one of your $PATH directories or run from rustypass directory

## Usage

Both programs have `-l` and `-s` options.
`-l` will set the length of the password generated
`-s` will add special characters "!@#$%^&*()_-+="

`-h` and `--help` are available to get help quickly from commandline.

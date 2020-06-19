# mdconv - a simple cli markdown to html converter
[![Crates.io](https://img.shields.io/crates/v/mdconv)](https://crates.io/crates/mdconv)
[![Crates.io](https://img.shields.io/crates/l/mdconv)](https://crates.io/crates/mdconv)
[![Continuous Deployment](https://github.com/groow-dev/mdconv/workflows/Continuous%20Deployment/badge.svg)](https://github.com/groow-dev/mdconv/actions)

## Installation
Pre-built binaries are available [on the GitHub Releases tab](https://github.com/groow-dev/mdconv/releases).

```console
# install from crates.io
$ cargo install mdconv

# upgrade
$ cargo install --force mdconv

# clone, build and install
$ git clone https://github.com/groow-dev/mdconv.git
$ cd mdconv
$ cargo install --path .
```

## Usage

```console
# using stdin and stdout
$ mdconv

# stdin to output file
$ mdconv -o output.html

# input file to stdout
$ mdconv -i input.md

# input file to output file
$ mdconv -i input.md  -o output.html
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions. 
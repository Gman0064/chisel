# chisel
`chisel` is a tool for decompiling *nix ELF programs for binary analysis and reverse engineering. This project is being developed for assignments pertaining to Auburn University's **COMP5970 Binary Program Analysis** course.


## Supported Binary formats

`chisel` supports binaries compiled to the [ELF format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) from most x86/x64 *nix systems, and __does not__ currently support macOS Mach-O or Windows PE binaries.

> Due to an indexing bug, current iterations of chisel do not support 32-bit x86 applications


## Building and Installing

To build and install `chisel`, use the following steps:

```shell
$ git clone git@github.com:Gman0064/chisel.git

$ cd chisel && cargo install
```


## Usage

```shell
$ chisel [path to ELF executable]
```

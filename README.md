# chisel
`chisel` is a tool for decompiling *nix ELF programs for binary analysis and reverse engineering. This project is being developed for assignments pertaining to Auburn University's **COMP5970 Binary Program Analysis** course.

## Binary Analysis Technique

`chisel` uses a linear dissassembly approach for converting the ELF text section into x86 assembly instructions. Future implementations will have more advances analysis techniques such as recursive descent, as well as potential binary patching.


## Supported formats

`chisel` supports binaries compiled to the [ELF format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) from most x86/x64 *nix systems, and __does not__ currently support macOS Mach-O or Windows PE binaries.

> Due to an indexing bug, current iterations of chisel do not support 32-bit x86 applications. Only x86-64 is supported.


## Building and Installing

To build and install `chisel`, use the following steps:

```shell
$ git clone git@github.com:Gman0064/chisel.git

$ cd chisel && cargo install
```


## Usage

The following format can be used to pass a binary to `chisel` to analyze. The repository also includes a few binaries in `./testing/` as included examples. `chisel` also supports binary rewriting/patching by specifying the `-p` flag along with a `.bin` file of assembly code you would like to inject.

```shell
$ chisel [EXECUTABLE] [-p] [PATCH_FILE]
```

> Binary patching is currently very buggy and may cause segmentation faults with the patched binary
# chisel
`chisel` is a tool for decompiling *nix ELF programs for binary analysis and reverse engineering. This project is being developed for assignments pertaining to Auburn University's **COMP5970 Binary Program Analysis** course.


## Supported Binary formats

`chisel` supports binaries compiled to the [ELF format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format) from most x86/x64 *nix systems, and __does not__ currently support macOS Mach-O or Windows PE binaries.
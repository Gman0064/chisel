# ELF Code injection notes

- Locate PT_NOTE segments in binary
- Locate a suitable `.note.*` section for replacing code
- Modify the injection code section type from `SHT_NOTE` to `SHT_PROGBITS`
- Update relevant start, offset, and end addresses for new section
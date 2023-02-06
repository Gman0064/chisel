# Lecture notes 1/31/2023

## Setup
- Load file into byte array
- check if ELF
- parse file header
- get offset and length of text section
    - virtual addresses and size
- get function addresses

## Disassembly
- Use something like [Capstone]() 
    - feed address and offset in linear sweep to find instruction
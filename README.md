# CHIP-8 Emulator

This project is a CHIP-8 emulator written in Rust. CHIP-8 is a simple, interpreted programming language used in the 1970s for creating games on 8-bit microcomputers. This emulator allows you to run CHIP-8 programs and games on modern systems.

## Specs

- 4 KB of RAM (512 Bytes reserved for the interpreter)
- 16 general purpose 8 bit registers labelled V0, V1 .... V9, VA, VB ..... VF.
- Special 16 bit register called I to store memory addresses
- 2 special purpose 8 bit registers for display timers and sound timers.
- 16 bit program counter to store the currently executing address
- 8 bit stack pointer to point towards the topmost element in the stack.
- Stack is an array of 16 16-bit values.

## Technical reference

http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0.0

## Key Mappings

The following key mappings are used to emulate the CHIP-8 keypad:

```
CHIP-8 Keypad:    Emulator Keypad:

1 2 3 C           1 2 3 4
4 5 6 D    =>     Q W E R
7 8 9 E           A S D F
A 0 B F           Z X C V
```

## Getting Started

1. Clone the repository:
    ```bash
    git clone https://github.com/aayush0325/chip8.git
    cd chip8
    ```

2. Build the project:
    ```bash
    cd desktop
    cargo build --release
    ```

3. Run a CHIP-8 ROM:
    ```bash
    cargo run --release path/to/rom
    ```

4. Try out some sample games:
    The `c8games` directory in the repository contains a collection of CHIP-8 games you can run with the emulator. Simply specify the path to a game file in the `c8games` directory when running the emulator.

5. For example:

    ```bash
    cargo run --release ../c8games/PONG
    ```

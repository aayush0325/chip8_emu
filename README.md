# CHIP-8 Emulator

This project is a high-performance CHIP-8 emulator meticulously crafted in Rust, bringing the nostalgic 8-bit microcomputer experience to modern platforms. Beyond a mere interpreter, this emulator showcases a robust, modular architecture designed for optimal performance and cross-platform compatibility.

## Development Process: A Deep Dive

At its heart, the emulator features a highly optimized **core module** encapsulating all CHIP-8 CPU instructions, memory management, and peripheral interactions. This foundational Rust crate ensures precise emulation of the original hardware's intricacies.

For a native desktop experience, we leveraged **SDL2** to provide a low-latency, high-fidelity graphical and input interface. This integration allows for seamless execution of classic CHIP-8 ROMs directly on your desktop, delivering a fluid and responsive gameplay experience.

Pushing the boundaries of retro emulation, this project also extends its reach to the web. By compiling the core Rust module to **WebAssembly (Wasm)**, we've enabled the emulator to run directly within modern web browsers. This innovative approach allows for instant, plugin-free access to CHIP-8 games, demonstrating the power of Wasm for high-performance, platform-agnostic applications.

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
    git clone https://github.com/aayush0325/chip8_emu.git
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

## To compile the WebAssembly

```bash
cd web && cargo build && wasm-pack build --target web
```

## To run the emulator in your browser

```bash
cd web && python3 -m http.server
```
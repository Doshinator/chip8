# CHIP-8 Emulator

A CHIP-8 emulator written in Rust.

## Status

The CHIP-8 instruction set and core emulator functionality are implemented. The emulator can load and execute CHIP-8 ROMs and has been tested with working games and test ROMs. This project is considered complete as a CHIP-8 emulator.

Future improvements would primarily be code quality, configuration, portability, and additional tooling rather than implementing the CHIP-8 specification itself.

## Features

- Full CHIP-8 instruction set
- 4 KB memory
- 16 8-bit general-purpose registers (`V0`–`VF`)
- 16-key CHIP-8 keypad
- Delay and sound timers running at 60 Hz
- 64 × 32 monochrome display
- Sprite drawing with collision detection
- Keyboard input mapped to the CHIP-8 keypad
- ROM loading with size validation
- Configurable CPU execution loop
- CLI ROM selection
- ROM rendering through `minifb`

## Usage

```sh
cargo run -- <path-to-rom>
```

```sh
cargo run -- roms/pong.ch8
```

Build an optimized release version:

```sh
cargo build --release
cargo run --release -- roms/pong.ch8
```

## Architecture

```text
             ┌──────────────┐
             │     CLI      │
             └──────┬───────┘
                    │
             ┌──────▼───────┐
             │    Chip8     │
             │              │
             │  CPU         │
             │  Memory      │
             │  Registers   │
             │  Timers      │
             │  Display     │
             │  Keypad      │
             └───┬──────┬───┘
                 │      │
        ┌────────▼─┐  ┌─▼────────┐
        │   Input  │  │  Render  │
        │          │  │          │
        │ Keyboard │  │ minifb   │
        └──────────┘  └──────────┘
```

### CPU

The CPU performs the standard CHIP-8 fetch/decode/execute cycle and runs independently from the 60 Hz timers and display rendering.

```
Fetch opcode → Decode instruction → Execute instruction → Update PC/state
```

### Memory

CHIP-8 provides 4096 bytes of memory. Programs are loaded starting at `0x200`. `Chip8::load_rom()` validates that the ROM fits in available memory before loading.

### Display

The CHIP-8 display is 64 × 32. The emulator scales this to a 640 × 320 window via `minifb`.

### Keypad

CHIP-8 has 16 keys mapped to a physical keyboard as follows:

| CHIP-8 | Keyboard |
|--------|----------|
| `1 2 3 C` | `1 2 3 4` |
| `4 5 6 D` | `Q W E R` |
| `7 8 9 E` | `A S D F` |
| `A 0 B F` | `Z X C V` |

## Timing

| Component | Rate   |
|-----------|--------|
| CPU       | 500 Hz |
| Timers    | 60 Hz  |
| Frame     | 60 FPS |

## Project Structure

```
src/
├── chip8.rs       # CHIP-8 CPU and machine state
├── instruction.rs # Instruction definitions
├── decode.rs      # Opcode decoding
├── registers.rs   # V0–VF registers
├── memory.rs      # Memory representation
├── stack.rs       # Subroutine stack
├── display.rs     # 64 × 32 display
├── keypad.rs      # CHIP-8 keypad
├── input.rs       # Physical keyboard → CHIP-8 keypad
├── render.rs      # minifb rendering
└── main.rs        # CLI and emulator loop
```

## Testing

```sh
cargo test
```

Tests cover opcode decoding, instruction execution, register operations, stack operations, keypad behavior, display behavior, ROM loading, invalid ROM handling, and CPU execution.

## ROMs

Tested with IBM logo, Pong, CHIP-8 test ROMs, and custom instruction-testing ROMs. ROMs are not included with this project.

## Dependencies

- [`minifb`](https://github.com/emoon/rust_minifb) — window creation and framebuffer rendering

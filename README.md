# CHIP-8 Emulator

A CHIP-8 emulator written in Rust, built as a learning project to practice idiomatic Rust, CPU/emulator design, state management, and breaking a system into small abstractions.

The goal is not just to finish an emulator, but to understand **why the code is designed the way it is**.

## Project Structure

- `chip8.rs` — CPU/emulator state, fetch/decode/execute cycle, and instruction execution
- `decode.rs` — converts CHIP-8 opcodes into typed instructions
- `instruction.rs` — instruction representation
- `registers.rs` — V registers and register access
- `display.rs` — 64×32 display and sprite drawing
- `keypad.rs` — CHIP-8 keypad state
- `timer.rs` — delay/sound timer behavior
- `stack.rs` — call/return stack

## Current CPU Cycle

The emulator follows the basic cycle:

```text
fetch → decode → execute
```

`tick()` performs one CPU step. Most instructions advance the PC during `fetch()`, while instructions such as jumps and skips modify it as part of execution.

---

# Learning Notes

## Drawing Sprites — `DXYN`

The important mental model:

> A sprite is literally the drawing encoded as bits in memory.

For example:

```text
11110000
```

means:

```text
* * * *
```

Each byte is one row of the sprite, and each bit is one horizontal pixel.

`DXYN` gets:

- `I` — the memory address where the sprite starts
- `N` — how many rows to read
- `Vx` — starting X coordinate
- `Vy` — starting Y coordinate

So the sprite is effectively:

```rust
let sprite = &memory[I as usize .. I as usize + N as usize];
```

The coordinates tell us **where the top-left corner of that sprite begins on the screen**.

If:

```text
Vx = 10
Vy = 5
```

then sprite row `0`, column `0` maps to:

```text
screen_x = 10 + 0
screen_y = 5 + 0
```

The next bit:

```text
screen_x = 10 + 1
screen_y = 5 + 0
```

The next row:

```text
screen_x = 10 + 0
screen_y = 5 + 1
```

So:

```text
screen_x = start_x + sprite_col
screen_y = start_y + sprite_row
```

### Wrapping

The display is:

```text
64 × 32
```

Coordinates wrap around the edges:

```rust
let pos_x = (start_x + sprite_col) % WIDTH;
let pos_y = (start_y + sprite_row) % HEIGHT;
```

For example:

```text
x = 63
sprite_col = 1

(63 + 1) % 64 = 0
```

The pixel wraps back to the left side.

### XOR and Collision

CHIP-8 draws using XOR:

```text
0 XOR 1 = 1
1 XOR 1 = 0
```

Therefore, drawing a pixel over an existing pixel turns it off.

That is the collision condition.

So `draw_sprite()`:

1. Checks whether the destination pixel is already on.
2. Records a collision if it is.
3. XORs the pixel.
4. Returns whether any collision occurred.

The display owns this behavior because **the display knows about pixels, coordinates, wrapping, and drawing**. The CPU only needs to know that `DXYN` asks the display to draw a sprite and receives a collision result.

---

## Waiting for a Key — `FX0A`

`FX0A` initially felt like it required a blocking loop:

```text
wait until key pressed
```

But the emulator should not literally block inside `execute()`.

Instead, the CPU remembers that it is waiting:

```rust
waiting_for_key: Option<Register>
```

The two states mean:

```text
None
→ CPU is not waiting

Some(VA)
→ CPU is waiting for a key and should
  store the key in VA
```

`Option<Register>` is useful because we need to remember both:

1. whether the CPU is waiting
2. which register should receive the key

### What happens

Suppose:

```text
0x200 = FX0A
0x202 = next instruction
```

First tick:

```text
fetch FX0A
PC = 0x202
execute FX0A
waiting_for_key = Some(VA)
return
```

Notice that the PC has already advanced to `0x202`.

Next tick:

```text
waiting_for_key = Some(VA)
        ↓
check keypad
        ↓
no key
        ↓
return Ok(())
```

Because we return **before `fetch()`**, the next instruction is not executed.

The PC remains:

```text
0x202
```

This can happen for as many ticks as necessary.

Eventually:

```text
waiting_for_key = Some(VA)
        ↓
key A is pressed
        ↓
VA = 0xA
        ↓
waiting_for_key = None
        ↓
return
```

The next tick sees:

```text
waiting_for_key = None
```

and normal execution resumes by fetching the instruction at `0x202`.

### The important insight

The CPU is not literally paused.

It is **remembering its state**.

Each call to `tick()` asks:

> "Given my current state, what should I do now?"

That is a small example of modeling the emulator as a **state machine**.

---

# Design Lessons

A useful question when designing each component is:

> **What does the system need to know, and what does it need to do?**

Then give that knowledge and behavior to the component that owns it.

For example:

```text
Keypad
  knows → which keys are pressed
  does  → press, release, check keys

Display
  knows → pixel state
  does  → clear, draw sprites

Chip8
  knows → CPU/emulator state
  does  → fetch, decode, execute, tick
```

The goal of this project is to get better at recognizing those responsibilities instead of immediately jumping to:

> "Should this be a struct or an enum?"

Start with the **behavior and state**. The Rust representation usually becomes much clearer afterward.

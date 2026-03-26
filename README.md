# WS2812B Rust Driver (no_std)

[![Crates.io](https://img.shields.io/crates/v/ws2812_rs.svg)](https://crates.io/crates/ws2812_rs)
[![Docs.rs](https://docs.rs/ws2812_rs/badge.svg)](https://docs.rs/ws2812_rs)

A lightweight, platform-agnostic Rust driver for WS2812B RGB LEDs. Designed with embedded systems in mind, this crate provides configurable timing strategies for controlling LED strips using only a data line. Compatible with `#![no_std]`, it now includes full support for both synchronous (`embedded-hal`) and asynchronous (`embedded-hal-async` / `embassy-time`) environments.

## Features

* **1-Wire RGB Protocol:** Fully compatible with WS2812B LEDs.
* **Multiple Sync Timing Strategies:** `own_delay`: Uses an external delay provider implementing `DelayNs`.
    * `manual_delay`: Expects delay objects at each function call.
    * `spinloop_delay` (Default): Pure spin-loop timing using CPU frequency.
* **Async Support:** First-class async/await support via the `async` feature, leveraging `embedded-hal-async` and `embassy-time`.
* **Rich Color Library:** Built-in modular `Color` struct with pre-defined constants (Red, Green, Blue, Cyan, Magenta, Yellow, White, Orange, Purple, Pink, Brown).
* **Highly Flexible:** Perfect for both bare-metal boards without advanced peripherals and modern async embedded ecosystems.

## Configuration

Enable your desired timing backend by activating one of the following **features** in your `Cargo.toml`:

| Feature | Description |
| :--- | :--- |
| `spinloop_delay` | **(Default)** Delay through CPU spin-loops using a known CPU frequency. |
| `own_delay` | Passes a mutable reference to an `embedded_hal::delay::DelayNs` trait impl upon driver creation. |
| `manual_delay` | Requires a delay provider to be passed explicitly at each call to `send_color`. |
| `async` | Enables asynchronous operation (`AsyncGlowColor` trait / `send_color_w_embassy`), requiring `embedded-hal-async` and `embassy-time`. |

> **Note:** You should only enable **one** of the synchronous delay strategies (`spinloop_delay`, `own_delay`, or `manual_delay`) at a time. The `async` feature can be layered on top of them.

## Usage

### `Cargo.toml`

```toml
[dependencies]
ws2812b = "*" # Replace with the actual version
embedded-hal = "1.0" 

# Only required if using the `async` feature:
embedded-hal-async = "1.0"
embassy-time = "0.3"
```

### Core Traits & Methods

Depending on your feature flags, you will interact with the LED strip using one of the following traits:

#### 1. Synchronous Operation (`GlowColor`)
Available by default. Use this for blocking execution.

* `send_color([Color; N])`: Sends an array of colors to the LED strip. (Usage changes slightly if `manual_delay` is enabled, as it will require a delay reference).

#### 2. Asynchronous Operation (`AsyncGlowColor`)
Available when the `async` feature is enabled. Use this in async contexts (e.g., Embassy).

* `async_send_color([Color; N])`: Asynchronously drives the LED pins, awaiting on high/low pin states and nanosecond delays. 
* `send_color_w_embassy([Color; N])`: A dedicated async method available on the standard `GlowColor` trait that specifically uses `embassy_time::Timer` for precision non-blocking delays.

### The `Color` Struct

You can easily define custom colors using RGB values or use the built-in constant methods:

```rust
use ws2812b::Color;

// Custom RGB
let custom_color = Color([100, 50, 200]);

// Built-in presets
let red = Color::red();
let cyan = Color::cyan();
let orange = Color::orange();
// Also available: green(), blue(), magenta(), yellow(), white(), purple(), pink(), brown()
```

### Driver Instantiation Examples

**Using `spinloop_delay` (Default)**
```rust
// Requires the output pin and your board's CPU frequency in Hz
let mut ws2812 = WS2812::new(led_pin, 160_000_000); 
```

**Using `own_delay`**
```rust
// Requires the output pin and a mutable reference to a DelayNs provider
let mut ws2812 = WS2812::new(led_pin, &mut delay_provider);
```

**Using `manual_delay`**
```rust
// Requires only the output pin upon creation
let mut ws2812 = WS2812::new(led_pin);

// The delay provider is passed during the transfer
ws2812.send_color([Color::red()], &mut delay_provider);
```

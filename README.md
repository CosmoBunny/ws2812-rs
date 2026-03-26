# WS2812B Rust Driver (no std)

[![Crates.io](https://img.shields.io/crates/v/ws2812_rs.svg)](https://crates.io/crates/ws2812_rs)
[![Docs.rs](https://docs.rs/ws2812_rs/badge.svg)](https://docs.rs/ws2812_rs)

<img width="2048" height="1536" alt="image" src="https://github.com/user-attachments/assets/8bd78753-7b22-4ade-92c1-06de3d2b1af1" />

A lightweight, platform-agnostic Rust driver for WS2812B RGB LEDs. Designed with embedded systems in mind, this crate provides configurable timing strategies for controlling LED strips. It supports traditional bit-banging over a single data line, as well as highly reliable hardware SPI waveform generation. Compatible with `#![no_std]`, it includes full support for synchronous (`embedded-hal`) and asynchronous (`embedded-hal-async` / `embassy-time`) environments.

## Features

  * **1-Wire RGB Protocol:** Fully compatible with WS2812B LEDs.
  * **Hardware SPI Support:** Generate strict timing waveforms automatically using an SPI MOSI pin, completely eliminating Wi-Fi/RTOS interrupt glitches. Uses dynamic frequency calculation to guarantee timing accuracy.
  * **Multiple Sync Timing Strategies:** \* `own_delay`: Uses an external delay provider implementing `DelayNs`.
      * `manual`: Expects delay objects at each function call.
      * `spinloop_delay` (Default): Pure spin-loop timing using CPU frequency.
  * **Async Support:** First-class async/await support via the `async` feature, leveraging `embedded-hal-async` and `embassy-time`.
  * **Rich Color Library:** Built-in modular `Color` struct with pre-defined constants (Red, Green, Blue, Cyan, Magenta, Yellow, White, Orange, Purple, Pink, Brown).

## Configuration

Enable your desired bit-banging timing backend by activating one of the following **features** in your `Cargo.toml`. *(Note: SPI support does not require a specific timing feature).*

| Feature | Description |
| :--- | :--- |
| `spinloop_delay` | **(Default)** Delay through CPU spin-loops using a known CPU frequency. |
| `own_delay` | Passes a mutable reference to an `embedded_hal::delay::DelayNs` trait impl upon driver creation. |
| `manual` | Requires a delay provider to be passed explicitly at each call to `send_color`. |
| `async` | Enables asynchronous operation (`AsyncGlowColor` trait / `send_color_w_embassy`), requiring `embedded-hal-async` and `embassy-time`. |

> **Note:** You should only enable **one** of the synchronous delay strategies (`spinloop_delay`, `own_delay`, or `manual`) at a time. The `async` feature can be layered on top of them.

## Usage

### `Cargo.toml`

```toml
[dependencies]
ws2812_rs = "*" # Replace with the actual version
embedded-hal = "1.0" 

# Only required if using the `async` feature:
embedded-hal-async = "1.0"
embassy-time = "0.3"
```

### Core Traits & Methods

Depending on your hardware setup, you will interact with the LED strip using one of the following traits:

#### 1\. Hardware SPI Operation (`SendColorBySPI`)

The most stable method for systems running Wi-Fi or RTOS.

  * `write_colors([Color; N], freq)`: Dynamically translates colors into SPI bit-patterns based on your SPI bus frequency and sends them to the strip.

#### 2\. Synchronous GPIO Operation (`GlowColor`)

Available by default for traditional bit-banging. Use this for blocking execution on simpler boards.

  * `send_color([Color; N])`: Sends an array of colors to the LED strip. (Requires a delay reference if `manual` is enabled).

#### 3\. Asynchronous GPIO Operation (`AsyncGlowColor`)

Available when the `async` feature is enabled. Use this in async contexts (e.g., Embassy).

  * `async_send_color([Color; N])`: Asynchronously drives the LED pins, awaiting on high/low pin states and nanosecond delays.
  * `send_color_w_embassy([Color; N])`: A dedicated async method utilizing `embassy_time::Timer` for precision non-blocking delays.

### The `Color` Struct

You can easily define custom colors using RGB values or use the built-in constant methods:

```rust
use ws2812_rs::Color;

// Custom RGB
let custom_color = Color([100, 50, 200]);

// Built-in presets
let red = Color::red();
let cyan = Color::cyan();
let orange = Color::orange();
// Also available: green(), blue(), magenta(), yellow(), white(), purple(), pink(), brown()
```

### Driver Instantiation Examples

**1. Using Hardware SPI (Recommended for ESP32/Wi-Fi)**

```rust
use ws2812_rs::{WS2812SPI, SendColorBySPI, Color};

// Assuming `spi` is an initialized embedded_hal SpiBus running at ~3.2 MHz
let mut ws2812 = WS2812SPI { led: spi };

// Write colors dynamically by passing the current SPI frequency
ws2812.write_colors([Color::red(), Color::blue()], 3_200_000).unwrap();
```

**2. Using `spinloop_delay` (Default Bit-Banging)**

```rust
// Requires the output pin and your board's CPU frequency in Hz
let mut ws2812 = WS2812::new(led_pin, 160_000_000); 
ws2812.send_color([Color::green()]);
```

**3. Using `own_delay`**

```rust
// Requires the output pin and a mutable reference to a DelayNs provider
let mut ws2812 = WS2812::new(led_pin, &mut delay_provider);
```

**4. Using `manual`**

```rust
// Requires only the output pin upon creation
let mut ws2812 = WS2812::new(led_pin);

// The delay provider is passed during the transfer
ws2812.send_color([Color::red()], &mut delay_provider);
```

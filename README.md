# WS2812B Rust Driver (no_std)

A lightweight, platform-agnostic Rust driver for WS2812B RGB LEDs. Designed with embedded systems in mind, this crate provides configurable timing strategies for controlling LED strips using only a data line. Compatible with `#![no_std]`.

## ✨ Features

- WS2812B-compatible 1-wire RGB protocol
- Multiple timing strategies:
  - `own_delay`: Uses an external delay provider implementing `DelayNs`
  - `manual_delay`: Expects delay objects at function call
  - `spinloop_delay`: Pure spin-loop timing using CPU frequency
- Modular color abstraction via the `Color` struct
- Flexible for boards without advanced peripherals
- Compile-time control via Cargo features

## 🛠 Configuration

Enable your desired timing backend by activating one of the following **features**:

| Feature                     | Description                                       |
|-----------------------------|---------------------------------------------------|
| `own_delay`                 | Use a mutable reference to a `DelayNs` trait impl |
| `manual_delay`              | Provide delay at each call to `send_color`        |
| `spinloop_delay`(default)   | Delay through CPU spin-loops with known frequency |

> **Note:** Only one feature should be enabled at a time.

## 📦 Usage

### `Cargo.toml`

```toml
[dependencies]
ws2812b = "*" 
embedded-hal = "1.*" # or compatible version
```

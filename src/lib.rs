#![no_std]

use embedded_hal::digital::OutputPin;

#[cfg(not(feature = "spinloop_delay"))]
use embedded_hal::delay::DelayNs;

#[cfg(feature = "own_delay")]
pub struct WS2812<'a, LED: OutputPin, D: DelayNs> {
    led: LED,
    delay: &'a mut D,
}

#[cfg(feature = "manual_delay")]
pub struct WS2812<LED: OutputPin> {
    led: LED,
}
#[cfg(feature = "spinloop_delay")]
pub struct WS2812<LED: OutputPin> {
    led: LED,
    cpu_freq: u64,
}

#[cfg(feature = "own_delay")]
impl<'a, LED: OutputPin, D: DelayNs> WS2812<'a, LED, D> {
    pub fn new(led: LED, delay: &'a mut D) -> Self {
        Self { led, delay }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}

#[cfg(feature = "manual_delay")]
impl<LED: OutputPin> WS2812<LED> {
    pub fn new(led: LED) -> Self {
        Self { led }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}
#[cfg(feature = "spinloop_delay")]
impl<LED: OutputPin> WS2812<LED> {
    pub fn new(led: LED, cpu_freq: u64) -> Self {
        Self { led, cpu_freq }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}

pub trait GlowColor {
    fn t0h() -> u32;
    fn t1h() -> u32;
    fn t0l() -> u32;
    fn t1l() -> u32;
    fn reset() -> u32;

    #[cfg(not(feature = "manual_delay"))]
    fn delay_ns(&mut self, ns: u32);
    fn led_low(&mut self);
    fn led_high(&mut self);

    #[cfg(not(feature = "manual_delay"))]
    fn send_color<const N: usize>(&mut self, color: [Color; N]) {
        for each_color in color {
            for byte in each_color.0 {
                for bit in (0..8).rev() {
                    if (byte & (1 << bit)) != 0 {
                        // Logic 1
                        self.led_high();
                        self.delay_ns(Self::t1h());
                        self.led_low();
                        self.delay_ns(Self::t1l());
                    } else {
                        // Logic 0
                        self.led_high();
                        self.delay_ns(Self::t0h());
                        self.led_low();
                        self.delay_ns(Self::t0l());
                    }
                }
            }
        }
        self.led_low();
        self.delay_ns(Self::reset())
    }

    #[cfg(feature = "manual_delay")]
    fn send_color<const N: usize, D: DelayNs>(&mut self, color: [Color; N], delay: &mut D) {
        for each_color in color {
            for byte in each_color.0 {
                for bit in (0..8).rev() {
                    if (byte & (1 << bit)) != 0 {
                        // Logic 1
                        self.led_high();
                        delay.delay_ns(Self::t1h());
                        self.led_low();
                        delay.delay_ns(Self::t1l());
                    } else {
                        // Logic 0
                        self.led_high();
                        delay.delay_ns(Self::t0h());
                        self.led_low();
                        delay.delay_ns(Self::t0l());
                    }
                }
            }
        }
        self.led_low();
        delay.delay_ns(Self::reset())
    }
}

#[derive(Default, Clone)]
pub struct Color(pub [u8; 3]);

impl Color {
    pub const fn red() -> Self {
        Self([255, 0, 0])
    }

    pub const fn green() -> Self {
        Self([0, 255, 0]) // Green
    }

    pub const fn blue() -> Self {
        Self([0, 0, 255]) // Blue
    }

    pub const fn cyan() -> Self {
        Self([0, 255, 255]) // Cyan (Green + Blue)
    }

    pub const fn magenta() -> Self {
        Self([255, 0, 255]) // Magenta (Red + Blue)
    }

    pub const fn yellow() -> Self {
        Self([255, 255, 0]) // Yellow (Red + Green)
    }

    pub const fn white() -> Self {
        Self([255, 255, 255]) // White (All colors)
    }

    pub const fn orange() -> Self {
        Self([255, 165, 0]) // Orange (Red + some Green)
    }

    pub const fn purple() -> Self {
        Self([128, 0, 128]) // Purple (Dimmed Magenta)
    }

    pub const fn pink() -> Self {
        Self([255, 192, 203]) // Pink (Light Red)
    }

    pub const fn brown() -> Self {
        Self([165, 42, 42]) // Brown
    }
}

#[cfg(feature = "own_delay")]
impl<'a, LED: OutputPin, D: DelayNs> GlowColor for WS2812<'a, LED, D> {
    fn t0h() -> u32 {
        350 // Time in nanoseconds (logic 0 high)
    }
    fn t1h() -> u32 {
        700 // Time in nanoseconds (logic 1 high)
    }
    fn t0l() -> u32 {
        800 // Time in nanoseconds (logic 0 low)
    }
    fn t1l() -> u32 {
        600 // Time in nanoseconds (logic 1 low)
    }
    fn reset() -> u32 {
        50
    }
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
    fn led_low(&mut self) {
        self.led.set_low().ok();
    }
    fn led_high(&mut self) {
        self.led.set_high().ok();
    }
}

#[cfg(feature = "spinloop_delay")]
impl<LED: OutputPin> GlowColor for WS2812<LED> {
    fn t0h() -> u32 {
        350 // Time in nanoseconds (logic 0 high)
    }
    fn t1h() -> u32 {
        700 // Time in nanoseconds (logic 1 high)
    }
    fn t0l() -> u32 {
        800 // Time in nanoseconds (logic 0 low)
    }
    fn t1l() -> u32 {
        600 // Time in nanoseconds (logic 1 low)
    }
    fn reset() -> u32 {
        50
    }
    fn led_low(&mut self) {
        self.led.set_low().ok();
    }
    fn delay_ns(&mut self, ns: u32) {
        let cycle = (self.cpu_freq * ns as u64) / 1_000_000_000;
        for _ in 0..cycle as u32 {
            core::hint::spin_loop();
        }
    }
    fn led_high(&mut self) {
        self.led.set_high().ok();
    }
}
#[cfg(feature = "manual_delay")]
impl<LED: OutputPin> GlowColor for WS2812<LED> {
    fn t0h() -> u32 {
        350 // Time in nanoseconds (logic 0 high)
    }
    fn t1h() -> u32 {
        700 // Time in nanoseconds (logic 1 high)
    }
    fn t0l() -> u32 {
        800 // Time in nanoseconds (logic 0 low)
    }
    fn t1l() -> u32 {
        600 // Time in nanoseconds (logic 1 low)
    }
    fn reset() -> u32 {
        50
    }
    fn led_low(&mut self) {
        self.led.set_low().ok();
    }
    fn led_high(&mut self) {
        self.led.set_high().ok();
    }
}

#![no_std]

use embedded_hal::digital::OutputPin;

use embedded_hal::spi::SpiBus;

#[cfg(feature = "async")]
use embedded_hal_async::digital::Wait;

#[cfg(not(feature = "spinloop_delay"))]
use embedded_hal::delay::DelayNs;

#[cfg(feature = "own_delay")]
pub struct WS2812<'a, LED, D> {
    led: LED,
    delay: &'a mut D,
}

#[cfg(feature = "manual")]
pub struct WS2812<LED> {
    led: LED,
}

#[cfg(feature = "spinloop_delay")]
pub struct WS2812<LED> {
    led: LED,
    cpu_freq: u64,
}

#[cfg(feature = "own_delay")]
impl<'a, LED, D: DelayNs> WS2812<'a, LED, D> {
    pub fn new(led: LED, delay: &'a mut D) -> Self {
        Self { led, delay }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}

#[cfg(feature = "manual")]
impl<LED> WS2812<LED> {
    pub fn new(led: LED) -> Self {
        Self { led }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}

#[cfg(feature = "spinloop_delay")]
impl<LED> WS2812<LED> {
    pub fn new(led: LED, cpu_freq: u64) -> Self {
        Self { led, cpu_freq }
    }
    pub fn destroy(self) -> LED {
        self.led
    }
}

pub trait TransferTiming {
    fn t0h() -> u32;
    fn t1h() -> u32;
    fn t0l() -> u32;
    fn t1l() -> u32;
    fn reset() -> u32;
}

pub trait AsyncGlowColor: TransferTiming {
    #[cfg(not(feature = "manual"))]
    fn delay_ns(&mut self, ns: u32) -> impl core::future::Future<Output = ()>;

    fn wait_for_low(&mut self) -> impl core::future::Future<Output = ()>;
    fn wait_for_high(&mut self) -> impl core::future::Future<Output = ()>;

    #[cfg(not(feature = "manual"))]
    fn async_send_color<const N: usize>(
        &mut self,
        color: [Color; N],
    ) -> impl core::future::Future<Output = ()> {
        async move {
            for each_color in color {
                for byte in each_color.0 {
                    for bit in (0..8).rev() {
                        if (byte & (1 << bit)) != 0 {
                            // Logic 1
                            self.wait_for_high().await;
                            self.delay_ns(Self::t1h()).await;
                            self.wait_for_low().await;
                            self.delay_ns(Self::t1l()).await;
                        } else {
                            // Logic 0
                            self.wait_for_high().await;
                            self.delay_ns(Self::t0h()).await;
                            self.wait_for_low().await;
                            self.delay_ns(Self::t0l()).await;
                        }
                    }
                }
            }
            self.wait_for_low().await;

            self.delay_ns(Self::reset()).await
        }
    }

    #[cfg(feature = "manual")]
    fn async_send_color<const N: usize, D: DelayNs>(
        &mut self,
        color: [Color; N],
        delay: &mut D,
    ) -> impl core::future::Future<Output = ()> {
        async move {
            for each_color in color {
                for byte in each_color.0 {
                    for bit in (0..8).rev() {
                        if (byte & (1 << bit)) != 0 {
                            // Logic 1
                            self.wait_for_high().await;
                            delay.delay_ns(Self::t1h());
                            self.wait_for_low().await;
                            delay.delay_ns(Self::t1l());
                        } else {
                            // Logic 0
                            self.wait_for_high().await;
                            delay.delay_ns(Self::t0h());
                            self.wait_for_low().await;
                            delay.delay_ns(Self::t0l());
                        }
                    }
                }
            }
            self.wait_for_low().await;
            delay.delay_ns(Self::reset())
        }
    }
}

pub trait GlowColor: TransferTiming {
    #[cfg(not(feature = "manual"))]
    fn delay_ns(&mut self, ns: u32);

    fn led_low(&mut self);
    fn led_high(&mut self);

    #[cfg(not(feature = "manual"))]
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

    #[cfg(feature = "manual")]
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

    #[cfg(feature = "async")]
    #[cfg(not(feature = "manual"))]
    fn send_color_w_embassy<const N: usize>(
        &mut self,
        color: [Color; N],
    ) -> impl Future<Output = ()> {
        async move {
            for each_color in color {
                for byte in each_color.0 {
                    for bit in (0..8).rev() {
                        if (byte & (1 << bit)) != 0 {
                            // Logic 1
                            self.led_high();
                            embassy_time::Timer::after_nanos(Self::t1h() as u64).await;
                            self.led_low();
                            embassy_time::Timer::after_nanos(Self::t1l() as u64).await;
                        } else {
                            // Logic 0
                            self.led_high();
                            embassy_time::Timer::after_nanos(Self::t0h() as u64).await;
                            self.led_low();
                            embassy_time::Timer::after_nanos(Self::t0l() as u64).await;
                        }
                    }
                }
                self.led_low();
                embassy_time::Timer::after_nanos(Self::reset() as u64).await;
            }
        }
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
impl<'a, LED, D> TransferTiming for WS2812<'a, LED, D> {
    fn t0h() -> u32 {
        400
    }
    fn t1h() -> u32 {
        800
    }
    fn t0l() -> u32 {
        850
    }
    fn t1l() -> u32 {
        450
    }
    fn reset() -> u32 {
        50_000
    }
}

#[cfg(feature = "own_delay")]
impl<'a, LED: OutputPin, D: DelayNs> GlowColor for WS2812<'a, LED, D> {
    fn led_low(&mut self) {
        self.led.set_low().ok();
    }
    fn delay_ns(&mut self, ns: u32) {
        self.delay.delay_ns(ns);
    }
    fn led_high(&mut self) {
        self.led.set_high().ok();
    }
}

#[cfg(feature = "async")]
#[cfg(feature = "own_delay")]
impl<'a, LED: Wait, D> AsyncGlowColor for WS2812<'a, LED, D> {
    async fn wait_for_high(&mut self) {
        self.led.wait_for_high().await.ok();
    }
    async fn delay_ns(&mut self, ns: u32) {
        embassy_time::Timer::after_nanos(ns as u64).await
    }
    async fn wait_for_low(&mut self) {
        self.led.wait_for_low().await.ok();
    }
}

#[cfg(feature = "spinloop_delay")]
impl<LED> TransferTiming for WS2812<LED> {
    fn t0h() -> u32 {
        400
    }
    fn t1h() -> u32 {
        800
    }
    fn t0l() -> u32 {
        850
    }
    fn t1l() -> u32 {
        450
    }
    fn reset() -> u32 {
        50_000
    }
}

#[cfg(feature = "spinloop_delay")]
impl<LED: OutputPin> GlowColor for WS2812<LED> {
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

#[cfg(feature = "async")]
#[cfg(feature = "spinloop_delay")]
impl<LED: Wait> AsyncGlowColor for WS2812<LED> {
    async fn wait_for_high(&mut self) {
        self.led.wait_for_high().await.ok();
    }
    async fn delay_ns(&mut self, ns: u32) {
        embassy_time::Timer::after_nanos(ns as u64).await
    }
    async fn wait_for_low(&mut self) {
        self.led.wait_for_low().await.ok();
    }
}

#[cfg(feature = "manual")]
impl<'a, LED> TransferTiming for WS2812<LED> {
    fn t0h() -> u32 {
        400
    }
    fn t1h() -> u32 {
        800
    }
    fn t0l() -> u32 {
        850
    }
    fn t1l() -> u32 {
        450
    }
    fn reset() -> u32 {
        50_000
    }
}

#[cfg(feature = "manual")]
impl<LED: OutputPin> GlowColor for WS2812<LED> {
    fn led_low(&mut self) {
        self.led.set_low().ok();
    }
    fn led_high(&mut self) {
        self.led.set_high().ok();
    }
}

#[cfg(feature = "async")]
#[cfg(feature = "manual")]
impl<LED: Wait> AsyncGlowColor for WS2812<LED> {
    async fn wait_for_high(&mut self) {
        self.led.wait_for_high().await.ok();
    }
    async fn wait_for_low(&mut self) {
        self.led.wait_for_low().await.ok();
    }
}

pub struct WS2812SPI<SPI: SpiBus> {
    led: SPI,
}

impl<SPI: SpiBus> WS2812SPI<SPI> {
    pub fn new(led: SPI) -> Self {
        Self { led }
    }
}

impl<SPI: SpiBus> TransferTiming for WS2812SPI<SPI> {
    fn t0h() -> u32 {
        400
    }
    fn t1h() -> u32 {
        800
    }
    fn t0l() -> u32 {
        850
    }
    fn t1l() -> u32 {
        450
    }
    fn reset() -> u32 {
        50_000
    }
}

pub trait SendColorBySPI: TransferTiming {
    fn write<E: embedded_hal::spi::ErrorType>(&mut self, data: &[u8]) -> Result<(), E>;
    fn color_send_by_spi<E: embedded_hal::spi::ErrorType>(
        &mut self,
        color: Color,
        freq: u32,
    ) -> Result<(), E> {
        if freq < 2_500_000 {
            return Ok(());
        }

        let ns_per_bit = 1_000_000_000 / freq;

        let t0h_bits = (Self::t0h() + (ns_per_bit / 2)) / ns_per_bit;
        let t0l_bits = (Self::t0l() + (ns_per_bit / 2)) / ns_per_bit;

        let t1h_bits = (Self::t1h() + (ns_per_bit / 2)) / ns_per_bit;
        let t1l_bits = (Self::t1l() + (ns_per_bit / 2)) / ns_per_bit;

        let mut spi_payload = [0u8; 24];
        let mut bit_pos = 0;

        let grb = [color.0[1], color.0[0], color.0[2]];

        for color_byte in grb {
            for bit_pair in (0..8).rev() {
                let bit = (color_byte >> bit_pair) & 1;

                let (high_bits, low_bits) = if bit == 1 {
                    (t1h_bits, t1l_bits)
                } else {
                    (t0h_bits, t0l_bits)
                };

                for _ in 0..high_bits {
                    let byte_idx = bit_pos / 8;
                    let bit_idx = 7 - (bit_pos % 8);
                    spi_payload[byte_idx as usize] |= 1 << bit_idx;
                    bit_pos += 1;
                }

                bit_pos += low_bits;
            }
        }

        let bytes_used = ((bit_pos + 7) / 8) as usize;

        self.write(&spi_payload[0..bytes_used])
    }

    fn write_colors<const N: usize, E: embedded_hal::spi::ErrorType>(
        &mut self,
        colors: [Color; N],
        freq: u32,
    ) -> Result<(), E> {
        for color in colors {
            self.color_send_by_spi(color, freq)?;
        }

        let reset_ns = Self::reset();
        let ns_per_bit = 1_000_000_000 / freq;
        let reset_bits = reset_ns / ns_per_bit;
        let reset_bytes = ((reset_bits + 7) / 8) as usize;

        let reset_payload = [0u8; 64];
        let clamped_bytes = reset_bytes.min(64);

        self.write(&reset_payload[0..clamped_bytes])?;

        Ok(())
    }
}

impl<SPI: SpiBus> SendColorBySPI for WS2812SPI<SPI> {
    fn write<E>(&mut self, data: &[u8]) -> Result<(), E>
    where
        E: embedded_hal::spi::ErrorType,
    {
        self.led.write(data).ok();
        Ok(())
    }
}

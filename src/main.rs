//! SPI loopback test using DMA
//!
//! Folowing pins are used:
//! SCLK    GPIO6
//! MISO    GPIO2
//! MOSI    GPIO7
//! CS      GPIO10
//!
//! Depending on your target and the board you are using you have to change the
//! pins.
//!
//! This example transfers data via SPI.
//! Connect MISO and MOSI pins to see the outgoing data is read as incoming
//! data.

#![no_std]
#![no_main]

use sdmmc_spi;
use esp32c6_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    spi::{Spi, SpiMode},
    timer::TimerGroup,
    Delay,
    Rtc,
};
use esp32c6_hal::gpio::{GpioPin, Output};
use esp32c6_hal::spi::FullDuplexMode;
use esp_backtrace as _;
use esp_println::println;
use sdmmc_spi::{DefaultSdMmcSpiConfig, DiskioDevice, SdMmcSpi};
use switch_hal::{ActiveLow, Switch};

use defmt::global_logger;

#[global_logger]
struct Logger;

unsafe impl defmt::Logger for Logger{
    fn acquire() {
        todo!()
    }

    unsafe fn flush() {
        todo!()
    }

    unsafe fn release() {
        todo!()
    }

    unsafe fn write(bytes: &[u8]) {
        todo!()
    }
}
use defmt as _;

defmt::timestamp!("{=u32:us}", {
    // NOTE(interrupt-safe) single instruction volatile read operation
    unsafe { Peripherals::steal().SYSTIMER.unit0_value_lo.read().timer_unit0_value_lo().bits() }
});

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.PCR.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the watchdog timers. For the ESP32-C6, this includes the Super WDT,
    // and the TIMG WDTs.
    let mut rtc = Rtc::new(peripherals.LP_CLKRST);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;

    // Disable watchdog timers
    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let miso = io.pins.gpio20;
    let mosi = io.pins.gpio18;
    let sclk = io.pins.gpio19;
    let cs = io.pins.gpio23;

    let spi = Spi::new_no_cs(
        peripherals.SPI2,
        sclk,
        mosi,
        miso,
        250u32.kHz(),
        SpiMode::Mode0,
        &mut system.peripheral_clock_control,
        &clocks,
    );
    let switch: Switch<GpioPin<Output<esp32c6_hal::gpio::PushPull>, 23>, ActiveLow> = Switch::new(cs.into_push_pull_output());
    let mut sd = SdMmcSpi::<Spi<'_, esp32c6_hal::peripherals::SPI2, FullDuplexMode>, Switch<GpioPin<Output<esp32c6_hal::gpio::PushPull>, 23>, ActiveLow>, DefaultSdMmcSpiConfig>::new(spi, switch);

    let delay = Delay::new(&clocks);

    sd.initialize().unwrap();

    println!("{:?}", sd.status());

    let mut buf = [0u8; 512];
    sd.read(&mut buf, 0).unwrap();

    println!("Read: {:?}", buf);

    loop {
        delay.delay(1000);
    }
}

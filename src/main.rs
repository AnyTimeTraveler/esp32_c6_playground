//! Blinks an LED
//!
//! This assumes that a LED is connected to the pin assigned to `led`. (GPIO5)

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use esp32c6_hal::{
    clock::ClockControl,
    gpio::IO,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
    Delay,
    Rtc,
};
use esp32c6_hal::mcpwm::{MCPWM, PeripheralClockConfig};
use esp32c6_hal::mcpwm::operator::PwmPinConfig;
use esp32c6_hal::mcpwm::timer::PwmWorkingMode;
use esp_backtrace as _;
use esp_println::println;

#[global_allocator]
static GLOBAL: EspHeap = EspHeap::empty();


#[entry]
fn main() -> ! {
    unsafe { GLOBAL.init(0x4080_0000 as *mut u8, 0x80000) }
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

    rtc.swd.disable();
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // initialize peripheral
    // let clock_cfg = PeripheralClockConfig::with_frequency(&clocks, 80u32.MHz()).unwrap();
    // let mut mcpwm = MCPWM::new(
    //     peripherals.MCPWM0,
    //     clock_cfg,
    //     &mut system.peripheral_clock_control,
    // );
    //
    // let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    //
    // // connect operator0 to timer0
    // mcpwm.operator0.set_timer(&mcpwm.timer0);
    // // connect operator0 to pin
    // let mut pwm_pin = mcpwm
    //     .operator0
    //     .with_pin_a(io.pins.gpio8, PwmPinConfig::UP_ACTIVE_HIGH);
    //
    // // start timer with timestamp values in the range of 0..=99 and a frequency of 20 kHz
    // let timer_clock_cfg = clock_cfg
    //     .timer_clock_with_frequency(64, PwmWorkingMode::Increase, 800u32.kHz())
    //     .unwrap();
    // mcpwm.timer0.start(timer_clock_cfg);

    // pin will be high 50% of the time
    // pwm_pin.set_timestamp(64);


    // Set GPIO5 as an output, and set its state high initially.
    // let mut led = io.pins.gpio8.into_push_pull_output();


    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    // let mut delay = Delay::new(&clocks);

    edwd();

    loop {
        // led.set_high().unwrap();
        // delay.delay_us()

        // led.toggle().unwrap();
        // delay.delay_ms(5u32);
        // println!("Hello World!");
    }
}

#[no_mangle]
pub extern "C" fn fmodf(a: f32, b: f32) -> f32 {
    libm::fmodf(a, b)
}

#[no_mangle]
pub extern "C" fn fmod(a: f64, b: f64) -> f64 {
    libm::fmod(a, b)
}

#[no_mangle]
pub extern "C" fn fminf(a: f32, b: f32) -> f32 {
    libm::fminf(a, b)
}

#[no_mangle]
pub extern "C" fn fmin(a: f64, b: f64) -> f64 {
    libm::fmin(a, b)
}

#[no_mangle]
pub extern "C" fn fmaxf(a: f32, b: f32) -> f32 {
    libm::fmaxf(a, b)
}

#[no_mangle]
pub extern "C" fn fmax(a: f64, b: f64) -> f64 {
    libm::fmax(a, b)
}


use esp_alloc::EspHeap;
use wasmi::{Caller, Engine, Func, Linker, Module, Store};

fn edwd() {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    // let wat = r#"
    //     (module
    //         (import "host" "hello" (func $host_hello (param i32)))
    //         (func (export "hello")
    //             (call $host_hello (i32.const 3))
    //         )
    //     )
    // "#;
    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    // let wasm = wat::parse_str(&wat)?;
    let wasm = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x60, 0x01, 0x7F, 0x00, 0x60, 0x00, 0x00, 0x02, 0x0E, 0x01, 0x04, 0x68, 0x6F, 0x73, 0x74, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x00, 0x03, 0x02, 0x01, 0x01, 0x07, 0x09, 0x01, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x01, 0x0A, 0x08, 0x01, 0x06, 0x00, 0x41, 0x03, 0x10, 0x00, 0x0B, 0x00, 0x14, 0x04, 0x6E, 0x61, 0x6D, 0x65, 0x01, 0x0D, 0x01, 0x00, 0x0A, 0x68, 0x6F, 0x73, 0x74, 0x5F, 0x68, 0x65, 0x6C, 0x6C, 0x6F];

    // let wasm = Vec::new();
    let module = Module::new(&engine, &wasm[..]).unwrap();

    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    let host_hello = Func::wrap(&mut store, |caller: Caller<'_, HostState>, param: i32| {
        println!("Got {param} from WebAssembly");
        println!("My host state is: {}", caller.data());
    });

    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let mut linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    linker.define("host", "hello", host_hello).unwrap();
    let instance = linker
        .instantiate(&mut store, &module).unwrap()
        .start(&mut store).unwrap();
    let hello = instance.get_typed_func::<(), ()>(&store, "hello").unwrap();

    // And finally we can call the wasm!
    hello.call(&mut store, ()).unwrap();
}
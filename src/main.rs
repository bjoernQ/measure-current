#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, i2c::I2C, peripherals::Peripherals, prelude::*, Delay, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();
    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100u32.kHz(),
        &clocks,
    );

    let mut ina219 = ti_ina219::AdafruitIna219::init(i2c, ti_ina219::DeviceAddress::new()).unwrap();

    loop {
        // let v = ina219.get_bus_voltage_v().unwrap();
        // println!("bus voltage {v}");

        // let sv = ina219.get_shunt_voltage_v().unwrap();
        // println!("shunt voltage {sv}");

        // let p = ina219.get_power_mw().unwrap();
        // println!("power {p}");

        let c = ina219.get_current_ma().unwrap();
        println!(">current:{c}");
        delay.delay_ms(50u32);
    }
}

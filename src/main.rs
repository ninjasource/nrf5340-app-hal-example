#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use hal::{
    clocks::Clocks,
    gpio::{Pin, PinMode, Port},
    low_power,
    timer::{Timer, TimerInterrupt},
};
// global logger
//use nrf5340_app_hal as hal;
use panic_halt as _;
use stm32_hal2 as hal;

#[rtic::app(device = crate::hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Pin,
    }

    #[init()]
    fn init(ctx: init::Context) -> init::LateResources {
        info!("init");

        //let mut p = ctx.device;

        // the app core is also called the "non-secure" core, hence "NS"
        //let p0 = hal::gpio::p0::Parts::new(p.P0_NS);
        //let p0 = hPin

        let clock_cfg = Clocks::default();
        clock_cfg.setup().unwrap();

        let dp = ctx.device;
        let mut timer = Timer::new_tim1(dp.TIM1, 0.2, Default::default(), &clock_cfg);
        timer.enable_interrupt(TimerInterrupt::Update);

        let led = Pin::new(Port::C, 7, PinMode::Output);
        //let led = p0.p0_28.into_push_pull_output(Level::High).degrade();
        /*
        // setup spi master mode
        let _cs_grey = p0.p0_18.into_push_pull_output(Level::High).degrade();
        let sck_purple = p0.p0_17.into_push_pull_output(Level::Low).degrade();
        let mosi_blue = p0.p0_14.into_push_pull_output(Level::Low).degrade();
        let miso_green = p0.p0_15.into_floating_input().degrade();
        let pins = hal::spim::Pins {
            sck: sck_purple,
            mosi: Some(mosi_blue),
            miso: Some(miso_green),
        };
        let _spi = Spim::new(p.SPIM0_NS, pins, Frequency::M2, MODE_0, 0); // this panics!!
        */

        init::LateResources { led }
    }

    #[idle(resources=[led])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::Resources { led } = ctx.resources;

        loop {
            led.set_high();
            cortex_m::asm::delay(20_000_000);
            led.set_low();
            cortex_m::asm::delay(20_000_000);
        }
    }
};

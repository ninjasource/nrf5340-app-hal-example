#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _; // global logger
use hal::{
    gpio::{Level, Output, Pin, PushPull},
    pac::Peripherals,
    prelude::OutputPin,
    spim::{Frequency, MODE_0},
    Spim,
};
use nrf5340_app_hal as hal;
use panic_halt as _;

// This achieves the same result as the bootloader suggested in the nrf5340_app_hal docs
// The code below comes from https://github.com/Dirbaio/nrf53-test
fn unlock_nrf5340_app_core(p: &mut Peripherals) {
    p.CACHE_S.enable.write(|w| w.enable().enabled());
    p.CLOCK_S.hfclkctrl.write(|w| w.hclk().div1());

    if !p.UICR_S.approtect.read().pall().is_unprotected() {
        info!("Setting UICR.APPROTECT=Unprotected");
        p.NVMC_S.config.write(|w| w.wen().wen());
        while p.NVMC_S.ready.read().bits() == 0 {}
        p.UICR_S.approtect.write(|w| w.pall().unprotected());
        while p.NVMC_S.ready.read().bits() == 0 {}
        p.NVMC_S.config.write(|w| w.wen().ren());
    }

    if !p.UICR_S.secureapprotect.read().pall().is_unprotected() {
        info!("Setting UICR.SECUREAPPROTECT=Unprotected");
        p.NVMC_S.config.write(|w| w.wen().wen());
        while p.NVMC_S.ready.read().bits() == 0 {}
        p.UICR_S.secureapprotect.write(|w| w.pall().unprotected());
        while p.NVMC_S.ready.read().bits() == 0 {}
        p.NVMC_S.config.write(|w| w.wen().ren());
    }

    p.CTRLAP_S
        .approtect
        .disable
        .write(|w| unsafe { w.bits(0x50FA50FA) });
    p.CTRLAP_S
        .secureapprotect
        .disable
        .write(|w| unsafe { w.bits(0x50FA50FA) });

    p.SPU_S.periphid[66]
        .perm
        .write(|w| w.secattr().non_secure());
    p.SPU_S.gpioport[0].perm.write(|w| unsafe { w.bits(0) });

    p.P0_S.pin_cnf[29].write(|w| w.mcusel().network_mcu());

    // Boot network core
    p.RESET_S.network.forceoff.write(|w| w.forceoff().release());
}

#[rtic::app(device = crate::hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Pin<Output<PushPull>>,
    }

    #[init()]
    fn init(ctx: init::Context) -> init::LateResources {
        info!("init app core");

        let mut p = ctx.device;
        unlock_nrf5340_app_core(&mut p);

        // the app core is also called the "non-secure" core, hence "NS"
        let p0 = hal::gpio::p0::Parts::new(p.P0_NS);
        let led = p0.p0_28.into_push_pull_output(Level::High).degrade();

        /*
        // setup spi master mode - this panics :(
        let _cs_grey = p0.p0_18.into_push_pull_output(Level::High).degrade();
        let sck_purple = p0.p0_17.into_push_pull_output(Level::Low).degrade();
        let mosi_blue = p0.p0_14.into_push_pull_output(Level::Low).degrade();
        let miso_green = p0.p0_15.into_floating_input().degrade();
        let pins = hal::spim::Pins {
            sck: sck_purple,
            mosi: Some(mosi_blue),
            miso: Some(miso_green),
        };
        let _spi = Spim::new(p.SPIM0_NS, pins, Frequency::M2, MODE_0, 0);
        */

        init::LateResources { led }
    }

    #[idle(resources=[led])]
    fn idle(ctx: idle::Context) -> ! {
        let idle::Resources { led } = ctx.resources;

        loop {
            led.set_high().unwrap();
            cortex_m::asm::delay(20_000_000);
            led.set_low().unwrap();
            cortex_m::asm::delay(20_000_000);
        }
    }
};

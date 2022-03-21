# nrf5340-dk Example

## Introduction

The purpose of this example is to clear up some of the mystery surrounding the nrf5340 mcu and to provide stable example code for better understanding.

This demo is a skeleton application which blinks an LED on the dual core nRF5340-DK. It demonstrates basic logging and panic handling (with line numbers) using the probe-run flashing tool.
The app assumes that there is no bootloader. See Setup section for fist time runs.

The nrf5340 has 2 mcu cores. An a cortex-m4 core called the net core and a cortex-m33 core (faster) called the app core. The net core always runs in s mode (secure-mode) and the app core can run in either s or ns (non-secure) mode. If the app core runs in s mode then it can be treated like a normal cortex-m device but it it runs in ns-mode it is not allowed to access memory and other s-mode peripherals using a system called trustzone. Therefore if you don't need trustzone then just run all your code in s mode. It is my understanding that you only need ns mode if you want to run untrusted code. 

Both cores run when the mcu powers up. However, the app core loads software from a different flash region then the net core so each core has its own `memory.x` file as follows:

App core with no boot loader running in s mode (by default):
```
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}
```

Net core (always s mode):
```
MEMORY
{
  FLASH : ORIGIN = 0x01000000, LENGTH = 256K
  RAM : ORIGIN = 0x21000000, LENGTH = 64K
}
```

App core running in ns mode using nordic's example SPM bootloader. It will boot your code from 0x50000:
```
MEMORY
{
    SECURE_FLASH : ORIGIN = 0x00000000, LENGTH = 256K
    FLASH        : ORIGIN = 0x00050000, LENGTH = 767K 
    SECURE_RAM   : ORIGIN = 0x20000000, LENGTH = 64K
    RAM          : ORIGIN = 0x20020000, LENGTH = 128K
}
```
However, at the time of writing I found that this ns-mode app core did not work nicely with probe-run and logging.

## Why the forked nrf-hal? 

At the time of writing nrf-hal only supports an ns mode app core for some unknown reason. The fork simply changes the peripherals to their `s` variant.


## Running

To run this demo:
```
cargo run
```

## Setup

Add the `thumbv8m.main-none-eabihf` target to your Rust toolchain
```console
$ rustup target add thumbv8m.main-none-eabihf
```

Install probe-run 0.3
```console
$ cargo install probe-run
```

If probe-run fails (e.g. `Error: A core architecture specific error occurred`), run this to prepare the nrf5340 to be flashed:
https://github.com/diondokter/nRF53-recovery
It needs to be run every time the nRF5340 is power cycled it seems. At least until probe-run is fixed at some point in the future.

## Additional Copyright

The code used to enable non-secure mode in the app core comes from the following Repo:
https://github.com/Dirbaio/nrf53-test

## License 

Licensed under either MIT or Apache-2.0 at your option
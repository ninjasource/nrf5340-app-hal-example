# nrf5340-dk Example

## Introduction

This demo is a skeleton application which blinks an LED on the dual core nRF5340-DK. It demonstrates basic logging and panic handling (with line numbers) using the probe-run flashing tool.
The app assumes that there is no bootloader and that the network and app cores are set up to run an insecure application. See Setup section for fist time runs.

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

If probe-run fails (e.g. `Error: A core architecture specific error occured`), run this to prepare the nrf5340 to be flashed:
https://github.com/diondokter/nRF53-recovery
It needs to be run every time the nRF5340 is power cycled it seems. At least until probe-run is fixed at some point in the future.

## Additional Copyright

The code used to unlock the app core comes from the following Repo:
https://github.com/Dirbaio/nrf53-test

## License 

Licensed under either MIT or Apache-2.0 at your option
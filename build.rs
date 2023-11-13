#![deny(warnings)]

use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let target = env::var("TARGET")?;

    // Manually list targets that have atomic load/store, but no CAS.
    // Remove when `cfg(target_has_atomic_load_store)` is stable.
    // last updated nightly-2023-10-28
    match &target[..] {
        "armv4t-none-eabi"
        | "armv5te-none-eabi"
        | "avr-unknown-gnu-atmega328"
        | "bpfeb-unknown-none"
        | "bpfel-unknown-none"
        | "thumbv4t-none-eabi"
        | "thumbv5te-none-eabi"
        | "thumbv6m-none-eabi" => println!("cargo:rustc-cfg=has_atomic_load_store"),
        _ => {}
    };

    Ok(())
}

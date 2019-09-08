#![no_std]
#![allow(non_camel_case_types)]

pub use stm32f0;

pub use stm32f0::stm32f0x1 as stm32;

#[cfg(feature = "device-selected")]
pub mod adc;
#[cfg(feature = "device-selected")]
pub mod delay;
#[cfg(feature = "device-selected")]
pub mod gpio;
#[cfg(feature = "device-selected")]
pub mod i2c;
#[cfg(feature = "device-selected")]
pub mod prelude;
#[cfg(feature = "device-selected")]
pub mod rcc;
#[cfg(feature = "device-selected")]
pub mod serial;
#[cfg(feature = "device-selected")]
pub mod spi;
#[cfg(feature = "device-selected")]
pub mod time;
#[cfg(feature = "device-selected")]
pub mod timers;
#[cfg(any(
    feature = "stm32f031",
    feature = "stm32f051",
    feature = "stm32f071",
    feature = "stm32f091",
    feature = "stm32f042",
    feature = "stm32f072",
    feature = "stm32f038",
    feature = "stm32f048",
    feature = "stm32f058",
    feature = "stm32f078",
    feature = "stm32f098",
))]
pub mod tsc;
#[cfg(feature = "device-selected")]
pub mod watchdog;

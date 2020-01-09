#[macro_use]
extern crate serde_derive;
extern crate pretty_env_logger;
// #[macro_use]
// extern crate log;

pub mod fastrs_config;

// Assume that crate is called adder, will have to extern it in integration test.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

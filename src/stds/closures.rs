use std::{thread, time::Duration};

#[allow(unused_imports)]
use crate::Runable;

use super::Test;

impl Test {
    fn simulated_expensive_calc(intensity: u32) -> u32 {
        println!("calculation slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    }

    pub fn closuers() {
        Self::simulated_expensive_calc(10);
    }
}

impl Runable for Test {
    fn run() {
        Self::closuers();
    }
}

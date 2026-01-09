#[allow(unused_imports)]
use crate::Runable;

use super::Test;

impl Test {
    fn variables() {
        let mut x = 5;
        println!("the value of x is: {x}");

        x = 6;
        println!("the value of x is: {x}");

        // shadowing (same name but changed mut or type or both)
        let x = "six";
        println!("the value of x is: {x}");

        // const cant be mut and must define the type
        const SUB_NUMBER_COUNT: u32 = 100_000_000;
        println!("the value of const subs: {SUB_NUMBER_COUNT}");
    }

    #[allow(unused_variables)]
    fn data_types() {
        // Integers
        let a: i32 = 98_222; // Decimal
        let b = 0xff; // Hex
        let c = 0o77; // Octal
        let d = 0b1111_0000; // Binary
        let e: u8 = b'A'; // Byte (u8 only)

        let f_over: u8 = 255; // 256 is overflow for u8

        // Floating points
        let f: f64 = 2.0;
        let g: f32 = 3.0;

        // addition
        let sum = a + b;
        // subtraction
        let difference = a - b;
        // multiplication
        let product = c - d;
        // divison
        let quotient = f_over / e;
        // reminder
        let remainder = g % f as f32;

        // bool
        let t = true;
        let f = false;

        // character
        let c: char = 'z';
    }

    #[allow(unused_variables)]
    fn compound_types() {
        // tuple
        let tup = ("lets get rusty", 1000);

        let (text, number) = tup;
        let number = tup.1;

        let error_codes = [500, 404, 401];
        let not_found = error_codes[1];

        // out of pounce
        // let x = error_codes = [3];

        // create array with 0 and length 8

        let arr = [0; 8];
    }

    pub fn common_concepts() {
        Self::variables();
        Self::data_types();
        Self::compound_types();
    }
}

impl Runable for Test {
    fn run() {
        Self::common_concepts();
    }
}

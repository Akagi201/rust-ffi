use core::ffi::c_double;
use core::ffi::c_int; // 32 bits // 64 bits

// Import three functions from the standard library libc.
// Here are the Rust declarations for the C functions:
extern "C" {
    fn abs(num: c_int) -> c_int;
    fn sqrt(num: c_double) -> c_double;
    fn pow(num: c_double, power: c_double) -> c_double;
}

fn main() {
    let x: i32 = -123;
    println!("Absolute value of {x}: {}.", unsafe { abs(x) });

    let n: f64 = 9.0;
    let p: f64 = 3.0;
    println!("{n} raised to {p}: {}.", unsafe { pow(n, p) });

    let mut y: f64 = 64.0;
    println!("Square root of {y}: {}.", unsafe { sqrt(y) });
    y = -3.14;
    println!("Square root of {y}: {}.", unsafe { sqrt(y) });
}

use std::collections::BTreeSet;
use std::num::NonZero;

fn i32_to_nonzero_u32(value: i32) -> Option<NonZero<u32>> {
    NonZero::new(value as u32).filter(|_| value > 0)
}

fn divisors(n: NonZero<u32>) -> BTreeSet<NonZero<u32>> {
    let arr: [NonZero<u32>; 2] = [1.into(), n];
    let mut set : BTreeSet<NonZero<u32>> = BTreeSet::from([n, 1]);
    let mut i :  = 1;
    set
}
fn main() {
    println!("Hello, world!");
}

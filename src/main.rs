#![allow(unused_assignments)]
#![allow(unused_variables)]

use timeit::timeit_loops;

const A: Option<u64> = Some(5);
const BOOL_EXPRESSION: bool = true;

fn main() {
    let sec = timeit_loops!(1_000_000_000, {
        let b = if BOOL_EXPRESSION {
            A
        } else {
            None
        };
    });

    println!("timeit_loops: {} seconds: {}", 1_000_000_000, sec);

    let sec = timeit_loops!(1_000_000_000, {
        let mut b: Option<u64> = None;
        if BOOL_EXPRESSION {
            b = A;
        }
    });

    println!("timeit_loops: {} seconds: {}", 1_000_000_000, sec);
}
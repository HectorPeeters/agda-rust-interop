// Needed for curried functions
#![feature(type_alias_impl_trait)]
// These are used to disable clippy warnings since the generated Rust code does not follow the rust
// styling guidelines
#![allow(non_camel_case_types)]
#![allow(unreachable_patterns)]
#![allow(non_snake_case)]

// Include the compiled Agda file
include!(concat!(env!("OUT_DIR"), "/Test.rs"));

fn to_native_number(mut x: Nat) -> u64 {
    let mut result = 0;

    loop {
        match x {
            Nat::zero() => break,
            Nat::suc(new_x) => {
                x = *new_x;
                result += 1;
            }
        };
    }

    result
}

fn main() {
    let number1 = two();
    let number2 = three();

    let sum = plus()(number1)(number2);

    println!("{:?}", to_native_number(sum));

    let negated_true = not()(ztrue());

    println!("{:?}", negated_true);
}

#![feature(type_alias_impl_trait)]

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
}

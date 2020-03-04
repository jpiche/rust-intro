
pub struct Number {
    odd: bool,
    value: i32,
}

pub fn print_signed(n: Number) {
    match n {
        Number { odd: true, value } => println!("Odd number: {}", value),
        Number { odd: false, value } => println!("Even number: {}", value),
    }
}

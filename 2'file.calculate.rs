use std::io;

fn mask(bits: u32) -> i32 {
    (1 << bits) - 1
}

fn to_twos_complement(value: i32, bits: u32) -> i32 {
    value & mask(bits)
}

fn from_twos_complement(value: i32, bits: u32) -> i32 {
    let sign_bit = 1 << (bits - 1);
    if value & sign_bit != 0 {
        value - (1 << bits)
    } else {
        value
    }
}

fn to_binary(value: i32, bits: u32) -> String {
    format!("{:0width$b}", value & mask(bits), width = bits as usize)
}

fn main() {
    let mut input = String::new();

    print!("Enter bit width: ");
    io::stdin().read_line(&mut input).unwrap();
    let bits: u32 = input.trim().parse().unwrap();
    input.clear();

    print!("Enter first number: ");
    io::stdin().read_line(&mut input).unwrap();
    let a: i32 = input.trim().parse().unwrap();
    input.clear();

    print!("Enter second number: ");
    io::stdin().read_line(&mut input).unwrap();
    let b: i32 = input.trim().parse().unwrap();

    let a_tc = to_twos_complement(a, bits);
    let b_tc = to_twos_complement(b, bits);

    println!("\nTwo's complement representations:");
    println!("A = {:>4} -> {}", a, to_binary(a_tc, bits));
    println!("B = {:>4} -> {}", b, to_binary(b_tc, bits));

    // Addition
    let sum_tc = to_twos_complement(a_tc + b_tc, bits);
    let sum = from_twos_complement(sum_tc, bits);

    // Subtraction
    let diff_tc = to_twos_complement(a_tc - b_tc, bits);
    let diff = from_twos_complement(diff_tc, bits);

    // Negation
    let neg_a_tc = to_twos_complement(!a_tc + 1, bits);
    let neg_a = from_twos_complement(neg_a_tc, bits);

    println!("\nResults:");
    println!("Add:  {:>4} -> {}", sum, to_binary(sum_tc, bits));
    println!("Sub:  {:>4} -> {}", diff, to_binary(diff_tc, bits));
    println!("Neg A:{:>4} -> {}", neg_a, to_binary(neg_a_tc, bits));
}

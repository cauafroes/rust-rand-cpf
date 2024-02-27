use clap::Parser;
use rand::Rng;

#[derive(Parser)]
struct Cli {
    //qual o último digito do cpf, se houver
    last_digit: Option<i8>
}

fn main() {
    let args = Cli::parse();

    println!("Your last digit: {:?}", args.last_digit);

    let vals = cpf_gen();
    println!("{:?}", vals);
}

fn cpf_gen() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<u8> = (0..9).map(|_| rng.gen_range(0..9)).collect();
    vals.push(get_digit(&vals, 2));
    vals.push(get_digit(&vals, 2));

    vals
}

fn get_digit(numbers: &Vec<u8>, digit: u8) -> u8 {
    let multipliers: Vec<u8> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];
    // numbers[0] + multiplier
}
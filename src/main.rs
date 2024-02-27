use clap::Parser;
// use rand::Rng;

#[derive(Parser)]
struct Cli {
    //qual o último digito do cpf, se houver
    last_digit: Option<i8>
}

fn main() {
    let last_digit = Cli::parse().last_digit.unwrap_or(0);

    let mut cpf = cpf_gen();

    while cpf[10] != last_digit as u16 {
        cpf = cpf_gen();
    }    

    println!("{:?}", cpf);
}

fn cpf_gen() -> Vec<u16> {
    let mut rng = rand::thread_rng();
    let mut vals: Vec<u16> = (0..9).map(|_| rng.gen_range(0..9)).collect();
    
    vals.push(get_digit(&vals, 1));
    vals.push(get_digit(&vals, 2));

    vals
}

fn get_digit(numbers: &Vec<u16>, digit: u16) -> u16 {
    let mut multipliers: Vec<u16> = vec![10, 9, 8, 7, 6, 5, 4, 3, 2];

    if digit == 2 {
        multipliers.insert(0, 11);
    }

    let result: Vec<u16> = numbers.iter().zip(multipliers.iter()).map(|(a, b)| a * b).collect();
    let sum: u16 = result.iter().sum();
    let rest: u16 = sum % 11;

    if rest >= 2 {
        return 11 - rest;
    }

    0
}
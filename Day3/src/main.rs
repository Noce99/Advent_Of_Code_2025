use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_maximum_bank_joltage_2_digits(bank: & mut Vec<u8>) -> u64{
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..(bank.len()-1) {
        if bank[i] > max {
            max = bank[i];
            max_index = i;
        }
    }
    let first_digit = max;
    let mut max = 0;
    for i in (max_index+1)..bank.len() {
        if bank[i] > max {
            max = bank[i];
        }
    }
    return (first_digit*10 + max) as u64
}

fn find_maximum_bank_joltage_n_digits(bank: & mut Vec<u8>, n: u16) -> u64{
    let mut digits: Vec<u8> = vec![];
    let mut start_index: usize = 0;
    
    for left_digits in (0..n).rev(){
        let mut max: u8 = 10;
        let mut max_index = 0;

        for i in (start_index)..(bank.len()-(left_digits as usize)) {
            if max == 10 || bank[i] > max {
                max = bank[i];
                max_index = i;
            }
        }
        start_index = max_index+1;
        digits.push(max);
    }

    let mut joltage: u64 = 0;
    for i in 0..n{
        joltage += 10_u64.pow((n-i-1) as u32) * (digits[i as usize] as u64);
    }

    return joltage
}

fn main(){
    const TASK : u8 = 2; // 1 or 2
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut total_joltage: u64 = 0;

    let mut batteries : Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        let content = line.unwrap();
        // println!("{}", content);
        let mut vector: Vec<u8> = vec![];
        for c in content.chars(){
            vector.push(c.to_digit(10).unwrap() as u8);
        }
        batteries.push(vector);
    }

    let start = std::time::Instant::now();
    for i in 0..batteries.len(){
        let mut joltage: u64 = 0;
        if TASK == 1{
            joltage = find_maximum_bank_joltage_2_digits(&mut batteries[i]);
        }else if TASK == 2{
            joltage = find_maximum_bank_joltage_n_digits(&mut batteries[i], 12);
        }else{
            println!("TASK must be 1 or 2!");
            break;
        }
        total_joltage += joltage;
    }
    let duration = start.elapsed();
    println!("Total Time = {} us", duration.as_micros());
    println!("Result = {}", total_joltage);
}

use std::fs::File;
use std::io::{BufRead, BufReader};

enum Operation{
    Sum,
    Product
}

fn str_to_operation(sign: &str) -> Operation{
    if sign == "+" {
        return Operation::Sum
    }else{
        return Operation::Product
    }
}

fn get_input(file_name: &str) -> (Vec<Vec<u64>>, Vec<Operation>){
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut all_numbers: Vec<Vec<u64>> = vec![];
    let operations: Vec<Operation>;

    let mut lines: Vec<_> = reader.lines().collect();
    let line_operations = lines.pop().unwrap().unwrap(); // We drop operations
    operations = line_operations
        .split_whitespace()
        .map(|o| str_to_operation(o))
        .collect();

    for line in lines {
        let content = line.unwrap();
        let one_raw_of_numbers: Vec<u64> = content
            .split_whitespace()
            .map(|w| w.parse().unwrap())
            .collect();
        all_numbers.push(one_raw_of_numbers);
    }

    (all_numbers, operations)
}


fn main() {
    let (all_numbers, operations) = get_input("input.txt");

    let mut total_sum: u64 = 0;

    for i in 0..operations.len() {
        let mut partial_op: u64;
        if matches!(operations[i], Operation::Product) {
            partial_op = 1;
        }else{
            partial_op = 0;
        }

        for k in 0..all_numbers.len(){
            if matches!(operations[i], Operation::Product) {
                partial_op *= all_numbers[k][i];
            }else{
                partial_op += all_numbers[k][i];
            }
        }
        total_sum += partial_op;
    }

    println!("Part 1: {}", total_sum);
}

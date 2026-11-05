use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_iter(i:usize, i_max:usize) -> impl Iterator<Item = usize> {
    let start: usize;
    let end: usize;
    if i == 0 {
        start = 0;
        end = i+2;
    }else{
        start = i-1;
        if i == i_max-1{
            end = i_max;
        }else{
            end = i+2;
        }
    }
    return start..end
}

fn count_neighboards(matrix : & mut Vec<Vec<u8>>) -> Vec<Vec<u8>>{
    let mut result: Vec<Vec<u8>> = vec![];

    for i in 0..matrix.len(){
        result.push(vec![0; matrix[i].len()]);
        for j in 0..matrix[i].len(){
            if matrix[i][j] == 1 {
                for k in get_iter(i, matrix.len()){
                    for l in get_iter(j, matrix[i].len()){
                        result[i][j] += matrix[k][l];
                    }
                }
                result[i][j] -= 1;
            }
        }
    }
    return result
}

fn remove_free(paper_rolls : & mut Vec<Vec<u8>>, neighboard_count: & Vec<Vec<u8>>) -> u32{
    let mut counter: u32 = 0;
    for i in 0..paper_rolls.len(){
        for j in 0..paper_rolls[i].len(){
            if neighboard_count[i][j] < 4 && paper_rolls[i][j] == 1{
                paper_rolls[i][j] = 0;
                counter += 1;
            }
        }
    }
    counter
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    const PART: u8 = 2; // 1 or 2

    let mut paper_rolls: Vec<Vec<u8>> = vec![];
    let mut line_number = 0;
    for line in reader.lines() {
        paper_rolls.push(vec![]);
        let content = line.unwrap();
        for c in content.chars(){
            if c == '.'{
                paper_rolls[line_number].push(0);
            }else if c == '@'{
                paper_rolls[line_number].push(1);
            }else{
                println!("WTF no . or @???");
            }
        }
        line_number += 1;
    }

    if PART == 1{
        let neighboard_count = count_neighboards(&mut paper_rolls);

        let mut counter: u32 = 0;
        for i in 0..neighboard_count.len(){
            for j in 0..neighboard_count[i].len(){
                if neighboard_count[i][j] < 4 && paper_rolls[i][j] == 1{
                    counter += 1;
                }
            }
        }
        println!("Part 1: {}", counter);
    }else if PART == 2{
        let mut total_removed: u32 = 0;
        while true{
            let neighboard_count = count_neighboards(&mut paper_rolls);
            let removed: u32 = remove_free(&mut paper_rolls, & neighboard_count);
            if removed == 0{
                break
            }else{
                total_removed += removed;
            }
        }
        println!("Part 2: {}", total_removed);
    }else{
        println!("PART MUST BE 1 or 2!");
    }
}

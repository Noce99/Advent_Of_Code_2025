use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_input(file_name: &str) -> (Vec<Vec<u64>>, Vec<u64>){
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);

    let mut ranges: Vec<Vec<u64>> = vec![];
    let mut ids: Vec<u64> = vec![];

    let mut first_input_part = true;
    for line in reader.lines() {
        let content = line.unwrap();
        if content == ""{
            first_input_part = false;
        }else if first_input_part{
            if let Some((before, after)) = content.split_once('-') {
                ranges.push(vec![before.parse::<u64>().unwrap(), after.parse::<u64>().unwrap()]);
            }
        }else{
            ids.push(content.parse::<u64>().unwrap());
        }
    }

    println!("Ranges[0] = {}-{}", ranges[0][0], ranges[0][1]);

    let input: (Vec<Vec<u64>>, Vec<u64>) = (ranges, ids);

    input
}

fn part_1(ranges: & Vec<Vec<u64>>, ids: & Vec<u64>) -> u32{
    let mut counter : u32 = 0;
    for id in ids{
        for k in 0..ranges.len(){
            if *id >= ranges[k][0] && *id <= ranges[k][1]{
                counter += 1;
                break;
            }
        }
    }
    counter
}

fn order_ranges_with_start(ranges: & Vec<Vec<u64>>) -> Vec<Vec<u64>>{
    let mut ranges_ordered: Vec<Vec<u64>> = vec![];

    for k in 0..ranges.len(){
        let mut found = false;
        for j in 0..ranges_ordered.len(){
            if ranges[k][0] < ranges_ordered[j][0]{
                if j < ranges_ordered.len(){
                    ranges_ordered.insert(j, ranges[k].clone());
                    found = true;
                }
                break;
            }
        }
        if !found {
            ranges_ordered.push(ranges[k].clone());
        }
    }

    ranges_ordered
}

fn part_2(ranges: & Vec<Vec<u64>>) -> u32{
    let ranges_ordered = order_ranges_with_start(ranges);
    let mut simplified_ranges: Vec<Vec<u64>> = vec![];

    let mut current_start = ranges_ordered[0][0];
    let mut current_end = ranges_ordered[0][1];

    for k in 0..ranges_ordered.len(){
        if ranges_ordered[k][0] > current_end{
            simplified_ranges.push(vec![current_start, current_end]);
            current_start = ranges_ordered[k][0];
            current_end = ranges_ordered[k][1];
        } else if ranges_ordered[k][1] > current_end{
            current_end = ranges_ordered[k][1];
        }
    }
    simplified_ranges.push(vec![current_start, current_end]);


    let mut sum: u64 = 0;
    for k in 0..simplified_ranges.len(){
        println!("{} - {}", simplified_ranges[k][0], simplified_ranges[k][1]);
        sum += (simplified_ranges[k][1] - simplified_ranges[k][0] + 1) as u64;
    }
    println!("SUM = {}", sum);
    return 10;
}

fn main() {
    let (ranges, ids) = get_input("example.txt");
    println!("Part1 = {}", part_1(& ranges, & ids));

    part_2(& ranges);
}

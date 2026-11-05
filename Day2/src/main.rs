struct Range{
    start: u64,
    end: u64,
    digits: u32,
}

impl Range{
    fn search_invalid_ids_1(&self) -> Vec<u64>{
        let mut invalid_ids: Vec<u64> = vec![];
        // If the range is made of a odd number of digits for sure no invalid ids
        if self.digits % 2 == 0{
            // let's suppose n=6 digits (general for each even n)
            let n = self.digits;
            // given start = ABC_DEF we get the n/2 more significant digits
            let half_power_of_ten = 10_u64.pow(n / 2);
            let most_significant_start = self.start / half_power_of_ten as u64;
            let most_significant_end = self.end / half_power_of_ten as u64;

            let smaller_invalid = most_significant_start * (half_power_of_ten + 1);
            let bigger_invalid = most_significant_end * (half_power_of_ten + 1);

            if self.start <= smaller_invalid && smaller_invalid <= self.end{
                invalid_ids.push(smaller_invalid);
            }

            if smaller_invalid != bigger_invalid {
                if self.start <= bigger_invalid && bigger_invalid <= self.end{
                    invalid_ids.push(bigger_invalid);
                }

                for most_significant in most_significant_start+1..most_significant_end{
                    invalid_ids.push(most_significant * (half_power_of_ten + 1));
                }
            }
            
            /*println!("half_power_of_ten = {}", half_power_of_ten);
            println!("most_significant_start = {}", most_significant_start);
            println!("most_significant_end = {}", most_significant_end);
            println!("smaller_invalid = {}", smaller_invalid);
            println!("bigger_invalid = {}", bigger_invalid);*/
        }
        invalid_ids
    }

    fn check_invalid(&self, to_check: u64, digit: u32) -> bool{
        let mut invalid_number = false;
        // println!("@@@@@@@@@@@@@@@@@@@@");
        // println!("Let's check {}", to_check);
        for i in 1..(digit / 2 + 1 as u32) {
            if digit % i == 0{
                let mut all_the_same = true;
                let period = to_check / 10_u64.pow(digit - i);
                // println!("Period that I'm checking: {} of {} digits!", period, i);
                let how_many_periods = digit / i as u32;
                // println!("how_many_periods = {}", how_many_periods);
                let mut last = 0;
                for j in 0..how_many_periods{
                    let removing_lower_digits = (to_check / 10_u64.pow(digit - i*j) as u64) * 10_u64.pow(digit - i*j);
                    let removing_heading_digits = to_check - removing_lower_digits;
                    let digit = removing_heading_digits / 10_u64.pow(digit - i*j - 1);
                    if j != 0 && digit != last{
                        all_the_same = false;
                    }
                    last = digit;
                   // println!("digit = {}", digit);
                }
                // println!("all_the_same = {}", all_the_same);
                if all_the_same{
                    invalid_number = true;
                }
            }
        }
        invalid_number
    }

    fn create_invalid(&self, digit: u32, period: u64) -> Option<u64>{
        // Bisogna controllare che il periodo non contenga dei sottoperiodi
        // tipo 222, 22, 3434 cannot be accepted.
        /*if self.check_invalid(period, digit){
            return None;
        }*/
        let mut invalid_id = 0;
        for k in 0..(self.digits / digit as u32){
            invalid_id += period * 10_u64.pow(digit * k);
        }
        Some(invalid_id)
    }

    fn custom_extend(&self, invalid_ids: &mut Vec<u64>, new_invalid_ids: Vec<u64>){
        for j in 0..new_invalid_ids.len(){
            let mut already_present = false;
            for k in 0..invalid_ids.len(){
                if new_invalid_ids[j] == invalid_ids[k]{
                    already_present = true;
                    break;
                }
            }
            if ! already_present{
                invalid_ids.push(new_invalid_ids[j]);
            }
        }
    }


    fn search_invalid_ids_2_digit(&self, digit: u32) -> Option<Vec<u64>>{
        if self.digits % digit != 0 {
            return None;
        }
        // println!("Number of digits of the period = {}", digit);
        let mut invalid_ids: Vec<u64> = vec![];
        let most_significant_start = self.start / 10_u64.pow(self.digits - digit);
        let most_significant_end = self.end / 10_u64.pow(self.digits - digit);

        // println!("Most Significant Start = {}", most_significant_start);
        // println!("Most Significant End = {}", most_significant_end);

        let first_invalid_option = self.create_invalid(digit, most_significant_start);
        let last_invalid_option = self.create_invalid(digit, most_significant_end);

        // Check that first and last are some!
        if let Some(first_invalid) = first_invalid_option {
            // println!("First Invalid {}", first_invalid);
            if first_invalid >= self.start && first_invalid <= self.end {
                invalid_ids.push(first_invalid);
                //println!("{:?}", invalid_ids);
            }
        }
        if let Some(last_invalid) = last_invalid_option {
            // println!("Last Invalid {}", last_invalid);
            if last_invalid >= self.start && last_invalid <= self.end && last_invalid != first_invalid_option.unwrap_or(0) {
                invalid_ids.push(last_invalid);
            }
        }   
        for period in (most_significant_start + 1)..most_significant_end{
            // println!("Priod = {}", period);
            if let Some(value) = self.create_invalid(digit, period) {
                invalid_ids.push(value);
            }
        }
        /* println!("most_significant_start {}", most_significant_start);
        println!("most_significant_end {}", most_significant_end);
        println!("first_invalid {}", first_invalid);
        println!("last_invalid {}", last_invalid); */
        Some(invalid_ids)
    }


    fn search_invalid_ids_2(&self) -> Vec<u64>{
        let mut invalid_ids: Vec<u64> = vec![];
        for i in 1..(self.digits / 2 + 1 as u32) {
            match self.search_invalid_ids_2_digit(i){
                Some(new_invalid_ids) => {
                    self.custom_extend(&mut invalid_ids, new_invalid_ids);
                },
                None => continue,
            }
        }
        // std::process::exit(1);
        invalid_ids
    }
}

fn common_set_up() -> Vec<Range>{
    // input: String = String::from("42382932-42449104");
    let input: String = String::from("132454-182049,42382932-42449104,685933-804865,5330496-5488118,21-41,289741-376488,220191-245907,49-70,6438484-6636872,2-20,6666660113-6666682086,173-267,59559721-59667224,307-390,2672163-2807721,658272-674230,485679-647207,429-552,72678302-72815786,881990-991937,73-111,416063-479542,596-934,32825-52204,97951700-98000873,18335-27985,70203-100692,8470-11844,3687495840-3687599608,4861-8174,67476003-67593626,2492-4717,1442-2129,102962-121710,628612213-628649371,1064602-1138912");
    // let input: String = String::from("1090286-1131879,3259566-3404881,138124-175118,266204727-266361099,16765-24272,7657360692-7657593676,88857504-88926597,6869078-6903096,48444999-48532270,61427792-61580535,71-103,8077-10421,1920-2560,2-17,951-1259,34-50,28994-36978,1309-1822,9393918461-9393960770,89479-120899,834641-988077,5389718924-5389797353,34010076-34214499,5063-7100,607034-753348,19098586-19261191,125085556-125188689,39839-51927,3246-5037,174-260,439715-473176,187287-262190,348-535,58956-78301,4388160-4505757,512092-584994,13388753-13534387");
    // let input: String = String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
    let ranges_str: Vec<&str> = input.split(',').collect();
    let mut ranges: Vec<Range> = Vec::new();
    let mut tmp: String;
    for range in ranges_str{
        let start_end: Vec<&str> = range.split('-').collect();
        let mut start_str: &str = start_end[0];
        let end_str: &str = start_end[1];
        while start_str.len() < end_str.len(){
            let digit_num: u32 = start_str.len() as u32;
            let tmp_end: u64 = 10_u64.pow(digit_num) - 1;
            ranges.push(Range{
                start: start_str.parse::<u64>().unwrap(),
                end: tmp_end,
                digits: start_str.len() as u32,
            });
            tmp = format!("{}", 10_u64.pow(digit_num));
            start_str = tmp.as_str();
        }
        
        ranges.push(Range{
            start: start_str.parse::<u64>().unwrap(),
            end: end_str.parse::<u64>().unwrap(),
            digits: start_str.len() as u32,
        });
    }

    ranges
}

fn part_one(ranges: Vec<Range>){
    let mut total_sum : u64 = 0;

    for range in ranges{
        println!("[{}; {}, {}]", range.start, range.end, range.digits);
        let invalid_ids : Vec<u64> = range.search_invalid_ids_1();
        println!("Invalid IDS = {:?}", invalid_ids);
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        total_sum += invalid_ids.iter().sum::<u64>();
    }
    println!("Total Sum = {}", total_sum);
}

fn part_two(ranges: Vec<Range>){
    let mut total_sum : u64 = 0;

    for range in ranges{
        println!("[{}; {}, {}]", range.start, range.end, range.digits);
        let invalid_ids : Vec<u64> = range.search_invalid_ids_2();
        println!("Invalid IDS = {:?}", invalid_ids);
        println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@");
        total_sum += invalid_ids.iter().sum::<u64>();
    }
    println!("Total Sum = {}", total_sum);
}

fn main() {
    let start = std::time::Instant::now();
    let ranges = common_set_up();

    // part_one(ranges);
    part_two(ranges);

    let duration = start.elapsed();
    println!("Total Time = {} us", duration.as_micros());
 }

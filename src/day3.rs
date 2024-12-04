use regex::Regex;
use std::fs;

pub fn foo(){
    let path = "day3.txt";
    let unparsed_input: &String =  &fs::read_to_string(path).expect("Unable to read the file");
    let first_regex = r"mul\((\d+),(\d+)\)";

    let muls = parse_muls(*&unparsed_input, first_regex);
    println!("Multiplication sum: {}", add_all_muls(muls));

    let second_regex = r"(do\(\)|don't\(\))|mul\(\d+,\d+\)";
    println!("Multiplication sum with dos and don'ts: {}", add_all_muls_with_does(parse_muls(*&unparsed_input, second_regex)));
}


fn parse_muls<'a>(unparsed_input: &'a str, regex: &'a str) -> Vec<&'a str>{
    let re = Regex::new(regex).unwrap();
    re.captures_iter(unparsed_input).filter_map(|caps| caps.get(0)).map(|m| m.as_str()).collect::<Vec<&str>>()
}

fn add_all_muls(muls: Vec<&str>)->i64{
    muls.iter().map(|s| parse_and_multiply(s)).sum()
}

fn parse_and_multiply(input: &str) -> i64{
    let numbers_str = &input[4..input.len()-1];
    let args: Vec<&str> = numbers_str.split(",").collect();
    let num1:i64 = args[0].trim().parse().expect("Invalid argument");
    let num2:i64 = args[1].trim().parse().expect("Invalid argument");

    num1 * num2
}

fn add_all_muls_with_does(muls: Vec<&str>)->i64{
    let mut can_add = true;
    let mut result = 0;
    for item in muls {
        if item == "do()"{can_add = true; continue;};
        if item == "don't()" {can_add = false; continue;};

        if can_add {result += parse_and_multiply(item);};
    }
    result
}


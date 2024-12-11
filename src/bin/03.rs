use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{izip, multipeek};
use itertools::chain;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";


// include the latest version of the regex crate in your Cargo.toml
extern crate regex;

use regex::Regex;

///Struct to represent mul(.,.) operation results as well as do/don't operations
struct SwitchToggleValue {
    index: usize,
    switch: bool,
    toggle: bool,
    value: usize,
}

impl SwitchToggleValue {
    fn merge(&self, other: &SwitchToggleValue) -> SwitchToggleValue {
        let new_switch = {
            match other.toggle {
                true => other.switch,
                false => self.switch,
            }
        };
        SwitchToggleValue{
            index: other.index,
            switch: new_switch,
            toggle: false,
            value: self.value + (new_switch as usize) * other.value,
        }
    }
}


fn mul_ops_from_string(input: &str) -> Vec<SwitchToggleValue> {
    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();

    let matches = regex.captures_iter(&input);
    let mut ops: Vec<SwitchToggleValue> = Vec::new();
    for mat in matches {
        let op1 = usize::from_str(&mat[1]).unwrap();
        let op2 = usize::from_str(&mat[2]).unwrap();
        ops.push(SwitchToggleValue {index: mat.get(0).unwrap().start(), switch:true, toggle:false, value:op1 * op2});
    }
    ops
}

fn dos_donts_from_string(input: &str) -> Vec<SwitchToggleValue> {
    let mut dos_donts: Vec<SwitchToggleValue> = Vec::new();
    for (index, _) in input.match_indices("do()") {
        dos_donts.push(SwitchToggleValue{index, switch: true, toggle:true, value: 0});
    }
    for (index, _) in input.match_indices("don't()") {
        dos_donts.push(SwitchToggleValue{index, switch: false, toggle:true, value: 0});
    }
    dos_donts
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: String = reader.lines().flatten().collect();
        let mul_ops = mul_ops_from_string(&input);
        let sum_value: SwitchToggleValue = mul_ops.iter().fold(SwitchToggleValue {index: 0, switch: true, toggle:false, value: 0}, |acc, x| acc.merge(x));
        Ok(sum_value.value)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
     println!("\n=== Part 2 ===");

     fn part2<R: BufRead>(reader: R) -> Result<usize> {
         let input: String = reader.lines().flatten().collect();
         let mul_ops = mul_ops_from_string(&input);
         let dos_donts = dos_donts_from_string(&input);
         let mut all_ops: Vec<SwitchToggleValue> = chain(mul_ops, dos_donts).collect();
         all_ops.sort_by_key(|x| x.index);
         let sum_value: SwitchToggleValue = all_ops.iter().fold(SwitchToggleValue {index: 0, switch: true, toggle:false, value: 0}, |acc, x| acc.merge(x));

         Ok(sum_value.value)
     }

     assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);

     let input_file = BufReader::new(File::open(INPUT_FILE)?);
     let result = time_snippet!(part2(input_file)?);
     println!("Result = {}", result);
    //endregion

    Ok(())
}

use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";


// include the latest version of the regex crate in your Cargo.toml
extern crate regex;

use regex::Regex;



fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: String = reader.lines().flatten().collect();
        let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)")?;

        let matches = regex.captures_iter(&input);
        let mut sum: usize = 0;
        for mat in matches {
            let op1 = usize::from_str(&mat[1])?;
            let op2 = usize::from_str(&mat[2])?;
            sum += op1 * op2;
        }
        Ok(sum)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
/*     println!("\n=== Part 2 ===");

     fn part2<R: BufRead>(reader: R) -> Result<usize> {
         Ok(0)
     }

     assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

     let input_file = BufReader::new(File::open(INPUT_FILE)?);
     let result = time_snippet!(part2(input_file)?);
     println!("Result = {}", result);*/
    //endregion

    Ok(())
}

use anyhow::*;
use std::fs::File;
use std::i64;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::str::FromStr;
use std::iter::zip;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn input_to_vec_pair<R: BufRead>(reader: R) -> Result<(Vec<i64>, Vec<i64>)> {
        let mut left: Vec<i64> = Vec::new();
        let mut right: Vec<i64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split("   ").collect();
            assert_eq!(parts.len(), 2);
            left.push(i64::from_str(parts[0])?);
            right.push(i64::from_str(parts[1])?);
        }
        Ok((left, right))
    }

    fn part1<R: BufRead>(reader: R) -> Result<i64> {
        let (mut left, mut right) = input_to_vec_pair(reader)?;
        left.sort();
        right.sort();
        let pairs = zip(left.iter(), right.iter());
        let diffs = pairs.map(|pair| (pair.0 - pair.1).abs());
        let sum: i64 = diffs.fold(0, |a, b| a + b);
        Ok(sum)
    }

    let test_answer = part1(BufReader::new(TEST.as_bytes()))?;
    assert_eq!(11, test_answer);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
     println!("\n=== Part 2 ===");

     fn part2<R: BufRead>(reader: R) -> Result<i64> {

         let (mut left, mut right) = input_to_vec_pair(reader)?;
         // Let's do this the inefficient way.
         // No sorting, no potential re-using of similarity scores
         let sim_scores = left.iter().map(|l| *l * i64::try_from(right.iter().filter(|r| **r == *l).count()).unwrap()).fold(0, |a, b| a + b );

         Ok(sim_scores)
     }

     let test_answer = part2(BufReader::new(TEST.as_bytes()))?;
     assert_eq!(31, test_answer);

     let input_file = BufReader::new(File::open(INPUT_FILE)?);
     let result = time_snippet!(part2(input_file)?);
     println!("Result = {}", result);
    //endregion

    Ok(())
}

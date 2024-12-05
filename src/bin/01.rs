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

    ///
    /// Read input file and map to left and right vectors of numbers
    ///
    fn input_to_vec_pair<R: BufRead>(reader: R) -> Result<(Vec<u64>, Vec<u64>)> {
        let mut left: Vec<u64> = Vec::new();
        let mut right: Vec<u64> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split("   ").collect();
            assert_eq!(parts.len(), 2);
            left.push(u64::from_str(parts[0])?);
            right.push(u64::from_str(parts[1])?);
        }
        Ok((left, right))
    }

    fn part1<R: BufRead>(reader: R) -> Result<u64> {
        let (mut left, mut right) = input_to_vec_pair(reader)?;
        // Sort
        left.sort();
        right.sort();
        // Build pair iterator
        let pairs = zip(left.iter(), right.iter());
        // Map pairs to abs diffs
        fn abs_diff(a: u64, b: u64) -> u64 {
            if a > b {
                a - b
            } else {
                b - a
            }
        }
        let diffs = pairs.map(|pair| abs_diff(*pair.0, *pair.1));
        // Sum up
        let sum: u64 = diffs.fold(0, |a, b| a + b);
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

     fn part2<R: BufRead>(reader: R) -> Result<u64> {

         let (mut left, mut right) = input_to_vec_pair(reader)?;
         // Let's do this the inefficient way.
         // No sorting, no potential re-using of similarity scores
         fn count_in_right(x: u64, right: &Vec<u64>) -> u64  {
             u64::try_from(right.iter().filter(|r| **r == x).count()).unwrap()
         }
         let sim_scores = left.iter().map(|l| *l * count_in_right(*l, &right)).fold(0, |a, b| a + b);

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

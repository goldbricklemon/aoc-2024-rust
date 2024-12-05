use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;
use std::str::FromStr;
use std::iter::zip;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";


///
/// Read input file and map to Vec of number Vecs
///
fn input_to_reports<R: BufRead>(reader: R) -> Result<Vec<Vec<u32>>> {
    let mut reports: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let report: Vec<u32> = line?.split(" ").map(|s| u32::from_str(s).unwrap()).collect();
        reports.push(report);
    }
    Ok(reports)
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let reports = input_to_reports(reader)?;

    fn report_is_safe(report: &Vec<u32>) -> bool {
        let mut safe: bool = true;
        // Report increasing (1) or decreasing (-1)
        let mut incdec: i64 = 0;
        for (cur, next) in report[.. report.len() - 1].iter().zip(&report[1..]) {
            let diff = i64::from(*next) - i64::from(*cur);
            let absdiff = diff.abs();
            if absdiff == 0 || absdiff > 3 {
                safe = false;
                break;
            } else{
                let cur_incdec = diff.signum();
                // Product is 0 for the very first pair
                // 1 for matching incdec
                // -1 for missmatch in incdec
                if (cur_incdec * incdec).signum() == -1 {
                    safe = false;
                    break;
                }
                incdec = cur_incdec;
            }
        }
        return safe;
    }

    let num_safe_reports = reports.iter().filter(|&r| report_is_safe(r)).count();

    Ok(num_safe_reports)
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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

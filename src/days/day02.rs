use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    // Your solution here...
    let input = include_str!("input.txt");
    let sol1 = count_safe_reports(input);
    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
       input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|el| el.parse().ok())
                    .collect()
            })
            .collect()
    }

fn count_safe_reports(input: &str) -> usize {
    let reports = parse_input(input);
    let mut count = 0;
    for report in reports {
        if report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| a > b) {
            println!("report is {:?}", report);
            let mut safe = "true";
            for n in 0..(report.len()-1) {
                if report[n].abs_diff(report[n+1]) > 3 || report[n].abs_diff(report[n+1]) < 1{
                    safe = "false";
                    }
                }
            if safe == "true" {count += 1;};
            }
            println!("count is {:?}", count);
        }
    println!("final count is {:?}", count);
    count
    }

    // fn generate_dampened_reports(input: Vec<usize>) -> Vec<Vec<usize>> {
    //     println!("input is {:?}", input);
    //     let mut dampened_reports = vec![];
    //     for i in 0..input.len()-1 {
    //         dampened_reports.push(input.clone());
    //         }
    //     println!("dampened is {:?}", dampened_reports);
    //     dampened_reports
    //     }


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9"
    }

    fn test_input_2() -> Vec<usize> {
        vec![7, 6, 4, 2, 1]
    }

    #[rstest]
    fn test_parse_input(test_input: &str) {
        assert_eq!(parse_input(test_input), (vec![vec![7, 6, 4, 2, 1], vec![1, 2, 7, 8, 9], vec![9, 7, 6, 2, 1], vec![1, 3, 2, 4, 5], vec![8, 6, 4, 4, 1], vec![1, 3, 6, 7, 9]]));
    }

    #[rstest]
    fn test_count_safe_reports(test_input: &str) {
        assert_eq!(count_safe_reports(test_input), 2)
        }

    // #[rstest]
    // fn test_generate_dampened_reports(test_input_2: Vec<usize>) {
    //     assert_eq!(generate_dampened_reports(test_input_2), vec![vec![6,4,2,1], vec![7,4,2,1], vec![7,6,2,1], vec![7,6,4,2]])
    //     }

}

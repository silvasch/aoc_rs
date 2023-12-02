use aoc_rs::{AocError, Solution, Solver};

struct PartOneSolution;

impl Solution for PartOneSolution {
    fn get_solution(&mut self, input: &str) -> Result<i64, aoc_rs::AocError> {
        let input = input.trim();
        match input.parse::<i64>() {
            Ok(solution) => Ok(solution),
            Err(_) => Err(AocError::SolutionFailed(
                "failed to parse the file".to_string(),
            )),
        }
    }
}

struct PartTwoSolution;

impl Solution for PartTwoSolution {
    fn get_solution(&mut self, input: &str) -> Result<i64, aoc_rs::AocError> {
        let input = input.trim();
        match input.parse::<i64>() {
            Ok(solution) => Ok(solution + 10),
            Err(_) => Err(AocError::SolutionFailed(
                "failed to parse the file".to_string(),
            )),
        }
    }
}

fn main() {
    Solver::new(
        1,
        (Box::new(PartOneSolution), Some(10)),
        (Box::new(PartTwoSolution), Some(22)),
    )
    .solve()
    .unwrap();
}

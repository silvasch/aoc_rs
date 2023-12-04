use clap::Parser;

use crate::{
    args::{Args, PartSelector},
    prelude::*,
};

pub struct Solver {
    args: Args,
    logger: Logger,

    day: u8,

    part_one_solution: Box<dyn Solution>,
    part_one_expected: Option<i64>,
    part_two_solution: Box<dyn Solution>,
    part_two_expected: Option<i64>,
}

impl Solver {
    pub fn new(
        day: u8,
        part_one: (Box<dyn Solution>, Option<i64>),
        part_two: (Box<dyn Solution>, Option<i64>),
    ) -> Self {
        let args = Args::parse();
        let logger = Logger::new(!args.hide_log);

        Self {
            args,
            logger,
            day,
            part_one_solution: part_one.0,
            part_one_expected: part_one.1,
            part_two_solution: part_two.0,
            part_two_expected: part_two.1,
        }
    }

    pub fn solve(&mut self) -> Result<(), AocError> {
        let input = self.get_input()?;
        if self.args.print_input {
            self.logger.lib_log(
                &format!(">> input start\n{}\n>> input end", input),
                LogLevel::Info,
            );
        }

        let expected = self.get_expected();
        let solution = match self.args.part {
            PartSelector::PartOne => &mut self.part_one_solution,
            PartSelector::PartTwo => &mut self.part_two_solution,
        };

        if self.args.real {
            match solution.get_solution(&input, &self.logger) {
                Ok(solution) => {
                    self.logger
                        .lib_log(&format!("solution: {}", solution), LogLevel::Success);
                    Ok(())
                }
                Err(e) => Err(e),
            }
        } else {
            match solution.get_solution(&input, &self.logger) {
                Ok(solution) => match expected {
                    Some(expected) => {
                        if expected == solution {
                            self.logger.lib_log("test successfull", LogLevel::Success);
                            Ok(())
                        } else {
                            Err(AocError::TestFailed(expected, solution))
                        }
                    }
                    None => {
                        self.logger
                            .lib_log(&format!("solution (test): {}", solution), LogLevel::Success);
                        Ok(())
                    }
                },
                Err(e) => Err(e),
            }
        }
    }

    fn get_input(&self) -> Result<String, AocError> {
        let file_suffix = if self.args.real {
            ""
        } else {
            match self.args.part {
                PartSelector::PartOne => "_test1",
                PartSelector::PartTwo => "_test2",
            }
        };

        let file_name = format!("input/day{:02}{}", self.day, file_suffix);

        self.logger.lib_log(
            &format!("using '{}' as input file", file_name),
            LogLevel::Info,
        );

        match std::fs::read_to_string(&file_name) {
            Ok(file_contents) => Ok(file_contents),
            Err(_) => Err(AocError::InputReadFailed(file_name)),
        }
    }

    fn get_expected(&self) -> Option<i64> {
        match self.args.part {
            PartSelector::PartOne => self.part_one_expected,
            PartSelector::PartTwo => self.part_two_expected,
        }
    }
}

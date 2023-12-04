pub(crate) mod args;

mod error;
pub use error::AocError;

mod logger;
pub use logger::{LogLevel, Logger};

mod solution;
pub use solution::Solution;

mod solver;
pub use solver::Solver;

pub mod prelude {
    pub use super::{AocError, LogLevel, Logger, Solution, Solver};
}

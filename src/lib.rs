pub(crate) mod args;

mod error;
pub use error::AocError;

mod solution;
pub use solution::Solution;

mod solver;
pub use solver::Solver;

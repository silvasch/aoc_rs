use crate::{AocError, Logger};

pub trait Solution {
    #[allow(unused)]
    fn get_solution(&mut self, input: &str, logger: &Logger) -> Result<i64, AocError> {
        Err(AocError::NotImplemented)
    }
}

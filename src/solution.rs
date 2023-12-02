use crate::AocError;

pub trait Solution {
    fn get_solution(&mut self, _input: &str) -> Result<i64, AocError> {
        Err(AocError::NotImplemented)
    }
}

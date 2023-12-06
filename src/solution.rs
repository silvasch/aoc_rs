use crate::AocError;

pub trait Solution {
    #[allow(unused)]
    fn get_solution(&mut self, input: &str) -> Result<i64, AocError> {
        Err(AocError::NotImplemented)
    }
}

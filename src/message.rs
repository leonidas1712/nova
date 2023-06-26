use std::ops::{Deref,DerefMut};

#[derive(Debug)]
pub struct NovaResult<T> {
    pub result: T,
    pub message: Option<String>,
}

// so we can easily call methods and fields
impl<T> Deref for NovaResult<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl<T> DerefMut for NovaResult<T> {    
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.result
    }
    
}

#[derive(Debug)]
pub struct NovaError {
    pub message:String,
}

impl NovaError {
    pub fn format_error(&self)->String{
        format!("Error: {}", self.message)
    }
}

impl<T> NovaResult<T> {
    pub fn new(result:T)->NovaResult<T>{
        NovaResult {
            result,
            message:None
        }
    }

    pub fn add_msg(self,msg:&str)->NovaResult<T> {
        NovaResult {
            result:self.result,
            message:Some(msg.to_string())
        }
    }
}

impl NovaError {
    pub fn new(msg:&str)->NovaError {
        NovaError { message:msg.to_string() }
    }
}

pub type Result<T> = std::result::Result<NovaResult<T>, NovaError>;

#[cfg(test)]
pub mod test {
    use super::{NovaResult, NovaError};

    #[test]
    fn nova_result_test_new() {
        let nr=NovaResult::new(20);
        assert_eq!(nr.result, 20);
        assert_eq!(nr.message, None);
    }

    #[test]
    fn nova_result_test_add_msg() {
        let nr=NovaResult::new(vec![1,2]);
        let n2=nr.add_msg("hi");
        assert_eq!(n2.result, vec![1,2]);
        assert_eq!(n2.message.unwrap(), "hi");
    }

    #[test]
    fn nova_error_test_new() {
        let ne=NovaError::new("Some error");
        assert_eq!(ne.format_error(), "Error: Some error");
    }
}



#[derive(Debug)]
pub struct NovaResult<T> {
    result: T,
    message: Option<String>,
}

#[derive(Debug)]
pub struct NovaError {
    message:String,
}

impl NovaError {
    pub fn format_error(&self)->String{
        format!("Error: {}", self.message)
    }
}

impl<T> NovaResult<T> {
    fn new(result:T)->NovaResult<T>{
        NovaResult {
            result,
            message:None
        }
    }

    fn add_msg(self,msg:String)->NovaResult<T> {
        NovaResult {
            result:self.result,
            message:Some(msg)
        }
    }
}

impl NovaError {
    fn new(message:String)->NovaError {
        NovaError { message }
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
        let n2=nr.add_msg(String::from("hi"));
        assert_eq!(n2.result, vec![1,2]);
        assert_eq!(n2.message.unwrap().as_str(), "hi");
    }

    #[test]
    fn nova_error_test_new() {
        let ne=NovaError::new("Some error".to_string());
        assert_eq!(ne.format_error(), "Error: Some error");
    }
}


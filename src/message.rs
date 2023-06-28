use std::{ops::{Deref,DerefMut}};

#[derive(Debug)]
pub struct NovaResult<T> {
    pub result: T,
    pub messages: Vec<String>,
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
    message:String,
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
            messages:vec![]
        }
    }

    pub fn add_msg(self,msg:&str)->NovaResult<T> {
        let mut old_messages=self.messages;
        let mut new_messages=vec![];

        new_messages.append(&mut old_messages);
        new_messages.push(msg.to_string());
        
        NovaResult {
            result:self.result,
            messages:new_messages
        }
    }

    // add messages from another result, consuming it
    pub fn add_messages<U>(&mut self, other:&NovaResult<U>) {
        if other.messages.len()==0 {
            return;
        }

        for msg in other.messages.iter() {
            self.messages.push(msg.clone());
        }
        // self.messages.append(&mut other.messages);
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
        assert_eq!(nr.messages, Vec::<String>::new());
    }

    #[test]
    fn nova_result_test_add_msg() {
        let nr=NovaResult::new(vec![1,2]);
        let n2=nr.add_msg("hi");
        let mut n3=n2.add_msg("hello");

        assert_eq!(n3.messages, vec!["hi","hello"]);

        let mut some_res=NovaResult::new("string")
            .add_msg("msg from string")
            .add_msg("msg from string 2");

        some_res.add_messages(&mut n3);

        assert_eq!(some_res.messages, vec!["msg from string", "msg from string 2", "hi", "hello"]);

        // let mut nr2=NovaResult::new(30).add_msg("Nr2");
        // some_res.add_messages(&mut nr2);
        // dbg!(some_res);

    }

    #[test]
    fn nova_error_test_new() {
        let ne=NovaError::new("Some error");
        assert_eq!(ne.format_error(), "Error: Some error");
    }
}


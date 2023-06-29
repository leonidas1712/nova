#[derive(Debug)]
// Ex for Exception

// add macro to create error
pub struct Ex {
    message: String,
    // type: Parse,Eval...
}

impl Ex {
    pub fn format_error(&self) -> String {
        format!("Error: {}", self.message)
    }
}

impl Ex {
    pub fn new(msg: &str) -> Ex {
        Ex {
            message: msg.to_string(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Ex>;

#[test]
fn nova_error_test_new() {
    let ne = Ex::new("Some error");
    assert_eq!(ne.format_error(), "Error: Some error");

    let v:Vec<std::result::Result<u32,u32>>=vec![Ok(1),Ok(2), Ok(1), Ok(2), Ok(3)];
    let k:std::result::Result<Vec<u32>,_>=v.into_iter().collect();

    let v:Vec<Result<u32>>=vec![Ok(1),Ok(2), Ok(1), Err(Ex::new("oops"))];
    let k:Result<Vec<u32>>=v.into_iter().collect();
}


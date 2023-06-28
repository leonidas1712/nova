#[derive(Debug)]
// Ex for Exception
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
}


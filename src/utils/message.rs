// Ex for Exception

// add macro to create error
#[macro_export]
macro_rules! err {
    ($msg:expr) => {
        Err(Ex::new(&$msg))
    };
}

#[macro_export]
macro_rules! errf {
    ($msg:expr, $( $var:expr ),*) => {
        Err(Ex::new(format!($msg, $($var)*).as_str()))
    };
}

pub use err;
pub use errf;

#[derive(Debug)]
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

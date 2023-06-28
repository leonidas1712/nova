macro_rules! prints {
    ($($arg:expr),*) => {
        $(println!("{}", $arg);)*
    };
}

pub(crate) use prints;
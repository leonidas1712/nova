#![macro_use]

macro_rules! prints {
    ($($arg:expr),*) => {
        $(println!("{}", $arg);)*
    };
}
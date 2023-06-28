#[cfg(test)]
use crate::evaluator::evaluate;

pub mod integration_tests {
    #[test]
    fn builtin_test() {
        evaluate()
    }
}
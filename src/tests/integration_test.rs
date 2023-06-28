use nova::evaluator::evaluator::evaluate_input;

#[test]
// arithmetic
fn calc_test() {
    let exp="(add 10 (sub (add 5 2) (sub 8 3)) (add (sub 4 1) 6))";
    let res=evaluate_input(exp);
    dbg!(res);
}
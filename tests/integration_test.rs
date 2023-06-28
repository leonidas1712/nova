use nova::{evaluate_input, evaluator::context::Context, setup_context};

fn compare(inp:&str, expected:&str, ctx:&mut Context) {
    let res=evaluate_input(inp, ctx);
    assert_eq!(res, expected);
}

fn compare_many(inputs:Vec<&str>, expected:Vec<&str>,  ctx:&mut Context) {
    
}

#[test]
// arithmetic
fn calc_test() {
    let mut ctx=setup_context();
    let exp="(add 10 (sub (add 5 2) (sub 8 3)) (add (sub 4 1) 6))";
    compare(exp, "21", &mut ctx);
}
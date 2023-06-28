use nova::{evaluate_input, evaluator::context::Context, setup_context};

fn compare(inp:&str, expected:&str, ctx:&mut Context) {
    let res=evaluate_input(inp.trim(), ctx);
    assert_eq!(res, expected.trim());
}

fn compare_many(inputs:Vec<&str>, expected:Vec<&str>,  ctx:&mut Context) {
    inputs
    .into_iter()
    .zip(expected.into_iter())
    .for_each(|tup| compare(tup.0, tup.1, ctx));
}

#[test]
// arithmetic
fn calc_test() {
    let mut ctx=setup_context();
    let exprs=vec![
        "(add 10 (sub (add 5 2) (sub 8 3)) (add (sub 4 1) 6))",
        "(add 4 5 (add 1 2) (sub 3 5 (mul 4 1 3 -5)))",
        "add 5 6 3 4",
        "add true false",
        "(add sub 5)"
    ];

    let expected=vec!["21","70", "18", "Error: Expected a number but got 'true'","Error: Expected a number but got 'sub'"];

    compare_many(exprs, expected, &mut ctx);
}
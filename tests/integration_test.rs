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

    let expected=vec!["21","70", "18", "Error: Expected a number but got 'true'","Error: Expected a number but got '<function 'sub'>'"];

    compare_many(exprs, expected, &mut ctx);
}

#[test]
fn if_test() {
    let mut ctx=setup_context();
    let exprs=vec![
        "(if (if true (add 0 0) (sub 5 4)) (add 10 20 30) (sub 5 (if 1 2 4) 7))",
        "if (add 0 0) (add 5 6) (mul 5 6)" // no brackets for outermost if
        ];
    let expected=vec!["-4", "30"];
    compare_many(exprs, expected, &mut ctx);
}

// (let x (if true (add 5 6), (sub (mul 10 20) (add 20 30) (if 1 2 3))),let y (let z (if (add 5 6) (sub 3 4) 0)),let z (add x y),let k (sub x y),(mul z k))



#[test]
fn let_test() {
    let inps=vec![
        "let x 2 y 3 (add x y)",
        "(let x (add 5 (if 1 2 (sub 5 6))) x)",
        "(let x (let y (let z 5) y) x)",
        "(let x (let y (if true (add 1 2) (sub 55 66))) y (if true 20 30) (add x y))",
        "(
            let x (if true (add 5 6), (sub (mul 10 20) (add 20 30) (if 1 2 3))),
            y (let z (if (add 5 6) (sub 3 4) 0)),
            z (add x y),
            k (sub x y),
            (mul z k)
        )",
    
    ];

    let exp=vec!["5","7","5","23","120"];
    let mut ctx=setup_context();
    compare_many(inps, exp, &mut ctx)
}

// let x 2 => x=2 in global
// (let x 2) => x not assigned
#[test]
fn test_let_global() {
    let mut ctx=setup_context();
    let expr="(let x 2)";
    evaluate_input(expr, &mut ctx);
    assert!(ctx.get_variable("x").is_none());

    let expr="let x 2";
    evaluate_input(expr, &mut ctx);
    assert!(ctx.get_variable("x").is_some());

    let expr="let x 3";
    evaluate_input(expr, &mut ctx);
    assert_eq!(ctx.get_variable("x").unwrap().expect_num().unwrap(), 3);
}



// (let x 2,let y (let x 3),(add x y))
#![recursion_limit = "5000"]
use nova::{evaluate_all, evaluate_input_tco, evaluator::context_tco::EvalContext};
// fn compare(inp: &str, expected: &str, ctx: &mut EvalContext) {
//     let res = evaluate_input_tco(inp.trim(), ctx);
//     assert_eq!(res, expected.trim());
// }

fn compare(inp: &str, expected: &str, ctx: &mut EvalContext) {
    let res = evaluate_all(inp.trim(), ctx);
    match res {
        Ok(strings) => {
            let res = strings.get(0).unwrap();
            assert_eq!(res.result, expected.trim());
        }
        Err(err) => {
            println!("{}", err.format_error());
            assert_eq!(err.format_error(), expected.trim());
        }
    }
}

fn compare_many(inputs: Vec<&str>, expected: Vec<&str>, ctx: &mut EvalContext) {
    inputs
        .into_iter()
        .zip(expected.into_iter())
        .for_each(|tup| {
            compare(tup.0, tup.1, ctx);
        });
}

#[test]
// arithmetic
fn calc_test() {
    let mut ctx = EvalContext::new();
    let exprs = vec![
        "(add 10 (sub (add 5 2) (sub 8 3)) (add (sub 4 1) 6))",
        "(add 4 5 (add 1 2) (sub 3 5 (mul 4 1 3 -5)))",
        "add 5 6 3 4",
        "add true false",
        "(add sub 5)",
    ];

    let expected = vec![
        "21",
        "70",
        "18",
        "1",
        "Error: Expected a number but got '<function 'sub'>'",
    ];

    compare_many(exprs, expected, &mut ctx);
}

#[test]
fn if_test() {
    let mut ctx = EvalContext::new();
    let exprs = vec![
        "(if (if true (add 0 0) (sub 5 4)) (add 10 20 30) (sub 5 (if 1 2 4) 7))",
        "if (add 0 0) (add 5 6) (mul 5 6)", // no brackets for outermost if
    ];
    let expected = vec!["-4", "30"];
    compare_many(exprs, expected, &mut ctx);
}

// (let x (if true (add 5 6), (sub (mul 10 20) (add 20 30) (if 1 2 3))),let y (let z (if (add 5 6) (sub 3 4) 0)),let z (add x y),let k (sub x y),(mul z k))

#[test]
fn let_test() {
    let inps = vec![
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

    let exp = vec!["5", "7", "5", "23", "120"];
    let mut ctx = EvalContext::new();
    compare_many(inps, exp, &mut ctx)
}

// // let x 2 => x=2 in global
// // (let x 2) => x not assigned
#[test]
fn test_let_global() {
    let mut ctx = EvalContext::new();
    let expr = "(let x 2)";
    evaluate_input_tco(expr, &mut ctx);
    assert!(ctx.read().get_variable("x").is_none());

    let expr = "let x 2";
    evaluate_input_tco(expr, &mut ctx);
    assert!(ctx.read().get_variable("x").is_some());

    let expr = "let x 3";
    evaluate_input_tco(expr, &mut ctx);
    assert_eq!(
        ctx.read().get_variable("x").unwrap().expect_num().unwrap(),
        3
    );
}

#[test]
pub fn fn_test() {
    let mut ctx = EvalContext::new();

    let inputs = vec![
        "(def id (x) x)",
        "(id 1)",
        "(let x 2 y (def g (a) (add a x)) (y x))",
        "(def recr (n) 
            (if (eq n 0) 0 
            (add n (recr (pred n)))
        ))",
        "(recr 10)",
        "(recr 50)",
        "(recr 1000)",
    ];

    let expected = vec![
        "id(x) => x",
        "1",
        "4",
        "recr(n) => (if (eq n 0) 0 (add n (recr (pred n))))",
        "55",
        "1275",
        "500500",
    ];

    compare_many(inputs, expected, &mut ctx);
}

// test ; separation
#[test]
pub fn evaluate_all_test() {
    let mut ctx = EvalContext::new();
    let expr = "let x 2;\nlet y 3;\nlet z 5;\n (add x y z)";
    let res = evaluate_all(expr, &mut ctx);

    let res = res.unwrap();
    let expected = ["2", "3", "5", "10"];

    for (actual, exp) in res.iter().zip(expected.iter()) {
        assert_eq!(&actual.result.as_str(), exp);
    }
}

// returned function not added to fn defs
#[test]
pub fn test_fn_return() {
    let func = "(def fn (x) (def fn2 (y) (add x y)))";
    let mut ctx = EvalContext::new();
    let res = evaluate_all(func, &mut ctx).expect("Should define function.");
    println!("{:?}", res.get(0).expect("Should have at least one node"));

    evaluate_all("(fn 1)", &mut ctx).expect("Should call function.");
    assert!(ctx.read().get_function("fn2").is_none());

    evaluate_all("(def app (f elem) (f elem))", &mut ctx).expect("Should define app");
    let call = "(app (def h(x) (succ x)) 1)"; // not global so h(x) eval as fn var

    let res = evaluate_all(call, &mut ctx).expect("Should call app on h");
    let res = res.get(0).expect("Should have one result");

    assert!(res.result.eq("2"));
}

#[test]
pub fn curry_test() {
    let app = "(def app (f elem) (f elem))";
    let func = "(def fn (a,b) (add a b))";
    let f2 = "(def fn2 (a,b,c) (add a b c))";

    let mut ctx = EvalContext::new();
    evaluate_all(app, &mut ctx).expect("Should be ok");
    evaluate_all(func, &mut ctx).expect("Should be ok");
    evaluate_all(f2, &mut ctx).expect("Should be ok");

    compare("(fn 1)", "fn(b) => (add a b)", &mut ctx);
    compare("(app fn 10)", "fn(b) => (add a b)", &mut ctx);
    compare("(app (fn 10) 20)", "30", &mut ctx);
    compare("(app (add 1) 10)", "11", &mut ctx);
    compare("((((mul 1) 2) 3 4))", "24", &mut ctx);
    compare("let h (fn2 10 20)", "fn2(c) => (add a b c)", &mut ctx);
    compare("(h 15)", "45", &mut ctx);
}
// // (let x 2, y (let x 3),(add x y))

// // ((map fn) x) -> (map fn) res is fn call
// // (g (map fn) x) -> (map fn) res is variable

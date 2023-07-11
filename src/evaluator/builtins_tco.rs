use std::cmp::Ordering;
use std::rc;
use std::rc::Rc;

use crate::constants::*;
use crate::message::*;

use super::context_tco::*;
use super::data_tco::*;
use super::evaluator_tco::*;
use super::function_tco::*;
use super::params::Params;

macro_rules! name {
    ($name:expr) => {
        String::from(format!("<function '{}'>", $name))
    };
}

// &[Arg] -> Vec<DataValue>
macro_rules! ev {
    ($args:expr) => {
        Arg::expect_all_eval($args)?
    };
}

macro_rules! check {
    ($name:expr, $num:expr, $args:expr) => {
        if $args.len() != $num {
            let msg = format!(
                "'{}' expected {} arguments but received {}.",
                $name,
                $num,
                $args.len()
            );
            return err!(msg);
        }
    };
}

macro_rules! unit {
    () => {
        Ok(EvaluatedExpr(Unit))
    };
}

fn get_nums(args: &[Arg]) -> Result<Vec<NumType>> {
    let r: Result<Vec<NumType>> =
        Arg::expect_all_eval(args).and_then(|f| f.into_iter().map(|x| x.expect_num()).collect());
    return r;
}

type Exec = fn(&[Arg], &EvalContext) -> Result<Expression>;

#[derive(Clone)]
pub struct BuiltIn {
    pub name: String,
    params: Params,
    exec_fn: Exec,
    arg_type: ArgType,
}

impl BuiltIn {
    pub fn new(name: String, params: Params, exec_fn: Exec, arg_type: ArgType) -> Self {
        BuiltIn {
            name,
            params,
            exec_fn,
            arg_type,
        }
    }
}

impl Function for BuiltIn {
    fn apply(&self, args: &[Arg]) -> Rc<dyn Function> {
        let new_fn = BuiltIn {
            name: self.name.clone(),
            params: self.params.apply(args),
            exec_fn: self.exec_fn,
            arg_type: self.arg_type.clone(),
        };
        Rc::new(new_fn)
    }

    fn execute(&self, args: &[Arg], context: &EvalContext) -> Result<Expression> {
        (self.exec_fn)(args, context)
    }

    fn resolve(&self, context: &EvalContext) -> Result<Expression> {
        match &self.params {
            Params::Finite(fin) => match fin.params_diff() {
                // when less return curried function
                Ordering::Less => Ok(EvaluatedExpr(FunctionVariable(Rc::new(self.clone())))),
                Ordering::Equal => self.execute(&fin.received_args, context),
                Ordering::Greater => {
                    let msg = format!(
                        "'{}' expected {} arguments but received {}.",
                        self.name,
                        fin.params.len(),
                        fin.received_args.len()
                    );
                    err!(msg)
                }
            },
            Params::Infinite(inf) => {
                if inf.received_args.len() < inf.min {
                    Ok(EvaluatedExpr(FunctionVariable(Rc::new(self.clone()))))
                } else {
                    self.execute(&inf.received_args, context)
                }
            }
        }
    }

    fn get_arg_type(&self) -> ArgType {
        self.arg_type.clone()
    }

    fn to_string(&self) -> String {
        // format!("<function '{}' len:{}, recv:{}>", self.name, self.params.clone().received_args().len(),self.params.to_string())
        format!("<function '{}'>", self.name)
    }
}

pub struct BuiltInBuilder {
    name: Option<String>,
    params: Option<Params>,
    exec_fn: Option<Exec>,
    arg_type: Option<ArgType>,
}

impl BuiltInBuilder {
    pub fn new() -> Self {
        BuiltInBuilder {
            name: None,
            params: None,
            exec_fn: None,
            arg_type: None,
        }
    }

    // arg type evaluated
    pub fn new_default() -> Self {
        BuiltInBuilder::new().arg_type(ArgType::Evaluated)
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name.replace(name.to_string());
        self
    }

    pub fn params(mut self, params: Params) -> Self {
        self.params.replace(params);
        self
    }

    pub fn exec(mut self, exec_fn: Exec) -> Self {
        self.exec_fn.replace(exec_fn);
        self
    }

    pub fn arg_type(mut self, arg_type: ArgType) -> Self {
        self.arg_type.replace(arg_type);
        self
    }

    pub fn build(self) -> BuiltIn {
        let name = self.name.expect("Empty name");
        let params = self.params.expect("Empty params");
        let exec = self.exec_fn.expect("Empty exec");
        let arg_type = self.arg_type.expect("Empty arg type");
        BuiltIn::new(name, params, exec, arg_type)
    }
}

// (((add 1) 2) 3 4)
fn add(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    let r = get_nums(args);
    let total: Result<NumType> = r.map(|v| v.into_iter().sum());

    total.map(|n| Num(n)).map(|val| EvaluatedExpr(val))
}

fn sub(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    get_nums(args)
        .map(|v| v.into_iter().reduce(|acc, e| acc - e))?
        .ok_or(Ex::new("Could not subtract provided expression"))
        .map(|x| Num(x))
        .map(|val| EvaluatedExpr(val))
}

fn mult(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    get_nums(args)
        .map(|v| v.into_iter().reduce(|acc, e| acc * e))?
        .ok_or(Ex::new("Could not multiply provided expression"))
        .map(|x| Num(x))
        .map(|val| EvaluatedExpr(val))
}

fn equals(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    let eval_args = ev!(args);
    // check!(EQUALS, 2, eval_args);

    let left = eval_args.get(0).unwrap();
    let right = eval_args.get(1).unwrap();

    Ok(EvaluatedExpr(Bool(left.equals(right))))
}

fn succ(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    let eval_args = get_nums(args)?;
    check!(INC, 1, eval_args);

    eval_args
        .get(0)
        .map(|x| Num(x + 1))
        .map(|val| EvaluatedExpr(val))
        .ok_or(Ex::new("Couldn't add num."))
}

fn pred(args: &[Arg], _context: &EvalContext) -> Result<Expression> {
    let eval_args = get_nums(args)?;
    // check!(DEC, 1, eval_args);

    eval_args
        .get(0)
        .map(|x| Num(x - 1))
        .map(|val| EvaluatedExpr(val))
        .ok_or(Ex::new("Couldn't subtract num.")) // err unreachable
}

fn puts(args: &[Arg], context: &EvalContext) -> Result<Expression> {
    let values = ev!(args);
    values.iter().for_each(|x| println!("{}", x.to_string()));
    unit!()
}

// ((> (puts 100)) (puts 200))
fn chain(args: &[Arg], context: &EvalContext) -> Result<Expression> {
    let args = Arg::expect_all_uneval(args)?;
    for node in args {
        evaluate_outer(context.clone(), node, false)?;
    }
    unit!()
}

// Builders
pub fn build_add() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(ADD)
        .params(Params::new_infinite(2))
        .exec(add)
        .build()
}

pub fn build_sub() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(SUB)
        .params(Params::new_infinite(2))
        .exec(sub)
        .build()
}

pub fn build_mult() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(MULT)
        .params(Params::new_infinite(2))
        .exec(mult)
        .build()
}

pub fn build_equals() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(EQUALS)
        .params(Params::new_finite(vec!["left", "right"]))
        .exec(equals)
        .build()
}

pub fn build_succ() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(INC)
        .params(Params::new_finite(vec!["x"]))
        .exec(succ)
        .build()
}

pub fn build_pred() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(DEC)
        .params(Params::new_finite(vec!["x"]))
        .exec(pred)
        .build()
}

pub fn build_puts() -> BuiltIn {
    BuiltInBuilder::new_default()
        .name(PUTS)
        .params(Params::new_infinite(1))
        .exec(puts)
        .build()
}

pub fn build_chain() -> BuiltIn {
    BuiltInBuilder::new()
        .name(CHAIN)
        .params(Params::new_infinite(1))
        .arg_type(ArgType::Unevaluated)
        .exec(chain)
        .build()
}

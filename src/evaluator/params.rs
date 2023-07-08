use crate::Arg;
use crate::message::*;

#[derive (Clone)]
pub struct FiniteParams {
    pub params:Vec<String>,
    pub params_idx:usize,
    pub received_args:Vec<Arg>
}

impl FiniteParams {
    pub fn new(params:Vec<String>)->FiniteParams {
        FiniteParams { params, params_idx: 0, received_args: vec![] }
    }

    pub fn apply(&self,args:&[Arg])->FiniteParams {
        let mut new_params=self.received_args.clone();
        new_params.extend_from_slice(args);

        FiniteParams { 
            params: self.params.clone(), 
            params_idx: self.params_idx+args.len(), 
            received_args: new_params
        }
    }

    pub fn expected_params(&self)->Vec<String> {
        self.params.iter()
            .skip(self.params_idx)
            .map(|x| x.clone())
            .collect()
    }

    pub fn num_expected_params(&self)->usize {
        self.expected_params().len()
    }
}


#[derive (Clone)]
pub struct InfiniteParams {
    pub received_args:Vec<Arg>,
    pub min:usize
}

impl InfiniteParams {
    pub fn new(min:usize)->InfiniteParams {
        InfiniteParams {
            received_args:vec![],
            min
        }
    }

    // clone - can later maintain references to previous instead to avoid n^2
    pub fn apply(&self,args:&[Arg])->InfiniteParams {
        let mut new_params=self.received_args.clone();
        new_params.extend_from_slice(args);

        InfiniteParams { received_args: new_params, min: self.min }
    }
}


pub enum Params {
    Finite(FiniteParams),
    Infinite(InfiniteParams)
}

// params: just stores Arg
    // finite: String->Arg
    // inf: Vec<Arg>
// curry: add to table/array
// resolve: return a Result<EvalContext> with the params added
    // err for not enough/too many
// use:
    // Evaluated: 

impl Params {
    pub fn new_finite(params:Vec<String>)->Params {
        Params::Finite(
            FiniteParams::new(params)
        )
    }

    pub fn new_infinite(min:usize)->Params {
        Params::Infinite(
            InfiniteParams::new(min)
        )
    }

    // partial application
    pub fn apply(&self, args:&[Arg])->Params {
        match &self {
            Params::Finite(fin) => Params::Finite(fin.apply(args)),
            Params::Infinite(inf) => Params::Infinite(inf.apply(args))
        }
    }

    // validation: that function received all params needed for execution
        // call for resolve
    // name: function name
    pub fn check(self, name:&str)->Result<Self> {
        match &self {
            Params::Finite(fin) => {
                let expected=fin.params.len();
                let actual=fin.received_args.len();

                if expected!=actual {
                    let msg=format!("'{}' expected {} arguments but received {}.", name, expected, actual);
                    err!(msg)
                } else {
                    Ok(self)
                }
            },

            Params::Infinite(inf) => {
                let actual=inf.received_args.len();
                if actual < inf.min {
                    let msg=format!("'{}' expected at least {} arguments but received {}.", name, inf.min, actual);
                    err!(msg)
                } else {
                    Ok(self)
                }
            }
        }
    }

    // expected params names for finite
    pub fn expected_params(&self)->Option<Vec<String>> {
        match &self {
            Params::Finite(finite) => {
                let exp:Vec<String>=finite.params.iter()
                .skip(finite.params_idx)
                .map(|x| x.clone())
                .collect();
                Some(exp)
            },
            Params::Infinite(_) => None
        }
    }

    pub fn received_args(&self)->&Vec<Arg> {
        match self {
            Params::Finite(fin) => &fin.received_args,
            Params::Infinite(inf) => &inf.received_args
        }
    }
}

use crate::DataValue::*;
#[test]
pub fn finite_params_test() {
    let fin=Params::new_finite(vec![String::from("a"), String::from("b")]);
    let args=[Arg::Evaluated(Num(20)), Arg::Evaluated(Num(30)), Arg::Evaluated(Num(40))];

    let fin_1=fin.apply(&args[0..1]);
    assert_eq!(vec!["b".to_string()], fin_1.expected_params().expect("Should be ok"));
    assert!(fin_1.check("fn").is_err());

    let fin_2=fin.apply(&args[0..2]);
    assert!(fin_2.expected_params().expect("Should be vec").is_empty());
    dbg!(fin_2.received_args().len());
    assert!(fin_2.check("fn").is_ok());

    let fin_3=fin.apply(&args[0..3]);
    assert!(fin_3.expected_params().expect("Should be vec").is_empty());
    assert!(fin_3.check("fn").is_err());

    let fin_4=fin.apply(&args[0..2]);
    let fin_4=fin_4.check("fn").expect("Should be ok");
    assert_eq!(fin_4.received_args().len(),2);

}

#[test]
pub fn inf_params_test() {
    let inf=Params::new_infinite(2);
    let args=[Arg::Evaluated(Num(20)), Arg::Evaluated(Num(30)), Arg::Evaluated(Num(40))];
    
    let inf_1=inf.apply(&args[0..1]);
    assert_eq!(inf_1.received_args().len(),1);
    assert!(inf_1.check("add").is_err());

    let inf_2=inf.apply(&args[0..2]);
    assert_eq!(inf_2.received_args().len(),2);
    assert!(inf_2.check("add").is_ok());

}
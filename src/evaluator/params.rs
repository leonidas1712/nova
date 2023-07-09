use std::cmp::Ordering;

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

    // cloning received - potential n^2
    pub fn apply(&self,args:&[Arg])->FiniteParams {
        let mut new_params=self.received_args.clone();
        new_params.extend_from_slice(args);

        FiniteParams { 
            params: self.params.clone(), 
            params_idx: self.params_idx+args.len(), 
            received_args: new_params
        }
    }

    // full params
    pub fn actual_params(&self)->&Vec<String> {
        &self.params
    }

    // calculated after taking into account idx
    pub fn expected_params(&self)->Vec<String> {
        self.params.iter()
            .skip(self.params_idx)
            .map(|x| x.clone())
            .collect()
    }

    // 0 <= diff < length: received less than expected
    // == length: received expected
    // > length: too many

    // len=2, idx=0
    pub fn params_diff(&self)->Ordering {
        self.params_idx.cmp(&self.params.len())
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


#[derive(Clone)]
pub enum Params {
    Finite(FiniteParams),
    Infinite(InfiniteParams)
}

impl Params {
    pub fn to_string(&self)->String {
        match self {
            Params::Finite(fin) => {
                let strs:Vec<String>=fin.received_args.iter().map(|x| x.to_string()).collect();
                strs.join(",")
            },
            Params::Infinite(inf) => {
                let strs:Vec<String>=inf.received_args.iter().map(|x| x.to_string()).collect();
                strs.join(",")
            }
        }
    }
}


impl Params {
    pub fn new_finite(params:Vec<&str>)->Params {
        Params::Finite(
            FiniteParams::new(params.iter().map(|x| x.to_string()).collect())
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

    pub fn get_finite(&self)->Option<&FiniteParams> {
        match self {
            Params::Finite(fin) => Some(fin),
            _ => None
        }
    }

    pub fn get_infinite(&self)->Option<&InfiniteParams> {
        match self {
            Params::Infinite(inf) => Some(inf),
            _ => None
        }
    }
    // consume so we can use it
    pub fn received_args(self)->Vec<Arg> {
        match self {
            Params::Finite(fin) => fin.received_args,
            Params::Infinite(inf) => inf.received_args
        }
    }
}

use crate::DataValue::*;

use super::data_tco::NumParams;
#[test]
pub fn finite_params_test() {
    let fin=Params::new_finite(vec!["a", "b"]);
    let args=[Arg::Evaluated(Num(20)), Arg::Evaluated(Num(30)), Arg::Evaluated(Num(40)), Arg::Evaluated(Num(50))];

    let mut fin_get=fin.apply(&args[0..2]);
    let fin_get=fin_get.received_args();

    let mut fin_get=fin_get.into_iter();
    let x=fin_get.next();
    let x=x.unwrap().expect_eval().expect("Should eval").expect_num().expect("Should num");
    
    assert_eq!(x, 20);

    let fin_5=fin.apply(&args[..]).apply(&args[..]);

    if let Params::Finite(fin) = fin_5 {
        println!("Params len:{}", fin.params.len());
        println!("Idx: {}", fin.params_idx);
    }
  

}

#[test]
pub fn inf_params_test() {
    let inf=Params::new_infinite(2);
    let args=[Arg::Evaluated(Num(20)), Arg::Evaluated(Num(30)), Arg::Evaluated(Num(40))];
    
    let inf_1=inf.apply(&args[0..1]);
    assert_eq!(inf_1.clone().received_args().len(),1);

    let inf_2=inf.apply(&args[0..2]);
    assert_eq!(inf_2.clone().received_args().len(),2);
}
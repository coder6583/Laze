use super::exp::{Exp, ExpList};

pub type ExpSuffixList = Vec<ExpSuffix>;
pub type ExpSuffix = Box<ExpSuffix_>;

#[derive(Clone, Debug)]
pub struct ExpSuffix_ {
    pub pos: (usize, usize),
    pub data: SuffixData,
}

#[derive(Clone, Debug)]
pub enum SuffixData {
    Call(ExpList),
    Dot(Exp),
    Arrow(Exp),
    Subscript(Exp),
}

impl ExpSuffix_ {
    pub fn call_suffix(pos: (usize, usize), explist: ExpList) -> ExpSuffix {
        Box::new(ExpSuffix_ {
            pos,
            data: SuffixData::Call(explist),
        })
    }
    pub fn dot_suffix(pos: (usize, usize), field: Exp) -> ExpSuffix {
        Box::new(ExpSuffix_ {
            pos,
            data: SuffixData::Dot(field),
        })
    }
    pub fn arrow_suffix(pos: (usize, usize), field: Exp) -> ExpSuffix {
        Box::new(ExpSuffix_ {
            pos,
            data: SuffixData::Arrow(field),
        })
    }
    pub fn subscript_suffix(pos: (usize, usize), index: Exp) -> ExpSuffix {
        Box::new(ExpSuffix_ {
            pos,
            data: SuffixData::Subscript(index),
        })
    }
}

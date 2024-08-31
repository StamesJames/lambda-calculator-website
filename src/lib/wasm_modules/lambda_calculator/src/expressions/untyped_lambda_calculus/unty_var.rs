use crate::expressions::Substitution;

use super::super::CalcStepError;
use super::super::Expression;
use super::super::Sum;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
pub struct UntyVar {
    name: String,
}

impl Display for UntyVar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl UntyVar {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    pub fn new_expr<EXPR: Sum<Self>>(name: &str) -> EXPR {
        EXPR::pack(Self {
            name: name.to_string(),
        })
    }
    pub fn into_expr<EXPR: Sum<Self>>(self) -> EXPR {
        EXPR::pack(self)
    }
}

impl<EXPR> Expression<EXPR> for UntyVar {
    fn is_value(&self) -> bool {
        true
    }

    fn calc_step(&self) -> Result<EXPR, CalcStepError> {
        todo!()
    }
}

impl<EXPR> Substitution<Self, EXPR> for UntyVar
where
    EXPR: Sum<Self> + Clone,
{
    fn substitute(&self, v: &Self, e: &EXPR) -> EXPR {
        if self == v {
            e.clone()
        } else {
            self.clone().into_expr()
        }
    }
}

use crate::expressions::Substitution;

use super::super::Expression;
use super::super::Sum;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
pub struct App<EXPR> {
    pub(crate) lhs: Box<EXPR>,
    pub(crate) rhs: Box<EXPR>,
}

impl<EXPR> Display for App<EXPR>
where
    EXPR: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.lhs, self.rhs)
    }
}

pub trait IsAbs {
    fn is_abs(&self) -> bool;
}

impl<EXPR> App<EXPR>
where
    EXPR: Sum<Self>,
{
    pub fn new(lhs: EXPR, rhs: EXPR) -> Self {
        Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }
    pub fn new_expr(lhs: EXPR, rhs: EXPR) -> EXPR {
        EXPR::pack(Self {
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        })
    }
    pub fn into_expr(self) -> EXPR {
        EXPR::pack(self)
    }
}

impl<EXPR> Expression<EXPR> for App<EXPR> {
    fn is_value(&self) -> bool {
        todo!()
    }

    fn calc_step(&self) -> Result<EXPR, crate::expressions::CalcStepError> {
        todo!()
    }
}

impl<VAR, EXPR> Substitution<VAR, EXPR> for App<EXPR>
where
    EXPR: Substitution<VAR, EXPR> + Sum<Self>,
{
    fn substitute(&self, v: &VAR, e: &EXPR) -> EXPR {
        Self {
            lhs: Box::new(self.lhs.substitute(v, e)),
            rhs: Box::new(self.rhs.substitute(v, e)),
        }
        .into_expr()
    }
}

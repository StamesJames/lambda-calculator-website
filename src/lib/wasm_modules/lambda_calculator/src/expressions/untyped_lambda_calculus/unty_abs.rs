use crate::expressions::Substitution;

use super::super::Expression;
use super::super::Sum;
use super::UntyVar;
use std::fmt::Display;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
pub struct UntyAbs<EXPR> {
    pub(crate) var: UntyVar,
    pub(crate) expr: Box<EXPR>,
}

impl<EXPR> Display for UntyAbs<EXPR>
where
    EXPR: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "λ {}. {}", self.var, self.expr)
    }
}

impl<EXPR> UntyAbs<EXPR> {
    pub fn new(var: UntyVar, expr: EXPR) -> Self {
        Self {
            var,
            expr: Box::new(expr),
        }
    }
    pub fn new_expr<SUM: Sum<Self>>(var: UntyVar, expr: EXPR) -> SUM {
        SUM::pack(Self {
            var,
            expr: Box::new(expr),
        })
    }
    pub fn into_expr<SUM: Sum<Self>>(self) -> SUM {
        SUM::pack(self)
    }
}

impl<EXPR> Expression<EXPR> for UntyAbs<EXPR> {
    fn is_value(&self) -> bool {
        todo!()
    }

    fn calc_step(&self) -> Result<EXPR, crate::expressions::CalcStepError> {
        todo!()
    }
}

impl<EXPR> Substitution<UntyVar, EXPR> for UntyAbs<EXPR>
where
    EXPR: Substitution<UntyVar, EXPR> + Sum<Self> + Clone,
{
    fn substitute(&self, v: &UntyVar, e: &EXPR) -> EXPR {
        if self.var == *v {
            self.clone().into_expr()
        } else {
            Self {
                var: v.clone(),
                expr: Box::new(self.expr.substitute(v, e)),
            }
            .into_expr()
        }
    }
}

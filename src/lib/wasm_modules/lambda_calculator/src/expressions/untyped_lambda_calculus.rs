use self::{
    app::{App, IsAbs},
    unty_abs::UntyAbs,
    unty_var::UntyVar,
};
use super::{Expression, Substitution, Sum};
use std::fmt::{Debug, Display};
pub mod app;
pub mod unty_abs;
pub mod unty_var;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash, Clone)]
pub enum UntyLamExpr {
    Var(UntyVar),
    Abs(UntyAbs<Self>),
    App(App<Self>),
}

impl Sum<UntyVar> for UntyLamExpr {
    fn pack(content: UntyVar) -> Self {
        Self::Var(content)
    }
}
impl Sum<UntyAbs<Self>> for UntyLamExpr {
    fn pack(content: UntyAbs<Self>) -> Self {
        Self::Abs(content)
    }
}
impl Sum<App<Self>> for UntyLamExpr {
    fn pack(content: App<Self>) -> Self {
        Self::App(content)
    }
}
impl Expression<Self> for UntyLamExpr {
    fn is_value(&self) -> bool {
        match self {
            UntyLamExpr::Var(e) => <UntyVar as Expression<Self>>::is_value(e),
            UntyLamExpr::Abs(e) => e.is_value(),
            UntyLamExpr::App(e) => e.is_value(),
        }
    }

    fn calc_step(&self) -> Result<Self, super::CalcStepError> {
        match self {
            UntyLamExpr::Var(e) => e.calc_step(),
            UntyLamExpr::Abs(e) => e.calc_step(),
            UntyLamExpr::App(e) => e.calc_step(),
        }
    }
}
impl Substitution<UntyVar, Self> for UntyLamExpr {
    fn substitute(&self, v: &UntyVar, e: &Self) -> Self {
        match self {
            UntyLamExpr::Var(exp) => exp.substitute(v, e),
            UntyLamExpr::Abs(exp) => exp.substitute(v, e),
            UntyLamExpr::App(exp) => exp.substitute(v, e),
        }
    }
}

impl IsAbs for UntyLamExpr {
    fn is_abs(&self) -> bool {
        matches!(self, Self::Abs(_))
    }
}

impl Display for UntyLamExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UntyLamExpr::Var(e) => Display::fmt(e, f),
            UntyLamExpr::Abs(e) => Display::fmt(e, f),
            UntyLamExpr::App(e) => Display::fmt(e, f),
        }
    }
}

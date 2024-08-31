pub mod untyped_lambda_calculus;

pub enum CalcStepError {
    NoRedex,
}

pub trait Sum<TERM> {
    fn pack(content: TERM) -> Self;
}
pub trait Expression<EXPR> {
    fn is_value(&self) -> bool;
    fn calc_step(&self) -> Result<EXPR, CalcStepError>;
}
pub trait Substitution<VAR, EXPR> {
    fn substitute(&self, v: &VAR, e: &EXPR) -> EXPR;
}

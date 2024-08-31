use crate::expressions::untyped_lambda_calculus::{
    app::App, unty_abs::UntyAbs, unty_var::UntyVar, UntyLamExpr,
};
use std::iter;
peg::parser! {
    grammar untyped_lambda_calculus_parser() for str {
        pub rule expression() -> UntyLamExpr
            = lam_abs_expr() / lam_app_expr() / lam_var_expr() / "(" ws()* e:expression() ws()* ")" {e}
        rule lam_abs_expr() -> UntyLamExpr
            = lambda() ws()* vs:(var() ++ (ws()+)) ws()* "." ws()* e:expression() {
                let first = vs.last().unwrap().clone();
                vs.into_iter().rev().skip(1).fold(UntyAbs::new_expr(first, e), |acc,v| UntyAbs::new_expr(v, acc))
            }
        rule lam_app_expr() -> UntyLamExpr
            = lhs:lam_app_opp() ws()+ rhs:(lam_app_opp() ++ (ws()+)) {
                iter::once(lhs).chain(rhs.into_iter()).reduce(App::new_expr).unwrap()
            }
        rule lam_var_expr() -> UntyLamExpr
            = v:var() {v.into_expr()}
        rule lam_app_opp() -> UntyLamExpr
            = "(" ws()* e:expression() ws()* ")" {e}  / lam_var()
        rule lam_var() -> UntyLamExpr
            = v:var() {v.into_expr()}
        rule var() -> UntyVar
            = v:$([ 'a'..='z' | 'A'..='Z']['a'..='z' | 'A'..='Z' | '0'..='9' ]*) {UntyVar::new(v)}
        rule lambda()
            = "λ"/ "\\" / "lam" ws()+ / "lambda" ws()+ / "fun" ws()+
        rule ws() = quiet!{[' ' | '\n' | '\t']+}

    }
}

pub struct UntypedLambdaCalculusParser;
impl UntypedLambdaCalculusParser {
    pub fn parse(input: &str) -> Result<UntyLamExpr, peg::error::ParseError<peg::str::LineCol>> {
        untyped_lambda_calculus_parser::expression(input)
    }
}

#[cfg(test)]
mod tests {
    use super::UntypedLambdaCalculusParser;
    use crate::expressions::untyped_lambda_calculus::{
        app::App, unty_abs::UntyAbs, unty_var::UntyVar,
    };

    #[test]
    fn combined_expressions() {
        let expr = UntypedLambdaCalculusParser::parse("λx. x x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                App::new_expr(UntyVar::new_expr("x"), UntyVar::new_expr("x"))
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx. x) x").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x")),
                UntyVar::new_expr("x")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx. x) (λy z. z z y)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x")),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(
                        UntyVar::new("z"),
                        App::new_expr(
                            App::new_expr(UntyVar::new_expr("z"), UntyVar::new_expr("z")),
                            UntyVar::new_expr("y")
                        )
                    )
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx. x) (λy z. z z y) (λ f. f) p").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(
                        UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x")),
                        UntyAbs::new_expr(
                            UntyVar::new("y"),
                            UntyAbs::new_expr(
                                UntyVar::new("z"),
                                App::new_expr(
                                    App::new_expr(UntyVar::new_expr("z"), UntyVar::new_expr("z")),
                                    UntyVar::new_expr("y")
                                )
                            )
                        )
                    ),
                    UntyAbs::new_expr(UntyVar::new("f"), UntyVar::new_expr("f"))
                ),
                UntyVar::new_expr("p")
            )
        );
    }

    #[test]
    fn lam_abs() {
        let expr = UntypedLambdaCalculusParser::parse("λx.x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("\\x.x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("lam x.x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("lambda x.x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.x)").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("((λx.x))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("((  λ  x .   x  ))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λx.x").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("x"))
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λy.y").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(UntyVar::new("y"), UntyVar::new_expr("y"))
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λx.y").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(UntyVar::new("x"), UntyVar::new_expr("y"))
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx y z.y").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λy.λz.y").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.λy.λz.y)").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.(λy.λz.y)").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λy.(λz.y)").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λy.λz.(y)").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.λy.(λz.(y))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("λx.(λy.λz.(y))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.λy.λz.(y))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.λy.(λz.y))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.(λy.λz.y))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.(λy.(λz.y)))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(λx.(λy.(λz.(y))))").unwrap();
        assert_eq!(
            expr,
            UntyAbs::new_expr(
                UntyVar::new("x"),
                UntyAbs::new_expr(
                    UntyVar::new("y"),
                    UntyAbs::new_expr(UntyVar::new("z"), UntyVar::new_expr("y"))
                )
            )
        );
    }

    #[test]
    fn lam_apps() {
        let expr = UntypedLambdaCalculusParser::parse("a b").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
        let expr = UntypedLambdaCalculusParser::parse("((a) b)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
        let expr = UntypedLambdaCalculusParser::parse("((a b))").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
        let expr = UntypedLambdaCalculusParser::parse("(((a b)))").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
        let expr = UntypedLambdaCalculusParser::parse("a b c").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                UntyVar::new_expr("c")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b) c").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                UntyVar::new_expr("c")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("a (b c)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                UntyVar::new_expr("a"),
                App::new_expr(UntyVar::new_expr("b"), UntyVar::new_expr("c")),
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("a b c d").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b) c d").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b c) d").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b c d)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("((a b) c) d").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("((a b) c d)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("((a b c) d)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(((a b) c) d)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(
                App::new_expr(
                    App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b")),
                    UntyVar::new_expr("c")
                ),
                UntyVar::new_expr("d")
            )
        );
        let expr = UntypedLambdaCalculusParser::parse("(a b)").unwrap();
        assert_eq!(
            expr,
            App::new_expr(UntyVar::new_expr("a"), UntyVar::new_expr("b"))
        );
    }

    #[test]
    fn simple_lam_vars() {
        let expr = UntypedLambdaCalculusParser::parse("a").unwrap();
        assert_eq!(expr, UntyVar::new_expr("a"));
        let expr = UntypedLambdaCalculusParser::parse("b").unwrap();
        assert_eq!(expr, UntyVar::new_expr("b"));
        let expr = UntypedLambdaCalculusParser::parse("c").unwrap();
        assert_eq!(expr, UntyVar::new_expr("c"));
        let expr = UntypedLambdaCalculusParser::parse("A").unwrap();
        assert_eq!(expr, UntyVar::new_expr("A"));
        let expr = UntypedLambdaCalculusParser::parse("B").unwrap();
        assert_eq!(expr, UntyVar::new_expr("B"));
        let expr = UntypedLambdaCalculusParser::parse("C").unwrap();
        assert_eq!(expr, UntyVar::new_expr("C"));
        let expr = UntypedLambdaCalculusParser::parse("a1").unwrap();
        assert_eq!(expr, UntyVar::new_expr("a1"));
        let expr = UntypedLambdaCalculusParser::parse("A1").unwrap();
        assert_eq!(expr, UntyVar::new_expr("A1"));
        let expr = UntypedLambdaCalculusParser::parse("a1b22CC33").unwrap();
        assert_eq!(expr, UntyVar::new_expr("a1b22CC33"));
        let expr = UntypedLambdaCalculusParser::parse("1");
        assert!(expr.is_err());
        let expr = UntypedLambdaCalculusParser::parse("1abc");
        assert!(expr.is_err());
        let expr = UntypedLambdaCalculusParser::parse("_abc");
        assert!(expr.is_err());
        let expr = UntypedLambdaCalculusParser::parse("abc_abc");
        assert!(expr.is_err());
    }
}

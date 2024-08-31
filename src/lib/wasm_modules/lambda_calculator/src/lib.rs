use expressions::untyped_lambda_calculus::UntyLamExpr;
use expressions::Expression;
use parsers::untyped_lambda_calculus::UntypedLambdaCalculusParser;
use wasm_bindgen::prelude::*;

pub mod expressions;
pub mod parsers;

#[wasm_bindgen]
#[derive(Debug)]
pub struct WasmInterface {
    expr_history: Vec<UntyLamExpr>,
    current_expr_index: usize,
}

#[wasm_bindgen]
impl WasmInterface {
    pub(crate) fn new() -> Self {
        Self {
            expr_history: vec![],
            current_expr_index: 0,
        }
    }

    fn get_current_expr(&self) -> Option<&UntyLamExpr> {
        self.expr_history.get(self.current_expr_index)
    }

    fn add_current_expr(&mut self, e: UntyLamExpr) {
        match self.expr_history.len().cmp(&(self.current_expr_index + 1)) {
            std::cmp::Ordering::Equal => {
                self.expr_history.push(e);
                self.current_expr_index += 1;
            }
            std::cmp::Ordering::Less => {
                self.expr_history.push(e);
                self.current_expr_index = self.expr_history.len() - 1
            }
            std::cmp::Ordering::Greater => {
                self.current_expr_index += 1;
                self.expr_history[self.current_expr_index] = e;
                self.expr_history.truncate(self.current_expr_index + 1);
            }
        }
    }

    fn add_step_expr(&mut self) -> Option<&UntyLamExpr> {
        self.get_current_expr()
            .and_then(|e| e.calc_step().ok())
            .and_then(|e| {
                self.add_current_expr(e);
                self.get_current_expr()
            })
    }

    fn undo(&mut self) -> Option<&UntyLamExpr> {
        if self.current_expr_index > 0 {
            self.current_expr_index -= 1;
            self.get_current_expr()
        } else {
            None
        }
    }

    fn redo(&mut self) -> Option<&UntyLamExpr> {
        if self.current_expr_index + 2 < self.expr_history.len() {
            self.current_expr_index += 1;
            self.get_current_expr()
        } else {
            None
        }
    }

    // Pub wasm interface
    pub fn add_step_exp_get_string(&mut self) -> Option<String> {
        self.add_step_expr().map(|e| e.to_string())
    }
    pub fn get_current_expr_string(&self) -> Option<String> {
        self.get_current_expr().map(|e| e.to_string())
    }
    pub fn add_current_expr_string(&mut self, s: &str) -> Option<String> {
        let parse_result = UntypedLambdaCalculusParser::parse(s);
        match parse_result {
            Ok(e) => {
                let s = e.to_string();
                self.add_current_expr(e);
                Some(s)
            }
            Err(_) => None,
        }
    }
}

#[wasm_bindgen]
pub fn init() -> WasmInterface {
    WasmInterface::new()
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

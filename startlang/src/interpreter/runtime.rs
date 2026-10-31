use ir::{lir, tir};

/// `tir` → `lir` (`codegen`) → run (`vm`); the state of both is kept between
/// commands
#[derive(Debug, Default)]
pub struct Runtime {
    codegen: driver::codegen::Codegen,
    vm: driver::vm::Env,
}

impl Runtime {
    /// generate a definition and compute its value
    ///
    /// # Panics
    /// if the definition cannot be generated or run: `typing` accepted it, so
    /// this is a bug of the compiler
    pub fn define(&mut self, def: &tir::ExpressionDefinition) -> lir::Global {
        let global = self
            .codegen
            .definition(def)
            .unwrap_or_else(|err| panic!("{err}"));
        self.vm
            .define(&global)
            .unwrap_or_else(|| panic!("vm: undefined global in {}", global.symbol));
        global
    }

    /// generate an expression and compute its value
    ///
    /// # Panics
    /// same as `define`
    pub fn eval(&self, expr: &tir::Expression) -> (lir::Expression, driver::vm::Value) {
        let expr = self
            .codegen
            .expression(expr)
            .unwrap_or_else(|err| panic!("{err}"));
        let value = self
            .vm
            .eval(&expr)
            .unwrap_or_else(|| panic!("vm: undefined global"));
        (expr, value)
    }
}

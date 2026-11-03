use crate::vm::{Value, error};
use crate::{Checked, CheckedResult, Partial};
use errors::Message;
use ir::lir::{Expression, File, Global, GlobalId, Item};

/// runs the files one after the other: the values of the globals of the
/// previous files (stdlib, REPL inputs) are kept, indexed by `GlobalId`
#[derive(Debug, Default, Clone)]
pub struct Vm {
    globals: Vec<Option<Value>>,
}

impl Vm {
    pub fn new() -> Self {
        Self::default()
    }

    /// run every item of `file`: the value of each `Eval` it could compute,
    /// in order, and every (internal) error
    pub fn run(&mut self, file: &Checked<File>) -> CheckedResult<Vec<Value>> {
        let not_computed = |err: errors::Error| {
            Partial::ok(None)
                .with_error(err.with_recovery(Message::text("the value is not computed")))
        };
        let values: Partial<Vec<_>> = file
            .items
            .iter()
            .map(|item| match item {
                Item::Global(global) => match self.define(global) {
                    Some(_) => Partial::ok(None),
                    None => not_computed(error::undefined(&global.symbol)),
                },
                Item::Eval(expr) => match self.eval(expr) {
                    Some(value) => Partial::ok(Some(value)),
                    None => not_computed(error::undefined("eval")),
                },
            })
            .collect();
        values
            .map(|values| values.into_iter().flatten().collect())
            .check()
    }

    /// value of a global, `None` if it was never defined
    pub fn get(&self, id: GlobalId) -> Option<&Value> {
        self.globals.get(id.0).and_then(Option::as_ref)
    }

    /// evaluate an expression; `None` if it reads an undefined global
    pub fn eval(&self, expr: &Expression) -> Option<Value> {
        match expr {
            Expression::Constant(c) => Some(Value::from(c)),
            Expression::Global(id) => self.get(*id).cloned(),
        }
    }

    /// compute the value of a global and store it; `None` if its initial
    /// value reads an undefined global
    pub fn define(&mut self, global: &Global) -> Option<&Value> {
        let value = self.eval(&global.init)?;
        let index = global.id.0;
        if self.globals.len() <= index {
            self.globals.resize(index + 1, None);
        }
        self.globals[index] = Some(value);
        self.globals[index].as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::lir::{Constant, Type};

    fn global(index: usize, ty: Type, init: Expression) -> Global {
        Global {
            id: GlobalId(index),
            symbol: format!("g{index}"),
            ty,
            init,
        }
    }

    #[test]
    fn define_and_eval_globals() {
        let mut vm = Vm::default();
        let a = global(
            0,
            Type::Nat,
            Expression::Constant(Constant::Nat(3u8.into())),
        );
        let b = global(1, Type::Nat, Expression::Global(a.id));

        assert_eq!(vm.define(&a), Some(&Value::Nat(3u8.into())));
        assert_eq!(vm.define(&b), Some(&Value::Nat(3u8.into())));
        assert_eq!(
            vm.eval(&Expression::Global(b.id)),
            Some(Value::Nat(3u8.into()))
        );
        assert_eq!(
            vm.eval(&Expression::Constant(Constant::Char('x'))),
            Some(Value::Char('x'))
        );
    }

    #[test]
    fn undefined_global() {
        let mut vm = Vm::default();
        let b = global(1, Type::Bool, Expression::Global(GlobalId(0)));
        assert_eq!(vm.define(&b), None);
        assert_eq!(vm.eval(&Expression::Global(GlobalId(5))), None);
    }
}

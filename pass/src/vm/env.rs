use crate::Partial;
use crate::vm::{Value, error};
use errors::Message;
use ir::lir::{Expression, File, Global, GlobalId, Item};

/// runs the files one after the other: the values of the globals of the
/// previous files (stdlib, REPL inputs) are kept, indexed by `GlobalId`
#[derive(Debug, Default)]
pub struct Vm {
    globals: Vec<Option<Value>>,
}

impl Vm {
    pub fn builder() -> VmBuilder {
        VmBuilder
    }

    /// run every item of `file`: the value of each `Eval` it could compute,
    /// in order, and every (internal) error
    pub fn run(&mut self, file: &File) -> Partial<Vec<Value>> {
        let mut values = Vec::new();
        let mut errors = Vec::new();
        for item in file.items() {
            match item {
                Item::Global(global) => {
                    if self.define(global).is_none() {
                        errors.push(error::undefined(&global.symbol));
                    }
                }
                Item::Eval(expr) => match self.eval(expr) {
                    Some(value) => values.push(value),
                    None => errors.push(error::undefined("eval")),
                },
            }
        }
        let recovery = || Message::text("the value is not computed.");
        Partial::ok(values).with_errors(errors.into_iter().map(|err| err.with_recovery(recovery())))
    }

    /// value of a global, `None` if it was never defined
    pub fn get(&self, id: GlobalId) -> Option<&Value> {
        self.globals.get(id.index()).and_then(Option::as_ref)
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
        let index = global.id.index();
        if self.globals.len() <= index {
            self.globals.resize(index + 1, None);
        }
        self.globals[index] = Some(value);
        self.globals[index].as_ref()
    }
}

/// arguments of a `Vm` (none yet)
#[derive(Debug, Default, Clone, Copy)]
pub struct VmBuilder;

impl VmBuilder {
    pub fn build(self) -> Vm {
        Vm::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::lir::{Constant, Type};

    fn global(index: usize, ty: Type, init: Expression) -> Global {
        Global {
            id: GlobalId::new(index),
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
        let b = global(1, Type::Bool, Expression::Global(GlobalId::new(0)));
        assert_eq!(vm.define(&b), None);
        assert_eq!(vm.eval(&Expression::Global(GlobalId::new(5))), None);
    }
}

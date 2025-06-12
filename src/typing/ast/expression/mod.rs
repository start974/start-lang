mod constant;
pub use constant::Constant;

mod expression;
pub use expression::Expression;
pub use expression::sealed_mut_ty as sealed_mut_ty_expr;

mod definition;
pub use definition::Definition as ExpressionDefinition;

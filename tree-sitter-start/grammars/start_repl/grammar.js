module.exports = grammar(require("../start/grammar"), {
  name: 'start_repl',
  rules: {
    definitions_or_expression: $ => choice(
      $.expression,
      $.definitions,
    ),

    definitions : $ => repeat1(
      $._definition),

    expression : $ => $._expr_final,

  }
})

// Make 'definitions_or_expression' the first rule
module.exports.grammar.rules = Object.assign(
  {definitions_or_expression: null},
  module.exports.grammar.rules
)

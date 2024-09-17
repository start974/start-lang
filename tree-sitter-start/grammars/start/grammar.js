const NUMBER = token(choice(
  /[0-9][0-9_]*/,
  /0[xX][0-9A-Fa-f][0-9A-Fa-f_]*/,
  /0[oO][0-7][0-7_]*/,
  /0[bB][01][01_]*/
))
//TODO: [0-9][0-9_]*([eE][0-9_]+)?/, (add exponent)

const IDENT = token(
  /[a-zA-Z_][a-zA-Z0-9_]*[']*/
)

module.exports = grammar({
  name: 'start',

  extras: $ => [
    /\s/,
  ],

  word: $ => $.ident,

  rules: {
    program: $ =>
      repeat($._definition),

    _definition: $ => choice(
      $.expr_def,
      // TODO: add type def
    ),

    expr_def: $ =>seq("def",
      $.ident,
      optional($.ty_restr),
      ":=",
      $._expr_final
    ),

    ty_restr: $ =>
      seq(":", $._ty),

    _ty: $ => choice(
      $.ident
      // TODO: other types
    ),

    _expr_final: $ =>
      seq($._expr, optional(".")),

    _expr: $ => choice(
      seq ("(", $._expr , ")"),
      $.ident,
      $.constant
    ),

    constant: $ => choice(
      $._number
    ),

    _number: $ => choice(
      $.number_N,
    ),

    number_N : $ =>
      NUMBER
    ,

    ident : $ =>
      IDENT
    ,
  }
});

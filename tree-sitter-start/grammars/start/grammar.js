module.exports = grammar({
  name: 'start',
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
      $.constant
      // TODO: other expression
    ),

    constant: $ => choice(
      $._number
    ),

    _number: $ => choice(
      $.number_N,
    ),

    number_N : $ =>
      /[0-9]+/,
    // TODO: add bin / oct / hex

    ident : $ => /[a-zA-Z_][a-zA-Z0-9_]*/
      // TODO: add unicode chars add finish by '
    ,
  }
});

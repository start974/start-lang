module.exports = grammar({
  name: 'start',

  rules: {
    program: $ =>
      repeat($._definition),

    _definition: $ => choice(
      $.expr_def,
      // TODO: add type def
    ),

    expr_def: $ =>seq("def",
      field("name", $.ident),
      field("type", optional($.ty_restr)),
      ":=",
      field("body", $._expr)
    ),

    ty_restr: $ =>
      seq(":", $._ty),

    _ty: $ => choice(
      $.ident
      // TODO: other types
    ),

    _expr: $ => choice(
      seq ("(", $._expr , ")"),
      $._const
      // TODO: other expression
    ),

    _const: $ => choice(
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

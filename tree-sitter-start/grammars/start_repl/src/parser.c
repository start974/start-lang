#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 26
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 23
#define ALIAS_COUNT 0
#define TOKEN_COUNT 10
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_ident = 1,
  anon_sym_def = 2,
  anon_sym_COLON_EQ = 3,
  anon_sym_type = 4,
  anon_sym_COLON = 5,
  anon_sym_DOT = 6,
  anon_sym_LPAREN = 7,
  anon_sym_RPAREN = 8,
  sym_number_N = 9,
  sym_definitions_or_expression = 10,
  sym__definition = 11,
  sym_expr_def = 12,
  sym_type_def = 13,
  sym_ty_restr = 14,
  sym__ty = 15,
  sym__expr_final = 16,
  sym__expr = 17,
  sym_constant = 18,
  sym__number = 19,
  sym_definitions = 20,
  sym_expression = 21,
  aux_sym_program_repeat1 = 22,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_ident] = "ident",
  [anon_sym_def] = "def",
  [anon_sym_COLON_EQ] = ":=",
  [anon_sym_type] = "type",
  [anon_sym_COLON] = ":",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_number_N] = "number_N",
  [sym_definitions_or_expression] = "definitions_or_expression",
  [sym__definition] = "_definition",
  [sym_expr_def] = "expr_def",
  [sym_type_def] = "type_def",
  [sym_ty_restr] = "ty_restr",
  [sym__ty] = "_ty",
  [sym__expr_final] = "_expr_final",
  [sym__expr] = "_expr",
  [sym_constant] = "constant",
  [sym__number] = "_number",
  [sym_definitions] = "definitions",
  [sym_expression] = "expression",
  [aux_sym_program_repeat1] = "program_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_ident] = sym_ident,
  [anon_sym_def] = anon_sym_def,
  [anon_sym_COLON_EQ] = anon_sym_COLON_EQ,
  [anon_sym_type] = anon_sym_type,
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_number_N] = sym_number_N,
  [sym_definitions_or_expression] = sym_definitions_or_expression,
  [sym__definition] = sym__definition,
  [sym_expr_def] = sym_expr_def,
  [sym_type_def] = sym_type_def,
  [sym_ty_restr] = sym_ty_restr,
  [sym__ty] = sym__ty,
  [sym__expr_final] = sym__expr_final,
  [sym__expr] = sym__expr,
  [sym_constant] = sym_constant,
  [sym__number] = sym__number,
  [sym_definitions] = sym_definitions,
  [sym_expression] = sym_expression,
  [aux_sym_program_repeat1] = aux_sym_program_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_ident] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_def] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON_EQ] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_type] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DOT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LPAREN] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RPAREN] = {
    .visible = true,
    .named = false,
  },
  [sym_number_N] = {
    .visible = true,
    .named = true,
  },
  [sym_definitions_or_expression] = {
    .visible = true,
    .named = true,
  },
  [sym__definition] = {
    .visible = false,
    .named = true,
  },
  [sym_expr_def] = {
    .visible = true,
    .named = true,
  },
  [sym_type_def] = {
    .visible = true,
    .named = true,
  },
  [sym_ty_restr] = {
    .visible = true,
    .named = true,
  },
  [sym__ty] = {
    .visible = false,
    .named = true,
  },
  [sym__expr_final] = {
    .visible = false,
    .named = true,
  },
  [sym__expr] = {
    .visible = false,
    .named = true,
  },
  [sym_constant] = {
    .visible = true,
    .named = true,
  },
  [sym__number] = {
    .visible = false,
    .named = true,
  },
  [sym_definitions] = {
    .visible = true,
    .named = true,
  },
  [sym_expression] = {
    .visible = true,
    .named = true,
  },
  [aux_sym_program_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 5,
  [6] = 6,
  [7] = 7,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(4);
      if (lookahead == '(') ADVANCE(8);
      if (lookahead == ')') ADVANCE(9);
      if (lookahead == '.') ADVANCE(7);
      if (lookahead == '0') ADVANCE(10);
      if (lookahead == ':') ADVANCE(6);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(13);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 1:
      if (lookahead == '0' ||
          lookahead == '1') ADVANCE(11);
      END_STATE();
    case 2:
      if (('0' <= lookahead && lookahead <= '7')) ADVANCE(12);
      END_STATE();
    case 3:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_COLON_EQ);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_COLON);
      if (lookahead == '=') ADVANCE(5);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_DOT);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_LPAREN);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_RPAREN);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(sym_number_N);
      if (lookahead == 'B' ||
          lookahead == 'b') ADVANCE(1);
      if (lookahead == 'O' ||
          lookahead == 'o') ADVANCE(2);
      if (lookahead == 'X' ||
          lookahead == 'x') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(13);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_number_N);
      if (lookahead == '0' ||
          lookahead == '1' ||
          lookahead == '_') ADVANCE(11);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_number_N);
      if (('0' <= lookahead && lookahead <= '7') ||
          lookahead == '_') ADVANCE(12);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_number_N);
      if (('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(13);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_number_N);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == '\'') ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_ident);
      if (lookahead == '\'') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (lookahead == 'd') ADVANCE(1);
      if (lookahead == 't') ADVANCE(2);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'e') ADVANCE(3);
      END_STATE();
    case 2:
      if (lookahead == 'y') ADVANCE(4);
      END_STATE();
    case 3:
      if (lookahead == 'f') ADVANCE(5);
      END_STATE();
    case 4:
      if (lookahead == 'p') ADVANCE(6);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_def);
      END_STATE();
    case 6:
      if (lookahead == 'e') ADVANCE(7);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_type);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_COLON_EQ] = ACTIONS(1),
    [anon_sym_type] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_number_N] = ACTIONS(1),
  },
  [1] = {
    [sym_definitions_or_expression] = STATE(24),
    [sym__definition] = STATE(3),
    [sym_expr_def] = STATE(3),
    [sym_type_def] = STATE(3),
    [sym__expr_final] = STATE(23),
    [sym__expr] = STATE(9),
    [sym_constant] = STATE(9),
    [sym__number] = STATE(7),
    [sym_definitions] = STATE(21),
    [sym_expression] = STATE(21),
    [aux_sym_program_repeat1] = STATE(3),
    [sym_ident] = ACTIONS(3),
    [anon_sym_def] = ACTIONS(5),
    [anon_sym_type] = ACTIONS(7),
    [anon_sym_LPAREN] = ACTIONS(9),
    [sym_number_N] = ACTIONS(11),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(9), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      sym_number_N,
    ACTIONS(13), 1,
      sym_ident,
    STATE(7), 1,
      sym__number,
    STATE(10), 1,
      sym__expr_final,
    STATE(9), 2,
      sym__expr,
      sym_constant,
  [20] = 4,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
    ACTIONS(17), 1,
      anon_sym_def,
    ACTIONS(19), 1,
      anon_sym_type,
    STATE(4), 4,
      sym__definition,
      sym_expr_def,
      sym_type_def,
      aux_sym_program_repeat1,
  [36] = 4,
    ACTIONS(21), 1,
      ts_builtin_sym_end,
    ACTIONS(23), 1,
      anon_sym_def,
    ACTIONS(26), 1,
      anon_sym_type,
    STATE(4), 4,
      sym__definition,
      sym_expr_def,
      sym_type_def,
      aux_sym_program_repeat1,
  [52] = 6,
    ACTIONS(9), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      sym_number_N,
    ACTIONS(13), 1,
      sym_ident,
    STATE(7), 1,
      sym__number,
    STATE(12), 1,
      sym__expr_final,
    STATE(9), 2,
      sym__expr,
      sym_constant,
  [72] = 5,
    ACTIONS(9), 1,
      anon_sym_LPAREN,
    ACTIONS(11), 1,
      sym_number_N,
    ACTIONS(29), 1,
      sym_ident,
    STATE(7), 1,
      sym__number,
    STATE(20), 2,
      sym__expr,
      sym_constant,
  [89] = 1,
    ACTIONS(31), 5,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [97] = 1,
    ACTIONS(33), 5,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [105] = 2,
    ACTIONS(37), 1,
      anon_sym_DOT,
    ACTIONS(35), 3,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
  [114] = 1,
    ACTIONS(39), 3,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
  [120] = 1,
    ACTIONS(41), 3,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
  [126] = 1,
    ACTIONS(43), 3,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
  [132] = 3,
    ACTIONS(45), 1,
      anon_sym_COLON_EQ,
    ACTIONS(47), 1,
      anon_sym_COLON,
    STATE(18), 1,
      sym_ty_restr,
  [142] = 1,
    ACTIONS(49), 3,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_type,
  [148] = 2,
    ACTIONS(51), 1,
      sym_ident,
    STATE(22), 1,
      sym__ty,
  [155] = 2,
    ACTIONS(53), 1,
      sym_ident,
    STATE(11), 1,
      sym__ty,
  [162] = 1,
    ACTIONS(55), 1,
      anon_sym_COLON_EQ,
  [166] = 1,
    ACTIONS(57), 1,
      anon_sym_COLON_EQ,
  [170] = 1,
    ACTIONS(59), 1,
      sym_ident,
  [174] = 1,
    ACTIONS(61), 1,
      anon_sym_RPAREN,
  [178] = 1,
    ACTIONS(63), 1,
      ts_builtin_sym_end,
  [182] = 1,
    ACTIONS(65), 1,
      anon_sym_COLON_EQ,
  [186] = 1,
    ACTIONS(67), 1,
      ts_builtin_sym_end,
  [190] = 1,
    ACTIONS(69), 1,
      ts_builtin_sym_end,
  [194] = 1,
    ACTIONS(71), 1,
      sym_ident,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 20,
  [SMALL_STATE(4)] = 36,
  [SMALL_STATE(5)] = 52,
  [SMALL_STATE(6)] = 72,
  [SMALL_STATE(7)] = 89,
  [SMALL_STATE(8)] = 97,
  [SMALL_STATE(9)] = 105,
  [SMALL_STATE(10)] = 114,
  [SMALL_STATE(11)] = 120,
  [SMALL_STATE(12)] = 126,
  [SMALL_STATE(13)] = 132,
  [SMALL_STATE(14)] = 142,
  [SMALL_STATE(15)] = 148,
  [SMALL_STATE(16)] = 155,
  [SMALL_STATE(17)] = 162,
  [SMALL_STATE(18)] = 166,
  [SMALL_STATE(19)] = 170,
  [SMALL_STATE(20)] = 174,
  [SMALL_STATE(21)] = 178,
  [SMALL_STATE(22)] = 182,
  [SMALL_STATE(23)] = 186,
  [SMALL_STATE(24)] = 190,
  [SMALL_STATE(25)] = 194,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(19),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(25),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definitions, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [19] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [21] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0),
  [23] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0), SHIFT_REPEAT(19),
  [26] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0), SHIFT_REPEAT(25),
  [29] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [31] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constant, 1, 0, 0),
  [33] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3, 0, 0),
  [35] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 1, 0, 0),
  [37] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [39] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 5, 0, 0),
  [41] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_type_def, 4, 0, 0),
  [43] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 4, 0, 0),
  [45] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [47] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [49] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 2, 0, 0),
  [51] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [53] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [55] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [57] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [59] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [61] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [63] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definitions_or_expression, 1, 0, 0),
  [65] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ty_restr, 2, 0, 0),
  [67] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1, 0, 0),
  [69] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [71] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_start_repl(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_ident,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif

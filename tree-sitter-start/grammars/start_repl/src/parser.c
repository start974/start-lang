#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 22
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 21
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_ident = 1,
  anon_sym_def = 2,
  anon_sym_COLON_EQ = 3,
  anon_sym_COLON = 4,
  anon_sym_DOT = 5,
  anon_sym_LPAREN = 6,
  anon_sym_RPAREN = 7,
  sym_number_N = 8,
  sym_definitions_or_expression = 9,
  sym__definition = 10,
  sym_expr_def = 11,
  sym_ty_restr = 12,
  sym__ty = 13,
  sym__expr_final = 14,
  sym__expr = 15,
  sym_constant = 16,
  sym__number = 17,
  sym_definitions = 18,
  sym_expression = 19,
  aux_sym_program_repeat1 = 20,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_ident] = "ident",
  [anon_sym_def] = "def",
  [anon_sym_COLON_EQ] = ":=",
  [anon_sym_COLON] = ":",
  [anon_sym_DOT] = ".",
  [anon_sym_LPAREN] = "(",
  [anon_sym_RPAREN] = ")",
  [sym_number_N] = "number_N",
  [sym_definitions_or_expression] = "definitions_or_expression",
  [sym__definition] = "_definition",
  [sym_expr_def] = "expr_def",
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
  [anon_sym_COLON] = anon_sym_COLON,
  [anon_sym_DOT] = anon_sym_DOT,
  [anon_sym_LPAREN] = anon_sym_LPAREN,
  [anon_sym_RPAREN] = anon_sym_RPAREN,
  [sym_number_N] = sym_number_N,
  [sym_definitions_or_expression] = sym_definitions_or_expression,
  [sym__definition] = sym__definition,
  [sym_expr_def] = sym_expr_def,
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
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      END_STATE();
    case 1:
      if (lookahead == 'e') ADVANCE(2);
      END_STATE();
    case 2:
      if (lookahead == 'f') ADVANCE(3);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(anon_sym_def);
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
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_ident] = ACTIONS(1),
    [anon_sym_def] = ACTIONS(1),
    [anon_sym_COLON_EQ] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [anon_sym_DOT] = ACTIONS(1),
    [anon_sym_LPAREN] = ACTIONS(1),
    [anon_sym_RPAREN] = ACTIONS(1),
    [sym_number_N] = ACTIONS(1),
  },
  [1] = {
    [sym_definitions_or_expression] = STATE(21),
    [sym__definition] = STATE(6),
    [sym_expr_def] = STATE(6),
    [sym__expr_final] = STATE(19),
    [sym__expr] = STATE(9),
    [sym_constant] = STATE(9),
    [sym__number] = STATE(8),
    [sym_definitions] = STATE(17),
    [sym_expression] = STATE(17),
    [aux_sym_program_repeat1] = STATE(6),
    [sym_ident] = ACTIONS(3),
    [anon_sym_def] = ACTIONS(5),
    [anon_sym_LPAREN] = ACTIONS(7),
    [sym_number_N] = ACTIONS(9),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 6,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    ACTIONS(11), 1,
      sym_ident,
    STATE(8), 1,
      sym__number,
    STATE(14), 1,
      sym__expr_final,
    STATE(9), 2,
      sym__expr,
      sym_constant,
  [20] = 6,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    ACTIONS(11), 1,
      sym_ident,
    STATE(8), 1,
      sym__number,
    STATE(13), 1,
      sym__expr_final,
    STATE(9), 2,
      sym__expr,
      sym_constant,
  [40] = 5,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    ACTIONS(13), 1,
      sym_ident,
    STATE(8), 1,
      sym__number,
    STATE(15), 2,
      sym__expr,
      sym_constant,
  [57] = 3,
    ACTIONS(15), 1,
      ts_builtin_sym_end,
    ACTIONS(17), 1,
      anon_sym_def,
    STATE(5), 3,
      sym__definition,
      sym_expr_def,
      aux_sym_program_repeat1,
  [69] = 3,
    ACTIONS(20), 1,
      ts_builtin_sym_end,
    ACTIONS(22), 1,
      anon_sym_def,
    STATE(5), 3,
      sym__definition,
      sym_expr_def,
      aux_sym_program_repeat1,
  [81] = 1,
    ACTIONS(24), 4,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [88] = 1,
    ACTIONS(26), 4,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [95] = 2,
    ACTIONS(30), 1,
      anon_sym_DOT,
    ACTIONS(28), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [103] = 3,
    ACTIONS(32), 1,
      anon_sym_COLON_EQ,
    ACTIONS(34), 1,
      anon_sym_COLON,
    STATE(18), 1,
      sym_ty_restr,
  [113] = 2,
    ACTIONS(36), 1,
      sym_ident,
    STATE(20), 1,
      sym__ty,
  [120] = 1,
    ACTIONS(38), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [125] = 1,
    ACTIONS(40), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [130] = 1,
    ACTIONS(42), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [135] = 1,
    ACTIONS(44), 1,
      anon_sym_RPAREN,
  [139] = 1,
    ACTIONS(46), 1,
      sym_ident,
  [143] = 1,
    ACTIONS(48), 1,
      ts_builtin_sym_end,
  [147] = 1,
    ACTIONS(50), 1,
      anon_sym_COLON_EQ,
  [151] = 1,
    ACTIONS(52), 1,
      ts_builtin_sym_end,
  [155] = 1,
    ACTIONS(54), 1,
      anon_sym_COLON_EQ,
  [159] = 1,
    ACTIONS(56), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 20,
  [SMALL_STATE(4)] = 40,
  [SMALL_STATE(5)] = 57,
  [SMALL_STATE(6)] = 69,
  [SMALL_STATE(7)] = 81,
  [SMALL_STATE(8)] = 88,
  [SMALL_STATE(9)] = 95,
  [SMALL_STATE(10)] = 103,
  [SMALL_STATE(11)] = 113,
  [SMALL_STATE(12)] = 120,
  [SMALL_STATE(13)] = 125,
  [SMALL_STATE(14)] = 130,
  [SMALL_STATE(15)] = 135,
  [SMALL_STATE(16)] = 139,
  [SMALL_STATE(17)] = 143,
  [SMALL_STATE(18)] = 147,
  [SMALL_STATE(19)] = 151,
  [SMALL_STATE(20)] = 155,
  [SMALL_STATE(21)] = 159,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(16),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0),
  [17] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0), SHIFT_REPEAT(16),
  [20] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definitions, 1, 0, 0),
  [22] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [24] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3, 0, 0),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constant, 1, 0, 0),
  [28] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 1, 0, 0),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [34] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 2, 0, 0),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 4, 0, 0),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 5, 0, 0),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_definitions_or_expression, 1, 0, 0),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expression, 1, 0, 0),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ty_restr, 2, 0, 0),
  [56] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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

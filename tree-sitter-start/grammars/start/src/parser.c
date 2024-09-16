#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 2
#define SYMBOL_COUNT 19
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
  sym_program = 9,
  sym__definition = 10,
  sym_expr_def = 11,
  sym_ty_restr = 12,
  sym__ty = 13,
  sym__expr_final = 14,
  sym__expr = 15,
  sym_constant = 16,
  sym__number = 17,
  aux_sym_program_repeat1 = 18,
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
  [sym_program] = "program",
  [sym__definition] = "_definition",
  [sym_expr_def] = "expr_def",
  [sym_ty_restr] = "ty_restr",
  [sym__ty] = "_ty",
  [sym__expr_final] = "_expr_final",
  [sym__expr] = "_expr",
  [sym_constant] = "constant",
  [sym__number] = "_number",
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
  [sym_program] = sym_program,
  [sym__definition] = sym__definition,
  [sym_expr_def] = sym_expr_def,
  [sym_ty_restr] = sym_ty_restr,
  [sym__ty] = sym__ty,
  [sym__expr_final] = sym__expr_final,
  [sym__expr] = sym__expr,
  [sym_constant] = sym_constant,
  [sym__number] = sym__number,
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
  [sym_program] = {
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
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
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
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
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
    [sym_program] = STATE(16),
    [sym__definition] = STATE(4),
    [sym_expr_def] = STATE(4),
    [aux_sym_program_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_def] = ACTIONS(5),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    STATE(7), 1,
      sym__number,
    STATE(12), 1,
      sym__expr_final,
    STATE(10), 2,
      sym__expr,
      sym_constant,
  [17] = 5,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    STATE(7), 1,
      sym__number,
    STATE(14), 1,
      sym__expr_final,
    STATE(10), 2,
      sym__expr,
      sym_constant,
  [34] = 3,
    ACTIONS(5), 1,
      anon_sym_def,
    ACTIONS(11), 1,
      ts_builtin_sym_end,
    STATE(5), 3,
      sym__definition,
      sym_expr_def,
      aux_sym_program_repeat1,
  [46] = 3,
    ACTIONS(13), 1,
      ts_builtin_sym_end,
    ACTIONS(15), 1,
      anon_sym_def,
    STATE(5), 3,
      sym__definition,
      sym_expr_def,
      aux_sym_program_repeat1,
  [58] = 4,
    ACTIONS(7), 1,
      anon_sym_LPAREN,
    ACTIONS(9), 1,
      sym_number_N,
    STATE(7), 1,
      sym__number,
    STATE(19), 2,
      sym__expr,
      sym_constant,
  [72] = 1,
    ACTIONS(18), 4,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [79] = 1,
    ACTIONS(20), 4,
      ts_builtin_sym_end,
      anon_sym_def,
      anon_sym_DOT,
      anon_sym_RPAREN,
  [86] = 3,
    ACTIONS(22), 1,
      anon_sym_COLON_EQ,
    ACTIONS(24), 1,
      anon_sym_COLON,
    STATE(17), 1,
      sym_ty_restr,
  [96] = 2,
    ACTIONS(28), 1,
      anon_sym_DOT,
    ACTIONS(26), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [104] = 2,
    ACTIONS(30), 1,
      sym_ident,
    STATE(18), 1,
      sym__ty,
  [111] = 1,
    ACTIONS(32), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [116] = 1,
    ACTIONS(34), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [121] = 1,
    ACTIONS(36), 2,
      ts_builtin_sym_end,
      anon_sym_def,
  [126] = 1,
    ACTIONS(38), 1,
      sym_ident,
  [130] = 1,
    ACTIONS(40), 1,
      ts_builtin_sym_end,
  [134] = 1,
    ACTIONS(42), 1,
      anon_sym_COLON_EQ,
  [138] = 1,
    ACTIONS(44), 1,
      anon_sym_COLON_EQ,
  [142] = 1,
    ACTIONS(46), 1,
      anon_sym_RPAREN,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(2)] = 0,
  [SMALL_STATE(3)] = 17,
  [SMALL_STATE(4)] = 34,
  [SMALL_STATE(5)] = 46,
  [SMALL_STATE(6)] = 58,
  [SMALL_STATE(7)] = 72,
  [SMALL_STATE(8)] = 79,
  [SMALL_STATE(9)] = 86,
  [SMALL_STATE(10)] = 96,
  [SMALL_STATE(11)] = 104,
  [SMALL_STATE(12)] = 111,
  [SMALL_STATE(13)] = 116,
  [SMALL_STATE(14)] = 121,
  [SMALL_STATE(15)] = 126,
  [SMALL_STATE(16)] = 130,
  [SMALL_STATE(17)] = 134,
  [SMALL_STATE(18)] = 138,
  [SMALL_STATE(19)] = 142,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [9] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [11] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_program, 1, 0, 0),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0),
  [15] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_program_repeat1, 2, 0, 0), SHIFT_REPEAT(15),
  [18] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_constant, 1, 0, 0),
  [20] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr, 3, 0, 0),
  [22] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [24] = {.entry = {.count = 1, .reusable = false}}, SHIFT(11),
  [26] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 1, 0, 0),
  [28] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(18),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 4, 0, 0),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym__expr_final, 2, 0, 0),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr_def, 5, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [40] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_ty_restr, 2, 0, 0),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
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

TS_PUBLIC const TSLanguage *tree_sitter_start(void) {
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

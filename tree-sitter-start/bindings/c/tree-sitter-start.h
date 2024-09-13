#ifndef TREE_SITTER_START_H_
#define TREE_SITTER_START_H_

typedef struct TSLanguage TSLanguage;

#ifdef __cplusplus
extern "C" {
#endif

const TSLanguage *tree_sitter_start_repl(void);

#ifdef __cplusplus
}
#endif

#endif // TREE_SITTER_START_H_

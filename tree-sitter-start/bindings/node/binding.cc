#include <napi.h>

typedef struct TSLanguage TSLanguage;

extern "C" TSLanguage *tree_sitter_start();
extern "C" TSLanguage *tree_sitter_start_repl();

// "tree-sitter", "language" hashed with BLAKE2
const napi_type_tag LANGUAGE_TYPE_TAG = {
  0x8AF2E5212AD58ABF, 0xD5006CAD83ABBA16
};

Napi::Object Init(Napi::Env env, Napi::Object exports) {
    auto start = Napi::Object::New(env);
    start["name"] = Napi::String::New(env, "start");
    auto start_language = Napi::External<TSLanguage>::New(env, tree_sitter_start());
    start_language.TypeTag(&LANGUAGE_TYPE_TAG);
    start["language"] = start_language;


    auto start_repl = Napi::Object::New(env);
    start_repl["name"] = Napi::String::New(env, "start");
    auto start_repl_language = Napi::External<TSLanguage>::New(env, tree_sitter_start());
    start_repl_language.TypeTag(&LANGUAGE_TYPE_TAG);
    start_repl["language"] = start_language;

    exports["start"] = start;
    exports["start_repl"] = start;
    return exports;
}

NODE_API_MODULE(tree_sitter_start_binding, Init)

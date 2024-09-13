#include <Python.h>

typedef struct TSLanguage TSLanguage;

TSLanguage *tree_sitter_start(void);
TSLanguage *tree_sitter_start_repl(void);

static PyObject* _binding_language_start(PyObject *self, PyObject *args) {
    return PyLong_FromVoidPtr(tree_sitter_start());
}

static PyObject* _binding_language_start(PyObject *self, PyObject *args) {
    return PyLong_FromVoidPtr(tree_sitter_start_repl());
}

static PyMethodDef methods[] = {
    {"language", _binding_language_start, METH_NOARGS,
     "Get the tree-sitter language for grammar start."},
    {"language", _binding_language_start_repl, METH_NOARGS,
     "Get the tree-sitter language for grammar start repl."},
    {NULL, NULL, 0, NULL}
};

static struct PyModuleDef module = {
    .m_base = PyModuleDef_HEAD_INIT,
    .m_name = "_binding",
    .m_doc = NULL,
    .m_size = -1,
    .m_methods = methods
};

PyMODINIT_FUNC PyInit__binding(void) {
    return PyModule_Create(&module);
}

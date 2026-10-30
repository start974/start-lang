# Backend

The `vm` interpreter is replaced by native code generation with
**Cranelift** (pure Rust).

```
tir → lir ─┬─ Cranelift : JIT (repl, snbx, test, lsp) + AOT
                    └─ LLVM      : optimized AOT release (later)
```

- Internal startlang IR, `lir` (*low IR*), lowered from `tir` (typed IR), independent of the backend.
- **JIT** (`cranelift-jit`) for the REPL, `.snbx`, tests and the LSP.
- **AOT** (`cranelift-object` + linker) for `startlang build`: native
  executables in `target/`.
- Tail calls (functional language with recursion).
- WebAssembly target (v0.8.0-beta.4).
- Memory: affine types and `const`, no GC (see [memory](./memory.md)).

## FFI with Rust

Through the C ABI (Rust side: `extern "C"`, `#[repr(C)]`).

- Plugins are compiled by the JIT inside the compiler process; Rust
  functions are injected as symbols (no dynamic library).
- AOT objects are linked with Rust crates.
- First version: scalars and pointers only (structures passed by pointer).

## Why Cranelift

| | LLVM | Cranelift |
|---|---|---|
| Generated code | fastest | slower on optimized code |
| Compilation speed | slow | very fast (good for JIT) |
| Integration | heavy C++ dependency | pure Rust |

Cranelift first; LLVM may be added later for optimized release builds
behind the same IR.

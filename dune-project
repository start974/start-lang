(lang dune 3.10)

(name start_lang)

(generate_opam_files true)

(source
 (github start974/start-lang))

(authors "Jérémy Damour")

(maintainers "Jérémy Damour")

(license LICENSE.md)

(documentation https://github.com/start974/start-lang)

(using menhir 2.0)

(package
 (name start_lang)
 (synopsis "A short synopsis")
 (description "A longer description")
 (depends ocaml dune)
 (tags
  (topics "to describe" your project)))

; See the complete stanza docs at https://dune.readthedocs.io/en/stable/dune-files.html#dune-project

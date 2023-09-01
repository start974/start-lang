.PHONY: build
build:
	dune build

.PHONY: test
test:
	dune runtest

.PHONY: coverage
coverage:
	dune runtest --instrument-with bisect_ppx --force
	bisect-ppx-report summary

.PHONY: coverage-html
coverage-html: coverage
	bisect-ppx-report html

.PHONY: format
format:
	dune fmt

.PHONY: clean
clean:
	dune clean
	rm -rf _coverage

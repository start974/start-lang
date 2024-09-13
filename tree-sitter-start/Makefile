TS ?= tree-sitter

all install uninstall clean:
	$(MAKE) -C grammars/start $@
	$(MAKE) -C grammars/start_repl $@

#test:
	#$(TS) test
	#$(TS) parse examples/* --quiet --time

generate:
	cd grammars/start && $(TS) generate --no-bindings
	cd grammars/start_repl && $(TS) generate --no-bindings

.PHONY: all install uninstall clean test update generate

.PHONY: clean

PROJECT := grepping-boom
PROJECT_UNDERSCORE := $(subst -,_,$(PROJECT))
UNAME      := $(shell uname | tr '[:upper:]' '[:lower:]')
LIBSUFFIX := $(if $(filter $(UNAME),darwin),dylib,so)

build:
	cargo build --release
	mkdir -p ./lua
	cp -f target/release/lib$(PROJECT_UNDERSCORE).$(LIBSUFFIX) ./lua/$(PROJECT).so

debug:
	cargo build
	mkdir -p ./lua
	cp -f target/debug/lib$(PROJECT_UNDERSCORE).$(LIBSUFFIX) ./lua/$(PROJECT).so

clean:
	cargo clean
	rm -rf ./lua

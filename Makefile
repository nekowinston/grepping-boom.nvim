.PHONY: all test clean

PROJECT      := grepping-boom
LIBNAME      := $(subst -,_,$(PROJECT))
UNAME        := $(shell uname | tr '[:upper:]' '[:lower:]')
LIBSUFFIX    := $(if $(filter $(UNAME),darwin),dylib,so)
NVIM_VERSION := $(shell nvim --headless --cmd 'lua print(string.format("neovim-%d-%d", vim.version().major, vim.version().minor))' --cmd qa 2>&1)
NVIM_FEAT    := $(if $(filter $(NVIM_VERSION),neovim-0-11),neovim-nightly,$(NVIM_VERSION))

all: build

build:
	cargo build --release --features $(NVIM_FEAT)
	mkdir -p ./lua
	cp -f target/release/lib$(LIBNAME).$(LIBSUFFIX) ./lua/$(PROJECT).so

debug:
	cargo build --features $(NVIM_FEAT)
	mkdir -p ./lua
	cp -f target/debug/lib$(LIBNAME).$(LIBSUFFIX) ./lua/$(PROJECT).so

test:
	cargo check

clean:
	cargo clean
	rm -rf ./lua

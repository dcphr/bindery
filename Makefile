all: build test
.DEFAULT: all
.PHONY: all

build: build-rust build-js
.PHONY: build

build-js:
	cd js && npm install
.PHONY: build-js

build-rust:
	cargo build
.PHONY: build-rust

test: test-rust test-js
.PHONY: test

test-rust:
	cargo test
.PHONY: test-rust

test-js:
	cd js && npm run test
.PHONY: test-js

help:
	@printf "\
bindery makefile\n\
Makefile targets (run \"make <target>\"):\n\
  all          Compile and test everything\n\
  build        Compile everyting\n\
  build-js     Compile web ui code\n\
  build-rust   Compile puzzle hunt compiler\n\
  test         Test everything\n\
  test-js      Test web ui code\n\
  test-rust    Test puzzle hunt compiler\n\
  help         Print this dialog\n\
"
.PHONY: help

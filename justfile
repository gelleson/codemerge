#!/usr/bin/env just --justfile
export PATH := join(justfile_directory(), "node_modules", "bin") + ":" + env_var('PATH')


rust-build:
    @cd rust && cargo build --release

install:
    @bun install

build: rust-build install
    @bun build --compile index.ts --minify --outfile=dist/codemerge

test:
    @bun test

test-watch:
    @bun test --watch

binary-install: build
    @sudo cp dist/codemerge /usr/local/bin/codemerge

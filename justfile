export PATH := join(justfile_directory(), "node_modules", "bin") + ":" + env_var('PATH')
set dotenv-load

rust-build:
    cargo build --release

install:
    bun install

build: rust-build install
    bun build --compile index.ts --minify --outfile=dist/codemerge

test:
    bun test

test-watch:
    bun test --watch

binary-install: build
    sudo cp dist/codemerge /usr/local/bin/codemerge

changelog:
    @echo "Generating changelog for the orphan branch..."

    # Use 'git log' to include dates and format the output
    @bash -c 'echo "## Changelog" > CHANGELOG.md; \
              git log --pretty=format:"%h %ad %s" --date=short >> CHANGELOG.md'

    @echo "Changelog generated."

release: changelog
    @echo "Creating GitHub release..."
    # Replace 'username/repo' with your GitHub repository path
    @gh release create $(shell git describe --tags --abbrev=0) --title "Release $(shell git describe --tags --abbrev=0)" --notes-file CHANGELOG.md
    @echo "GitHub release created."

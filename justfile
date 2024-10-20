
# Build and run the project
build:
    cargo build --release

# Build and run the project in release mode
run:
    cargo run --release

# Publish the project to crates.io
publish:
    cargo publish

# Run tests
test:
    cargo test

# Clean the project
clean:
    cargo clean


# Build and copy to /usr/local/bin
install: build
    sudo mv target/release/codemerge /usr/local/bin/codemerge
    codemerge -V

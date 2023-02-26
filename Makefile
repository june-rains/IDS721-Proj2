install:
	# Install if needed
	@echo "Updating rust toolchain"
	rustup update stable
	rustup default stable

rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

format:
	@echo "Formatting all projects with cargo"
	cargo fmt --quiet

lint:
	@echo "Linting project with cargo"
	cargo clippy --quiet

test:
	@echo "Testing project with cargo"
	cargo test --quiet

run:
	cargo run

all: format lint test run




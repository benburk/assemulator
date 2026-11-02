# Display available recipes
default:
    just --list --unsorted

# Install dependencies and set up the development environment
bootstrap:
    cargo build --all-targets

alias fmt := format

# Format code
format:
    just --fmt
    dprint fmt
    cargo fmt --all
    fd -e nix -X nixfmt
    # The trailing `.` is required: with no path, ripgrep reads stdin when
    # stdin is not a TTY and blocks forever instead of searching the tree.
    rg -l '[^\n]\z' --multiline . | xargs -r sed -i -e '$a\\'

# Run linters and static analysis
check:
    just --fmt --check
    dprint check
    @fd -e md --hidden -E .git -X awk '/^[[:space:]]*[|]/ && length($0) > 80 {print FILENAME ":" FNR ": table row is " length($0) " chars (>80)"; bad=1} END {exit bad}'
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets -- -D warnings
    fd -e nix -X nixfmt --check
    ! rg -l '[^\n]\z' --multiline .

# Run the test suite
test:
    cargo test --workspace

# Run an example CPU, e.g. `just run photon examples/photon/programs/hello_world.asm run`
run cpu *args:
    cargo run --release --example {{ cpu }} -- {{ args }}

# Assemble every finished example program (skips wip/)
examples:
    cargo build --release --examples
    fd -e asm -E wip -E ports.asm -E macros.asm . examples/mira/programs -x ./target/release/examples/mira {} assemble > /dev/null
    fd -e asm -E wip -E ports.asm . examples/photon/programs -x ./target/release/examples/photon {} assemble > /dev/null
    fd -e asm -E wip -E ports.asm -E macros.asm . examples/risc16/programs -x ./target/release/examples/risc16 {} assemble > /dev/null

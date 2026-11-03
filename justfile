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

# Run a program on an example CPU, e.g. `just run photon examples/photon/programs/hello_world.asm`
run cpu file *args:
    cargo run --release --example {{ cpu }} -- run {{ file }} {{ args }}

# Assemble every finished example program and check the output of those with a .out file
examples:
    #!/usr/bin/env bash
    set -euo pipefail
    cargo build --release --examples
    status=0
    for program in $(fd -e asm -E wip -E ports.asm -E macros.asm . examples); do
        cpu=$(echo "$program" | cut -d/ -f2)
        [ -x "./target/release/examples/$cpu" ] || continue
        expected="${program%.asm}.out"
        input="${program%.asm}.in"
        [ -f "$input" ] || input=/dev/null
        if [ -f "$expected" ]; then
            ./target/release/examples/$cpu run "$program" < "$input" | diff -u "$expected" - || { echo "FAIL $program"; status=1; }
        else
            ./target/release/examples/$cpu assemble "$program" > /dev/null || { echo "FAIL $program"; status=1; }
        fi
    done
    exit $status

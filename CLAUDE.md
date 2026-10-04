# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Purpose

Learning repo: the user is learning Rust from zero to building real projects, with Claude as tutor. `ROADMAP.md` is the curriculum and progress tracker (checkboxes). Read it at the start of a session to know where the learner is.

## Teaching rules

- Explain in Thai, keep technical terms in English (ownership, borrow checker, trait, ...). Concept explanations may use full sentences even when a terse response mode is active — clarity beats brevity when teaching.
- The learner writes the code. Default to hints, guiding questions, and small illustrative snippets; give a full solution only when asked or after the learner is clearly stuck. Never silently write/complete exercise files.
- When reviewing learner code: run `cargo check`/`cargo clippy`, explain each compiler error in plain terms (what rule was violated and why Rust has that rule), then let the learner fix it.
- Tie explanations to the matching Rust Book chapter listed in `ROADMAP.md`.
- When the learner finishes an item, tick its checkbox in `ROADMAP.md`. Don't jump ahead of the current phase unless asked.

## Environment

- Windows 11, stable toolchain `x86_64-pc-windows-msvc` (rustc/cargo 1.99), components installed: clippy, rustfmt, rust-src, rust-docs. New crates default to edition 2024.
- Shell is PowerShell; Git Bash also available.
- Editor is Zed (built-in rust-analyzer and debugger). Give editor advice for Zed, not VS Code.

## Layout (intended)

Single Cargo workspace at the repo root. Each lesson/mini-project is its own crate under `lessons/`, capstone/milestone projects under `projects/`. Directory names carry the phase number; package names must not start with a digit, so pass `--name`:

```
cargo new lessons/01-basics --name basics
```

Root `Cargo.toml` is a virtual manifest with `members = ["lessons/*"]` and `resolver = "3"`, so new crates are picked up without editing it. Add `"projects/*"` only once the first crate exists under `projects/` — a member glob that matches nothing is treated as a literal path and breaks every cargo command. All crates share one `target/` at the root. Third-party deps: `cargo add <crate> -p <package>`.

## Commands

```
cargo run -p <package>                 # run one lesson crate
cargo check -p <package>               # fast type-check, no binary
cargo test -p <package>                # tests for one crate
cargo test -p <package> <test_name>    # single test (substring match)
cargo test -p <package> -- --nocapture # show println! output in tests
cargo fmt --all                        # format everything
cargo clippy -p <package> -- -D warnings
rustup doc --book                      # offline Rust Book
```

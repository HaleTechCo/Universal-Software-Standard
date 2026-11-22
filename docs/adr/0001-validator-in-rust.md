# 0001: Write the validator in Rust

* Status: accepted
* Date: 2026-11-21

## Context and Problem Statement

The USS validator must be easy to run anywhere (Windows, Linux, macOS, CI) and
must later run inside a VS Code extension. Which language should it use?

## Considered Options

* Go
* Rust
* Python
* TypeScript

## Decision Outcome

Chosen option: Rust, for WebAssembly and library embedding (VS Code), its type
system, and the original "OS-native -> Rust" rule.

### Consequences

* More dependencies, so `cargo deny` checks them.
* A weaker YAML ecosystem, so ADR 0002 records the YAML crate choice.
* Slower builds.
* The library core does no I/O, so it can be embedded and compiled to WebAssembly.
* The library builds for `wasm32-unknown-unknown`, but `jsonschema` turns on the
  `wasm_js` backend of `getrandom`, so the WebAssembly artifact needs a
  JavaScript host (wasm-bindgen).

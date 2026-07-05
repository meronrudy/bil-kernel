# Getting Started

This guide is for engineers evaluating the workspace from a fresh checkout. It focuses on the fastest path to building the project, validating the invariants, and locating the important code.

For the conceptual model, continue with the [kernel model](kernel-model.md). For the formal reference, see the [spec](../bil_kernel_v0_1_spec.md) and [observation algebra note](../OBSERVATION_ALGEBRA.md).

## Prerequisites

- Rust toolchain with `cargo`
- A shell environment that can run workspace commands from the repository root

## Clone and Enter the Workspace

```bash
git clone https://github.com/meronrudy/bil-kernel.git
cd bil-kernel
```

If you already have the repository checked out, start from the workspace root instead.

## Build and Validate

Run the core verification flow:

```bash
cargo test --workspace
cargo run -p bil-analyzer
cargo run -p observation-algebra-demo
```

## Recommended Validation Before Opening a PR

Before opening or updating a pull request, run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p bil-analyzer
cargo run -p observation-algebra-demo
```

The first three commands check Rust formatting, lint hygiene, and workspace tests. The analyzer checks the kernel boundary. The demo verifies that the observable model and projection examples still execute end to end.

## Prototype Boundary

The examples are designed to demonstrate architecture, not production policy. The projection crates intentionally stay small so the kernel boundary remains visible.

## What Success Looks Like

### 1. Workspace tests

`cargo test --workspace` should finish with all tests passing across:

- `bil-core` receipt lifecycle tests
- `bil-codec` canonical encoding tests
- `bil-crypto` hashing and signature tests

### 2. Analyzer run

`cargo run -p bil-analyzer` should report passes for:

- `bil-core` being `no_std`
- `bil-core` forbidding unsafe code
- absence of forbidden semantic identifiers in `bil-core`
- absence of domain crate imports in `bil-core`
- presence of example dependencies on `bil-core`
- presence of lifecycle and codec/hash tests
- presence of the observation algebra demo

The run should end with:

```text
RESULT: HOLONOMY PRESERVED
```

### 3. Observation algebra demo

`cargo run -p observation-algebra-demo` should:

- construct a semantically neutral observable
- canonically encode and hash it
- move it through `Draft -> StructurallyVerified -> Signed -> Anchored`
- print three different projection views for bank, insurance, and legal readers

The run should end with:

```text
RESULT: SEMANTIC NON-CONTAINMENT WITNESSED
```

## Where to Read the Code First

- `crates/bil-core/src/event.rs`: `KernelEvent` and the kernel observable shape
- `crates/bil-core/src/receipt.rs`: typed receipt lifecycle and verification/signing/anchoring traits
- `crates/bil-codec/src/lib.rs`: canonical event encoding
- `crates/bil-crypto/src/lib.rs`: receipt hashing and mock signature engine
- `crates/bil-analyzer/src/main.rs`: boundary enforcement checks
- `examples/observation-algebra-demo/src/main.rs`: end-to-end walkthrough of one observable flowing through the system

## Next Reads

- [Kernel model](kernel-model.md)
- [Projections and examples](projections-and-examples.md)
- [BIL Kernel Specification v0.1](../bil_kernel_v0_1_spec.md)
- [BIL Observation Algebra](../OBSERVATION_ALGEBRA.md)

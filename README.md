# BIL Kernel

Shared facts without shared institutional meaning.

BIL Kernel is a minimal `no_std` Rust trust kernel for institutional evidence receipts. It records semantically neutral observables that can be verified, hashed, signed, anchored, and then interpreted by domain-specific projection layers without merging institutional state into the kernel itself.

## What It Is

- A small core for representing institutionally observed facts as opaque kernel data
- A typed receipt lifecycle for moving an observation from `Draft` to `Anchored`
- A workspace that separates kernel types, canonical encoding, cryptographic helpers, analyzer checks, and example projections

## What It Is Not

- Banking, insurance, legal, actuarial, treasury, or regulatory business logic
- A domain state machine or decision engine
- A place to store institutional semantics such as balances, claims outcomes, premiums, duties, or risk scores

The core invariant is semantic non-containment:

`K ∩ S_i = ∅`

In practice, that means the kernel preserves shared evidence structure while every domain computes its own local meaning outside the kernel.

## Quickstart

From the workspace root:

```bash
cargo test --workspace
cargo run -p bil-analyzer
cargo run -p observation-algebra-demo
```

What these commands verify:

- `cargo test --workspace` exercises receipt lifecycle, canonical encoding, and hashing/signing helpers across the workspace.
- `cargo run -p bil-analyzer` checks that `bil-core` stays `no_std`, forbids unsafe code, avoids domain-specific identifiers, and keeps the examples wired to the kernel.
- `cargo run -p observation-algebra-demo` constructs one anchored observation and shows how bank, insurance, and legal projections derive different local views from the same kernel record.

The analyzer should end with `RESULT: HOLONOMY PRESERVED`. The demo should end with `RESULT: SEMANTIC NON-CONTAINMENT WITNESSED`.

## Workspace Map

| Crate | Responsibility |
| --- | --- |
| `bil-core` | Core kernel types: `KernelEvent`, typed receipt lifecycle, primitive identifiers, and projection trait |
| `bil-codec` | Canonical byte encoding for `KernelEvent` |
| `bil-crypto` | Hashing and signature helper traits plus mock engines used by tests and demos |
| `bil-analyzer` | Static checks that enforce kernel boundary rules |
| `examples/*-projection` | Domain-specific projections that interpret the same kernel observation differently |
| `examples/observation-algebra-demo` | End-to-end runnable example of lifecycle plus multi-domain projection |

## Kernel Model at a Glance

`KernelEvent` is the kernel's observable record. It captures:

- who or what was observed (`subject`)
- under whose authority it was emitted (`authority`)
- when it was anchored (`time`)
- what opaque type was declared (`event_type`)
- what evidence hash was committed (`evidence`)
- optional value, state, and previous-link commitments

The kernel does not decide what any of that means for a bank, insurer, court, or regulator. It only preserves the observable in a reproducible form.

Receipts move through a typed lifecycle:

`Draft -> StructurallyVerified -> Signed -> Anchored`

Once anchored, projection crates consume the same `KernelEvent` and produce domain-local interpretations. That is where semantics are added.

## Read Next

- [Getting started](docs/getting-started.md)
- [Kernel model](docs/kernel-model.md)
- [Projections and examples](docs/projections-and-examples.md)
- [BIL Kernel Specification v0.1](bil_kernel_v0_1_spec.md)
- [BIL Observation Algebra](OBSERVATION_ALGEBRA.md)

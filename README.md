# BIL Kernel

**Shared facts without shared institutional meaning.**

BIL Kernel is a minimal `no_std` Rust trust kernel for institutional evidence receipts. It emits deterministic, cryptographically reproducible, semantically neutral events that can be interpreted by banking, insurance, legal, treasury, regulatory, or actuarial projection layers without merging institutional state.

## Status

BIL Kernel is an early research/prototype workspace for institutional evidence receipts.

The current repository demonstrates:

- semantically neutral kernel observables
- typed receipt lifecycle transitions
- canonical encoding
- hashing and signing interfaces
- analyzer checks for kernel-boundary preservation
- example projection crates for banking, insurance, and legal interpretations

It is not production cryptography, regulatory advice, a compliance product, or a deployable banking system.

## Who This Is For

This repository is intended for:

- Rust developers evaluating typed evidence systems
- researchers studying institutional observability and assurance kernels
- builders designing banking, insurance, legal, treasury, regulatory, or actuarial projection layers
- early adopters evaluating semantically neutral receipt infrastructure

If you are looking for domain business logic, start in the example projection crates. If you are evaluating the kernel boundary itself, start with `bil-core`, `bil-codec`, `bil-crypto`, and `bil-analyzer`.

## Security Notice

The current workspace demonstrates deterministic evidence structure and receipt lifecycle mechanics.

Mock signing engines and demo cryptographic helpers are used for tests and examples. They are not production key-management, compliance-grade cryptography, hardware-backed signing, or regulatory assurance infrastructure.

Production use would require a separate security design covering key custody, signing policy, canonicalization stability, versioning, audit retention, threat modeling, and operational controls.

## The Core Invariant: Semantic Non-Containment

The kernel models *institutional observables*, not reality directly. It records that an opaque observation was emitted, committed, signed, and anchored under specified structural rules.

The core invariant is semantic non-containment:

$$
\mathcal K \cap S_i = \varnothing
$$

The kernel ($\mathcal K$) contains no domain semantics ($S_i$). Domain crates may depend on the kernel; the kernel may not depend on domains.

## Observation Algebra

The kernel provides an executable observation algebra:

1.  **KernelObservable**: A semantically neutral observable admitted into the evidence kernel.
2.  **Canonical Encoding**: Identical observables produce identical byte representations.
3.  **Deterministic Hashing**: Canonical bytes yield a reproducible receipt hash.
4.  **Typestate Lifecycle**: Receipts mature through strict stages (`Draft` → `StructurallyVerified` → `Signed` → `Anchored`). Only anchored receipts are admissible as evidence.
5.  **Measurable Projections**: Domain meaning is computed only by projection maps ($\phi_i:\mathcal K\rightarrow S_i$). The kernel preserves the observation without collapsing it into any institutional interpretation.

## Project Structure

The workspace is divided into core kernel crates and domain projection examples:

*   `crates/bil-core`: The `#![no_std]` kernel defining opaque primitives, the `KernelEvent` observable, and the typestate receipt lifecycle.
*   `crates/bil-codec`: Canonical encoding for kernel observables.
*   `crates/bil-crypto`: Cryptographic traits (hashing, signatures) and deterministic mock engines.
*   `crates/bil-analyzer`: A static analysis tool that verifies the kernel's invariants (e.g., no domain semantics, `#![no_std]`).
*   `examples/observation-algebra-demo`: An executable witness proving semantic non-containment.
*   `examples/*-projection`: Example domain crates (Bank, Insurance, Legal) that project kernel observables into local views.

## Getting Started

### Running the Observation Algebra Demo

This demo proves the core thesis: a single kernel observable can be anchored and projected into multiple institutional views while preserving an invariant receipt hash.

```bash
cargo run -p observation-algebra-demo
```

### Running the Analyzer

The analyzer acts as an implementation certificate, verifying that the kernel strictly adheres to its design invariants.

```bash
cargo run -p bil-analyzer
```

### Running Tests

```bash
cargo test --workspace
```

## Documentation

*   [Getting Started](docs/getting-started.md): Build, validate, and navigate the workspace.
*   [Kernel Model](docs/kernel-model.md): The practical observable model and receipt lifecycle.
*   [Projections and Examples](docs/projections-and-examples.md): How the example domains consume the same anchored observation.
*   [Observation Algebra](OBSERVATION_ALGEBRA.md): The mathematical and conceptual foundation of the kernel.
*   [Specification v0.1](bil_kernel_v0_1_spec.md): Detailed specification of the kernel's purpose, primitives, and lifecycle.

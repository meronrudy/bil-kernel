# BIL Kernel Specification v0.1

This document is the formal reference for the current kernel scope and invariants. If you are new to the repository, start with the [README](README.md) and the practical docs in [docs/getting-started.md](docs/getting-started.md) and [docs/kernel-model.md](docs/kernel-model.md), then return here for the precise specification language.

Use this document when you need the authoritative statement of kernel purpose, non-goals, lifecycle boundaries, and projection model.

The BIL Kernel is a minimal institutional evidence kernel for producing deterministic, verifiable, semantically neutral receipts.

The kernel is not banking software, insurance software, legal software, actuarial software, or governance software. It does not compute premiums, balances, legal duties, reserves, risk scores, compliance outcomes, or claims decisions.

The kernel records only the minimal factual structure required for institutions to coordinate over shared events:

- who or what participated;
- when the event was anchored;
- what opaque event type was declared;
- under whose authority it was emitted;
- what evidence hash was committed;
- what optional value/state commitments were attached;
- what prior receipt it links to.

All domain meaning is computed outside the kernel by projection crates.

## 1. Kernel purpose
To provide a thin-waist executable artifact that preserves holonomy invariants across institutional boundaries.

## 2. Non-goals
- Business logic execution
- Domain semantic interpretation
- State machine transitions (beyond receipt lifecycle)

## 3. Primitive set
Opaque byte arrays representing identifiers, hashes, and commitments.

## 4. Receipt lifecycle
`Draft` → `StructurallyVerified` → `Signed` → `Anchored`

## 5. Semantic non-containment
The kernel must not import or define domain-specific types.

## 6. Projection model
Domain crates implement `Projection<KernelEvent>` to derive local meaning.

## 7. Analyzer checks
Static analysis to enforce `no_std`, acyclic dependencies, and semantic exclusion.

## 8. Actuarial bridge
Projections that extract statistical features from the receipt graph for risk inference.

## 9. Kernel observables, not world events
`KernelEvent` is named historically. Conceptually, it should be understood as a kernel observable.

The BIL Kernel does not claim that a world event occurred, nor that a domain interpretation is true. It records that an opaque observation was emitted under an authority reference, time anchor, event type identifier, evidence commitment, optional value/state commitments, and previous-link commitment.

Future versions may rename `KernelEvent` to `KernelObservable`, but v0.1 preserves the existing name for API stability.

## Related Reads

- [README](README.md)
- [Getting started](docs/getting-started.md)
- [Kernel model](docs/kernel-model.md)
- [Projections and examples](docs/projections-and-examples.md)
- [BIL Observation Algebra](OBSERVATION_ALGEBRA.md)

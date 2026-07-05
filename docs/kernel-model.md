# Kernel Model

This document explains the practical model exposed by the current workspace. It is the bridge between the top-level README and the deeper formal documents.

For the formal framing, see the [spec](../bil_kernel_v0_1_spec.md) and [observation algebra note](../OBSERVATION_ALGEBRA.md).

## Kernel Observable

`KernelEvent` is the kernel's observable record. It does not claim direct truth about the world. It records that an observation entered an evidence process under a particular authority, time anchor, and cryptographic commitment structure.

The current event fields are:

- `id`: kernel event identifier
- `subject`: identity or thing being observed
- `authority`: authority reference under which the observation was emitted
- `time`: time anchor
- `event_type`: opaque event type identifier
- `evidence`: evidence commitment hash
- `value`: optional value commitment
- `state`: optional state commitment
- `previous`: optional link to a prior observation

These are structural fields. Domain meaning is intentionally left out.

## Semantic Non-Containment

The kernel is designed so that shared evidence structure can be reused across institutions without forcing those institutions to share semantic state.

That is the purpose of the invariant:

`K ∩ S_i = ∅`

`K` is the kernel space. `S_i` is a domain-specific semantic space such as banking, insurance, or legal interpretation. The kernel may be consumed by those domains, but it must not contain them.

## Receipt Lifecycle

The core type-level workflow is:

`Draft -> StructurallyVerified -> Signed -> Anchored`

Each stage narrows what operations are available:

- `Receipt<Draft>` can be created from a `KernelEvent` plus a `ReceiptHash`
- `verify_structure(...)` produces `Receipt<StructurallyVerified>`
- `sign(...)` produces `Receipt<Signed>`
- `anchor(...)` produces `Receipt<Anchored>`

Only an anchored receipt exposes the finalized observable through `event()`, `hash()`, and `signature()`.

This keeps the lifecycle explicit in the type system instead of relying on runtime flags.

## Encoding, Hashing, Signing, Anchoring

The surrounding crates provide the minimal machinery needed to operate on the observable:

- `bil-codec` defines canonical encoding for `KernelEvent`
- `bil-crypto` hashes those canonical bytes into a `ReceiptHash`
- `bil-core` defines `ReceiptSigner` and `ReceiptAnchor` traits for signature and anchoring steps

Conceptually, the flow is:

1. Build a semantically neutral `KernelEvent`
2. Canonically encode it into deterministic bytes
3. Hash those bytes into a receipt hash
4. Verify structure, sign, and anchor the receipt
5. Hand the anchored observable to projections

The current crypto crate uses mock engines in tests and demos. The important property here is deterministic structure, not production cryptography policy.

## Cryptographic Boundary

The kernel model depends on deterministic structure:

1. canonical bytes
2. stable receipt hashes
3. explicit signing interfaces
4. anchored receipt lifecycle states

The current implementation demonstrates these boundaries with mock or helper cryptographic components suitable for tests and examples.

Production deployments should treat signing, key custody, anchoring, algorithm agility, audit retention, and revocation as separate integration layers outside the kernel core.

## Where Semantics Enter

Semantics enter through projections, not through the kernel. A projection implements:

```rust
pub trait Projection<E> {
    type Output;

    fn project(event: &E) -> Self::Output;
}
```

That allows different institutions to derive different local views from the same anchored observation while the kernel remains neutral.

Continue with [projections and examples](projections-and-examples.md) for the concrete walkthrough.

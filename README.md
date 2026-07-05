# BIL Kernel

Shared facts without shared institutional meaning.

BIL Kernel is a minimal `no_std` Rust trust kernel for institutional evidence receipts. It emits deterministic, cryptographically reproducible, semantically neutral events that can be interpreted by banking, insurance, legal, treasury, regulatory, or actuarial projection layers without merging institutional state.

The core invariant is semantic non-containment:

K ∩ Sᵢ = ∅

The kernel contains no domain semantics. Domain crates may depend on the kernel; the kernel may not depend on domains.

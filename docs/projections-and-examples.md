# Projections and Examples

This guide walks through how the workspace demonstrates semantic non-containment in code. It is centered on the observation algebra demo and the three example projection crates.

For the underlying model, see the [kernel model](kernel-model.md). For the formal framing, see the [spec](../bil_kernel_v0_1_spec.md) and [observation algebra note](../OBSERVATION_ALGEBRA.md).

## Observation Algebra Demo

Run:

```bash
cargo run -p observation-algebra-demo
```

The demo performs five steps:

1. Constructs a `KernelEvent` with opaque identifiers and commitments
2. Canonically encodes and hashes that event
3. Moves the receipt through verification, signing, and anchoring
4. Applies three projections to the anchored event
5. Confirms that projection-local labels differ while the underlying receipt hash stays unchanged

This is the core claim of the workspace in runnable form: one shared anchored observation, multiple institutional readings, no shared semantic state inside the kernel.

## Example Projections

The example crates are intentionally small. They show where domain meaning belongs, not complete domain systems.

### `bank-projection`

`examples/bank-projection/src/lib.rs` produces a `BankView` with:

- a bank-local label
- whether an authority was observed
- whether a value commitment is present

### `insurance-projection`

`examples/insurance-projection/src/lib.rs` produces an `InsuranceView` with:

- an insurance-local label
- whether evidence is present
- whether a prior link is present

### `legal-projection`

`examples/legal-projection/src/lib.rs` produces a `LegalView` with:

- a legal-local label
- whether an authority reference is present
- whether an evidence commitment is present

Each projection reads the same `KernelEvent`. The difference is entirely in the local output type.

## Why These Examples Matter

The examples show three important boundaries:

- `bil-core` does not import domain crates
- domain crates depend on `bil-core`, not the other way around
- the same anchored observation can support multiple readings without forcing shared business logic

That is also what `bil-analyzer` checks statically.

## Where to Extend From Here

If you are evaluating the architecture, start with these files:

- `examples/observation-algebra-demo/src/main.rs`
- `examples/bank-projection/src/lib.rs`
- `examples/insurance-projection/src/lib.rs`
- `examples/legal-projection/src/lib.rs`

If you are designing a new projection, follow the same pattern: consume `KernelEvent`, define a local output type, and keep all domain semantics outside the kernel.

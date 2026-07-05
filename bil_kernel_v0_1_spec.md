# BIL Kernel Specification v0.1

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

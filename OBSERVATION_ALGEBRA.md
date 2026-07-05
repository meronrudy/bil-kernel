# BIL Observation Algebra

The BIL Kernel does not model reality directly. It models institutional observables: opaque, authority-bound, time-anchored, cryptographically committed objects that have entered an evidence process.

A kernel observable is not a claim that an event is true. It is a record that an observation was emitted, committed, signed, and anchored under specified structural rules.

This distinction is central:

- reality is not directly available to the kernel;
- institutional evidence is available to the kernel;
- domain meaning is computed only by projections;
- the kernel preserves the observation without collapsing it into any institutional interpretation.

## Mathematical Bridge

The kernel observable $K_t$ is an element of the kernel space $\mathcal K$:

$$
K_t \in \mathcal K
$$

Domain meaning is derived via measurable projection maps $\phi_i$ into semantic spaces $S_i$:

$$
\phi_i:\mathcal K\rightarrow S_i
$$

The core invariant of the kernel is semantic non-containment:

$$
\mathcal K\cap S_i=\varnothing
$$

The history of anchored observables forms an evidence filtration:

$$
\mathcal H_t^K=\sigma(K_s:s\le t)
$$

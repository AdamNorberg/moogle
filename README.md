# Moogle

`moogle` is an alternative to `BTreeMap` with a lot of desirable properties for game-like simulations. These include determinism, symmetry, and uniqueness of associations between keys/values.

Its interface is fairly close to Rust's builtin map interface. I haven't profiled it yet, but based on its implementation, I'd expect it to be within an order of magnitude of BTreeMap in all respects. 

## Specifics

Specifically: `moogle` provides an internal bimap type with four public specializations. Each is equivalent to a different kind of junction table.

The four specializations are below:

- `OneToOne`: maps `Optional<T>` to `Optional<T>`
- `OneToSet`: maps `Optional<T>` to `BTreeSet<T>`
- `SetToOne`: maps `BTreeSet<T>` to `Optional<T>`
- `SetToSet`: maps `BTreeSet<T>` to `BTreeSet<T>`

The bimap is based on `BTreeMap`, meaning it preserves `Ord` of elements and is deterministic. Elements are required to be `PartialEq`, `Ord` and `Copy`. (Some examples of types satisfying these requirements are numeric IDs and UUIDs.)

Each specialization can be viewed in a forwards direction (using the `.fwd()` accessor) and a backwards direction (using the `.bwd()` accessor) -- for instance, `OneToSet<usize, char>` corresponds to a `BTreeMap<usize, BTreeSet<char>>` and a `BTreeMap<char, usize>` that are always kept in sync. 

(What does "kept in sync" mean? Formally: `insert()`ing on one automatically `insert()`s on the other, and each pair `(a, b)` in one has a corresponding pair `(b, a)` in the other.)

`Set`-based specializations provide an `Entry`-like interface for insertions, as well as a `.items()` iterator that averts that interface.
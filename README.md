# reco
**R**ust **E**ncyclopaedia of **C**hess **O**penings.

This crate is an implementation of the [ECO](https://en.wikipedia.org/wiki/Encyclopaedia_of_Chess_Openings) standard using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

Specifically, this crate contains:
- Types to represent each part of an ECO entry.
- ECO entries as constants, sourced from [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

No-std and no-alloc compatible.

## Features
- `book`: re-exports [`reco-book`](https://crates.io/crates/reco-book).
- `alloc`: feature forwarded from [`reco-core`](https://crates.io/crates/reco-core).
- `serde`: feature forwarded from [`reco-core`](https://crates.io/crates/reco-core).

## Sourcing
The ECO entries are sourced using the (unpublished) `librarian` crate (see the [repository](https://github.com/tigerros/reco)).
It generates a file for each ECO code in the corresponding volume directory.
That file contains all the openings, as constants, for that ECO code.

TODO: when is the librarian script going to run? set up a workflow that runs it if all other workflows pass, then commit the changes? perhaps make even releasing a new version automatic?
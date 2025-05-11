# reco
**R**ust **E**ncyclopaedia of **C**hess **O**penings.

This crate is an implementation of the [ECO](https://en.wikipedia.org/wiki/Encyclopaedia_of_Chess_Openings) using [`shakmaty`](https://crates.io/crates/shakmaty) for relevant types.

Specifically, this crate contains:
- Types to represent each part of an ECO entry.
- ECO entries as constants, sourced from [lichess-org/chess-openings](https://github.com/lichess-org/chess-openings).

`#![no_std]` and no-alloc compatible.

## Features
- `alloc`: adds the `OpeningOwned` struct, which is an owned version of the `Opening` struct.
- `serde`: enables serde support for all types. **All implementations are derived with no parameters. This includes types that have a `FromStr` and `Display` implementation.**

## Sourcing
The ECO entries are sourced using the (unpublished) `updater` crate (see the [repository](https://github.com/tigerros/reco)).
It generates a file for each ECO code in the corresponding volume directory.
That file contains all the openings, as constants, for that ECO code.

TODO: when is the updater script going to run? set up a workflow that runs it if all other workflows pass, then commit the changes? perhaps make even releasing a new version automatic?
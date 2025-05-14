# reco-core
This crate contains the core types used by the main crate, [`reco`](https://crates.io/crates/reco).
You should use that crate instead.

No-std and no-alloc compatible.

## Features
- `alloc`: adds the `OpeningOwned` struct, which is an owned version of the `Opening` struct.
- `serde`: enables serde support for applicable types. **All implementations are derived with no parameters. This includes types that have a `FromStr` and `Display` implementation.**
# use-rustacean

`use-rustacean` provides friendly deterministic fixture data for RustUse docs, tests, examples, tutorials, demos, and UI/sample data.

The main concept is **Fellow Friends**: typed metadata records inspired by historical and modern technology mascots, logo characters, community symbols, and ecosystem identities. Some entries are mascots, some are logo characters, and some are symbols. The crate includes metadata only. It does not include graphics, logos, SVGs, images, mascot artwork, or fan-art assets.

## Basic Usage

```rust
use use_rustacean::prelude::*;

let friends = FriendFixtures::small();
let registry = FriendRegistry::new(friends);
let rust_friends = registry.by_ecosystem("Rust");

assert!(!rust_friends.is_empty());
```

## Fixture Use Cases

- tests that need stable, readable records
- docs that need small examples with friendly names
- examples and tutorials that should not depend on network calls
- sorting demos with predictable data
- filtering demos by ecosystem, tag, technology kind, identity kind, or figure kind
- grouping demos with deterministic `BTreeMap` counts
- markdown rendering for docs snippets and sample pages
- serialization fixtures through the optional `serde` feature

## Crates

- `use-friend`: core `Friend` model, taxonomy enums, helper methods, display labels, and optional serde support.
- `use-friend-fixture`: deterministic static Fellow Friends records.
- `use-friend-registry`: collection and query helpers.
- `use-friend-markdown`: simple deterministic markdown rendering.
- `use-rustacean`: facade crate with ergonomic re-exports and a prelude.

## Scope

- The data is deterministic and static.
- The fixture set includes historical and modern technology friends.
- Entries may describe mascots, logo characters, or symbols.
- Notes are short, factual, and conservative.
- The records are fixtures, not authoritative trademark, branding, or legal metadata.
- No randomness, network calls, artwork, logos, SVGs, images, fan-art, or binary assets are included.

## License

Licensed under `MIT OR Apache-2.0`.

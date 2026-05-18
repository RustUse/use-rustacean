# use-rustacean

`use-rustacean` is a deterministic RustUse fixture/data set for docs, tests, examples, tutorials, demos, and UI/sample data.

The set introduces **Fellow Friends**, a typed collection of metadata records inspired by historical and modern technology mascots, logo characters, community symbols, and ecosystem identities. Some entries are mascots, some are logo characters, and some are symbols. It provides metadata only and intentionally ships no artwork, logos, SVGs, images, or fan-art assets.

## Example

```rust
use use_rustacean::prelude::*;

let friends = FriendFixtures::small();
let registry = FriendRegistry::new(friends);
let rust_friends = registry.by_ecosystem("Rust");

assert!(!rust_friends.is_empty());
```

## What It Is For

- stable docs and tutorials
- fixture records for tests
- demos for sorting, filtering, grouping, and rendering
- deterministic sample data for UI and markdown output
- serialization fixtures behind the optional `serde` feature

## Boundaries

The data is intentionally conservative and non-authoritative. It avoids legal or trademark claims, first-seen years, network lookups, random generation, artwork, and media assets.

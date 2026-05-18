# use-rustacean

`use-rustacean` is the facade crate for RustUse Fellow Friends deterministic fixture data.

It re-exports the core model, deterministic fixture records, registry query helpers, markdown rendering helpers, and a prelude for common usage. The crate includes metadata only and does not include graphics, logos, SVGs, images, mascot artwork, or fan-art assets.

## Example

```rust
use use_rustacean::prelude::*;

let friends = FriendFixtures::small();
let registry = FriendRegistry::new(friends);
let rust_friends = registry.by_ecosystem("Rust");

assert!(!rust_friends.is_empty());
```

## License

Licensed under `MIT OR Apache-2.0`.

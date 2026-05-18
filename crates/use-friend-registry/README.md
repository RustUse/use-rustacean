# use-friend-registry

`use-friend-registry` provides collection and query helpers for deterministic Fellow Friends fixture records.

Use it to filter by ecosystem, tags, technology kind, identity kind, or figure kind, and to build deterministic grouped counts for examples and tests.

## Example

```rust
use use_friend_fixture::FriendFixtures;
use use_friend_registry::FriendRegistry;

let registry = FriendRegistry::new(FriendFixtures::small());
let rust_friends = registry.by_ecosystem("Rust");

assert!(!rust_friends.is_empty());
```

## License

Licensed under `MIT OR Apache-2.0`.

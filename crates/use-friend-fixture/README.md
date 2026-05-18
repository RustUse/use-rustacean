# use-friend-fixture

`use-friend-fixture` provides deterministic Fellow Friends fixture records for RustUse examples, tests, docs, demos, and UI/sample data.

The records are static metadata only. They are not authoritative legal, trademark, or branding data, and they do not include artwork, logos, SVGs, images, or binary assets.

## Example

```rust
use use_friend_fixture::FriendFixtures;

let friends = FriendFixtures::small();

assert!(friends.iter().any(|friend| friend.matches_ecosystem("Rust")));
```

## License

Licensed under `MIT OR Apache-2.0`.

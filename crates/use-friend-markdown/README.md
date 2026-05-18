# use-friend-markdown

`use-friend-markdown` renders Fellow Friends fixture records as small deterministic markdown snippets.

The renderer produces simple cards, tables, and lists for docs, examples, tutorials, and snapshot-style tests. It renders metadata text only and does not reference artwork or media assets.

## Example

```rust
use use_friend_fixture::FriendFixtures;
use use_friend_markdown::FriendMarkdown;

let card = FriendMarkdown::card(&FriendFixtures::tiny()[0]);

assert!(card.contains("### Ferris"));
```

## License

Licensed under `MIT OR Apache-2.0`.
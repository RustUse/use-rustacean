use use_rustacean::prelude::*;

fn main() {
    let friends = FriendFixtures::small();
    let registry = FriendRegistry::new(friends);
    let rust_friends = registry.by_ecosystem("Rust");

    assert!(!rust_friends.is_empty());

    let markdown = FriendMarkdown::list(friends);
    assert!(markdown.contains("Ferris"));
}

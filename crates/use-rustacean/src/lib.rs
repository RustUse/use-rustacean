#![doc = include_str!("../README.md")]

//! Thin facade crate for RustUse Fellow Friends fixture data.

/// Core model crate re-export.
pub use use_friend as friend;
/// Deterministic fixture crate re-export.
pub use use_friend_fixture as fixture;
/// Markdown rendering crate re-export.
pub use use_friend_markdown as markdown;
/// Registry helper crate re-export.
pub use use_friend_registry as registry;

#[cfg(feature = "serde")]
pub use use_friend::FriendRecord;
pub use use_friend::{FigureKind, Friend, IdentityKind, TechnologyKind};
pub use use_friend_fixture::FriendFixtures;
pub use use_friend_markdown::FriendMarkdown;
pub use use_friend_registry::FriendRegistry;

pub mod prelude;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[test]
    fn prelude_supports_expected_usage() {
        let friends = FriendFixtures::small();
        let registry = FriendRegistry::new(friends);
        let rust_friends = registry.by_ecosystem("Rust");

        assert!(!rust_friends.is_empty());
        assert_eq!(rust_friends[0].slug(), "rust-ferris");
    }

    #[test]
    fn facade_exposes_markdown_renderer() {
        let card = FriendMarkdown::card(&FriendFixtures::tiny()[0]);

        assert!(card.contains("### Ferris"));
    }
}

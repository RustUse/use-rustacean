//! Common re-exports for ergonomic Fellow Friends examples.

#[cfg(feature = "serde")]
pub use use_friend::FriendRecord;
pub use use_friend::{FigureKind, Friend, IdentityKind, TechnologyKind};
pub use use_friend_fixture::FriendFixtures;
pub use use_friend_markdown::FriendMarkdown;
pub use use_friend_registry::FriendRegistry;

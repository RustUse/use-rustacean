#![doc = include_str!("../README.md")]

//! Collection and query helpers for Fellow Friends fixture records.

use std::{borrow::Borrow, collections::BTreeMap};

use use_friend::{FigureKind, Friend, IdentityKind, TechnologyKind};
use use_friend_fixture::FriendFixtures;

/// Owned registry of Fellow Friends records.
#[derive(Clone, Debug, Default)]
pub struct FriendRegistry {
    friends: Vec<Friend>,
}

impl FriendRegistry {
    /// Builds a registry from owned friends or borrowed fixture records.
    #[must_use]
    pub fn new<I>(friends: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Friend>,
    {
        Self {
            friends: friends.into_iter().map(|friend| *friend.borrow()).collect(),
        }
    }

    /// Builds a registry from a static fixture slice.
    #[must_use]
    pub fn from_static(friends: &'static [Friend]) -> Self {
        Self::new(friends)
    }

    /// Builds a registry containing every bundled fixture record.
    #[must_use]
    pub fn all() -> Self {
        Self::from_static(FriendFixtures::all())
    }

    /// Returns the number of friends in the registry.
    #[must_use]
    pub fn len(&self) -> usize {
        self.friends.len()
    }

    /// Returns `true` when the registry has no records.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.friends.is_empty()
    }

    /// Iterates over friends in registry order.
    pub fn iter(&self) -> impl Iterator<Item = &Friend> {
        self.friends.iter()
    }

    /// Returns friends matching an ecosystem name.
    #[must_use]
    pub fn by_ecosystem(&self, ecosystem: &str) -> Vec<&Friend> {
        self.iter()
            .filter(|friend| friend.matches_ecosystem(ecosystem))
            .collect()
    }

    /// Returns friends containing a tag.
    #[must_use]
    pub fn by_tag(&self, tag: &str) -> Vec<&Friend> {
        self.iter().filter(|friend| friend.has_tag(tag)).collect()
    }

    /// Returns friends with the requested technology kind.
    #[must_use]
    pub fn by_technology_kind(&self, kind: TechnologyKind) -> Vec<&Friend> {
        self.iter()
            .filter(|friend| friend.technology_kind == kind)
            .collect()
    }

    /// Returns friends with the requested identity kind.
    #[must_use]
    pub fn by_identity_kind(&self, kind: IdentityKind) -> Vec<&Friend> {
        self.iter()
            .filter(|friend| friend.identity_kind == kind)
            .collect()
    }

    /// Returns friends with the requested figure kind.
    #[must_use]
    pub fn by_figure_kind(&self, kind: FigureKind) -> Vec<&Friend> {
        self.iter()
            .filter(|friend| friend.figure_kind == kind)
            .collect()
    }

    /// Finds one friend by stable identifier.
    #[must_use]
    pub fn find_by_id(&self, id: &str) -> Option<&Friend> {
        let normalized = id.trim();

        if normalized.is_empty() {
            return None;
        }

        self.iter().find(|friend| friend.id == normalized)
    }

    /// Counts friends by technology kind in deterministic key order.
    #[must_use]
    pub fn count_by_technology_kind(&self) -> BTreeMap<TechnologyKind, usize> {
        let mut counts = BTreeMap::new();

        for friend in &self.friends {
            *counts.entry(friend.technology_kind).or_default() += 1;
        }

        counts
    }

    /// Counts friends by identity kind in deterministic key order.
    #[must_use]
    pub fn count_by_identity_kind(&self) -> BTreeMap<IdentityKind, usize> {
        let mut counts = BTreeMap::new();

        for friend in &self.friends {
            *counts.entry(friend.identity_kind).or_default() += 1;
        }

        counts
    }
}

#[cfg(test)]
mod tests {
    use super::FriendRegistry;
    use use_friend::{FigureKind, IdentityKind, TechnologyKind};
    use use_friend_fixture::FriendFixtures;

    #[test]
    fn builds_from_static_slice_with_new() {
        let registry = FriendRegistry::new(FriendFixtures::small());
        let rust_friends = registry.by_ecosystem(" rust ");

        assert_eq!(registry.len(), 6);
        assert_eq!(rust_friends.len(), 1);
        assert_eq!(rust_friends[0].name, "Ferris");
        assert!(!registry.is_empty());
    }

    #[test]
    fn filters_by_tag_and_kinds() {
        let registry = FriendRegistry::all();

        assert_eq!(registry.by_tag("container")[0].ecosystem, "Docker");
        assert_eq!(
            registry
                .by_technology_kind(TechnologyKind::ProgrammingLanguage)
                .len(),
            12
        );
        assert_eq!(
            registry.by_identity_kind(IdentityKind::LogoCharacter).len(),
            8
        );
        assert_eq!(registry.by_figure_kind(FigureKind::Creature).len(), 7);
    }

    #[test]
    fn filters_new_zig_database_and_language_records() {
        let registry = FriendRegistry::all();
        let zig_names: Vec<_> = registry
            .by_ecosystem("Zig")
            .into_iter()
            .map(|friend| friend.name)
            .collect();
        let database_names: Vec<_> = registry
            .by_tag("database")
            .into_iter()
            .map(|friend| friend.name)
            .collect();
        let language_ecosystems: Vec<_> = registry
            .by_technology_kind(TechnologyKind::ProgrammingLanguage)
            .into_iter()
            .map(|friend| friend.ecosystem)
            .collect();

        assert!(zig_names.contains(&"Zero the Ziguana"));
        assert!(zig_names.contains(&"Ziggy the Ziguana"));
        assert!(zig_names.contains(&"Carmen the Allocgator"));
        assert!(database_names.contains(&"Sakila"));
        assert!(database_names.contains(&"MariaDB Sea Lion"));

        for ecosystem in ["Java", "PHP", "Zig", "Raku", "Perl", "Crystal"] {
            assert!(language_ecosystems.contains(&ecosystem));
        }
    }

    #[test]
    fn finds_by_stable_identifier() {
        let registry = FriendRegistry::all();

        assert_eq!(
            registry
                .find_by_id(" github-octocat ")
                .map(|friend| friend.name),
            Some("Octocat")
        );
        assert_eq!(registry.find_by_id(""), None);
        assert_eq!(registry.find_by_id("missing"), None);
    }

    #[test]
    fn counts_with_deterministic_maps() {
        let registry = FriendRegistry::all();
        let technology_counts = registry.count_by_technology_kind();
        let identity_counts = registry.count_by_identity_kind();

        assert_eq!(
            technology_counts.get(&TechnologyKind::ProgrammingLanguage),
            Some(&12)
        );
        assert_eq!(
            technology_counts.get(&TechnologyKind::OperatingSystem),
            Some(&7)
        );
        assert_eq!(identity_counts.get(&IdentityKind::Mascot), Some(&20));
        assert_eq!(identity_counts.get(&IdentityKind::LogoSymbol), Some(&4));
    }
}

#![doc = include_str!("../README.md")]

//! Core data model for RustUse Fellow Friends fixture records.

use std::fmt;

/// A typed metadata record for one Fellow Friend fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Friend {
    /// Stable fixture identifier.
    pub id: &'static str,
    /// Human-readable display name.
    pub name: &'static str,
    /// Technology ecosystem associated with the record.
    pub ecosystem: &'static str,
    /// Broad technology category for the ecosystem.
    pub technology_kind: TechnologyKind,
    /// Broad identity category for the friend or symbol.
    pub identity_kind: IdentityKind,
    /// Broad figure category for the visual or symbolic form.
    pub figure_kind: FigureKind,
    /// Short neutral form label, when one is useful.
    pub form: Option<&'static str>,
    /// Stable lowercase tags for examples and filtering.
    pub tags: &'static [&'static str],
    /// Short conservative note suitable for docs and sample data.
    pub notes: &'static str,
}

impl Friend {
    /// Returns `true` when this friend has a tag matching `tag`.
    ///
    /// Matching trims the input and uses ASCII case-insensitive comparison.
    #[must_use]
    pub fn has_tag(&self, tag: &str) -> bool {
        let normalized = tag.trim();

        !normalized.is_empty()
            && self
                .tags
                .iter()
                .any(|candidate| candidate.eq_ignore_ascii_case(normalized))
    }

    /// Returns `true` when this friend belongs to `ecosystem`.
    ///
    /// Matching trims the input and uses ASCII case-insensitive comparison.
    #[must_use]
    pub fn matches_ecosystem(&self, ecosystem: &str) -> bool {
        let normalized = ecosystem.trim();

        !normalized.is_empty() && self.ecosystem.eq_ignore_ascii_case(normalized)
    }

    /// Returns the stable slug for this fixture record.
    #[must_use]
    pub const fn slug(&self) -> &'static str {
        self.id
    }
}

/// Broad technology category for a Fellow Friend fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TechnologyKind {
    /// Programming language ecosystem.
    ProgrammingLanguage,
    /// Browser ecosystem.
    Browser,
    /// Operating system ecosystem.
    OperatingSystem,
    /// Database ecosystem.
    Database,
    /// Developer tool ecosystem.
    DevTool,
    /// Platform ecosystem.
    Platform,
    /// Framework ecosystem.
    Framework,
    /// Runtime ecosystem.
    Runtime,
    /// Editor ecosystem.
    Editor,
    /// Package manager ecosystem.
    PackageManager,
    /// Community ecosystem.
    Community,
    /// Unknown or intentionally unspecified technology category.
    Unknown,
}

impl fmt::Display for TechnologyKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(technology_kind_label(*self))
    }
}

/// Broad identity category for a Fellow Friend fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum IdentityKind {
    /// Mascot identity.
    Mascot,
    /// Unofficial mascot identity.
    UnofficialMascot,
    /// Logo character identity.
    LogoCharacter,
    /// Logo symbol identity.
    LogoSymbol,
    /// Companion character identity.
    CompanionCharacter,
    /// Community character identity.
    CommunityCharacter,
    /// Symbol identity.
    Symbol,
    /// Unknown or intentionally unspecified identity category.
    Unknown,
}

impl fmt::Display for IdentityKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(identity_kind_label(*self))
    }
}

/// Broad figure category for a Fellow Friend fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FigureKind {
    /// Animal figure.
    Animal,
    /// Creature figure.
    Creature,
    /// Object figure.
    Object,
    /// Symbol figure.
    Symbol,
    /// Human-like figure.
    HumanLike,
    /// Abstract figure.
    Abstract,
    /// Unknown or intentionally unspecified figure category.
    Unknown,
}

impl fmt::Display for FigureKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(figure_kind_label(*self))
    }
}

/// Owned serde-friendly representation of a Fellow Friend record.
///
/// `Friend` stores static fixture references. `FriendRecord` is available behind
/// the `serde` feature for examples that need to deserialize owned sample data.
#[cfg(feature = "serde")]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FriendRecord {
    /// Stable fixture identifier.
    pub id: String,
    /// Human-readable display name.
    pub name: String,
    /// Technology ecosystem associated with the record.
    pub ecosystem: String,
    /// Broad technology category for the ecosystem.
    pub technology_kind: TechnologyKind,
    /// Broad identity category for the friend or symbol.
    pub identity_kind: IdentityKind,
    /// Broad figure category for the visual or symbolic form.
    pub figure_kind: FigureKind,
    /// Short neutral form label, when one is useful.
    pub form: Option<String>,
    /// Stable tags for examples and filtering.
    pub tags: Vec<String>,
    /// Short conservative note suitable for docs and sample data.
    pub notes: String,
}

#[cfg(feature = "serde")]
impl From<&Friend> for FriendRecord {
    fn from(friend: &Friend) -> Self {
        Self {
            id: friend.id.to_owned(),
            name: friend.name.to_owned(),
            ecosystem: friend.ecosystem.to_owned(),
            technology_kind: friend.technology_kind,
            identity_kind: friend.identity_kind,
            figure_kind: friend.figure_kind,
            form: friend.form.map(str::to_owned),
            tags: friend.tags.iter().map(|tag| (*tag).to_owned()).collect(),
            notes: friend.notes.to_owned(),
        }
    }
}

#[cfg(feature = "serde")]
impl From<Friend> for FriendRecord {
    fn from(friend: Friend) -> Self {
        Self::from(&friend)
    }
}

const fn technology_kind_label(kind: TechnologyKind) -> &'static str {
    match kind {
        TechnologyKind::ProgrammingLanguage => "Programming language",
        TechnologyKind::Browser => "Browser",
        TechnologyKind::OperatingSystem => "Operating system",
        TechnologyKind::Database => "Database",
        TechnologyKind::DevTool => "Developer tool",
        TechnologyKind::Platform => "Platform",
        TechnologyKind::Framework => "Framework",
        TechnologyKind::Runtime => "Runtime",
        TechnologyKind::Editor => "Editor",
        TechnologyKind::PackageManager => "Package manager",
        TechnologyKind::Community => "Community",
        TechnologyKind::Unknown => "Unknown",
    }
}

const fn identity_kind_label(kind: IdentityKind) -> &'static str {
    match kind {
        IdentityKind::Mascot => "Mascot",
        IdentityKind::UnofficialMascot => "Unofficial mascot",
        IdentityKind::LogoCharacter => "Logo character",
        IdentityKind::LogoSymbol => "Logo symbol",
        IdentityKind::CompanionCharacter => "Companion character",
        IdentityKind::CommunityCharacter => "Community character",
        IdentityKind::Symbol => "Symbol",
        IdentityKind::Unknown => "Unknown",
    }
}

const fn figure_kind_label(kind: FigureKind) -> &'static str {
    match kind {
        FigureKind::Animal => "Animal",
        FigureKind::Creature => "Creature",
        FigureKind::Object => "Object",
        FigureKind::Symbol => "Symbol",
        FigureKind::HumanLike => "Human-like",
        FigureKind::Abstract => "Abstract",
        FigureKind::Unknown => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::{FigureKind, Friend, IdentityKind, TechnologyKind};

    const FERRIS: Friend = Friend {
        id: "rust-ferris",
        name: "Ferris",
        ecosystem: "Rust",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::UnofficialMascot,
        figure_kind: FigureKind::Animal,
        form: Some("crab"),
        tags: &["rust", "crab", "community"],
        notes: "Friendly crab associated with the Rust community.",
    };

    #[test]
    fn helper_methods_match_trimmed_ascii_case_insensitive_values() {
        assert!(FERRIS.has_tag(" community "));
        assert!(FERRIS.has_tag("CRAB"));
        assert!(!FERRIS.has_tag(""));
        assert!(FERRIS.matches_ecosystem(" rust "));
        assert!(!FERRIS.matches_ecosystem("Go"));
        assert_eq!(FERRIS.slug(), "rust-ferris");
    }

    #[test]
    fn display_labels_are_human_readable() {
        assert_eq!(
            TechnologyKind::ProgrammingLanguage.to_string(),
            "Programming language"
        );
        assert_eq!(
            TechnologyKind::PackageManager.to_string(),
            "Package manager"
        );
        assert_eq!(
            IdentityKind::UnofficialMascot.to_string(),
            "Unofficial mascot"
        );
        assert_eq!(IdentityKind::LogoCharacter.to_string(), "Logo character");
        assert_eq!(FigureKind::HumanLike.to_string(), "Human-like");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn serde_feature_exposes_serialize_and_owned_deserialize_shapes() {
        use super::FriendRecord;

        fn assert_serialize<T: serde::Serialize>() {}
        fn assert_deserialize<'de, T: serde::Deserialize<'de>>() {}

        assert_serialize::<Friend>();
        assert_serialize::<TechnologyKind>();
        assert_deserialize::<TechnologyKind>();
        assert_deserialize::<FriendRecord>();

        let record = FriendRecord::from(FERRIS);

        assert_eq!(record.id, "rust-ferris");
        assert_eq!(record.tags, ["rust", "crab", "community"]);
    }
}

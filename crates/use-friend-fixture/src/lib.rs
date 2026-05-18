#![doc = include_str!("../README.md")]

//! Deterministic static Fellow Friends fixture records.

use use_friend::{FigureKind, Friend, IdentityKind, TechnologyKind};

/// Accessor type for deterministic Fellow Friends fixture slices.
pub struct FriendFixtures;

impl FriendFixtures {
    /// Returns the smallest useful fixture slice.
    #[must_use]
    pub fn tiny() -> &'static [Friend] {
        &ALL_FRIENDS[..3]
    }

    /// Returns a small fixture slice suitable for docs and examples.
    #[must_use]
    pub fn small() -> &'static [Friend] {
        &ALL_FRIENDS[..6]
    }

    /// Returns all bundled Fellow Friends fixture records.
    #[must_use]
    pub fn all() -> &'static [Friend] {
        &ALL_FRIENDS
    }
}

const ALL_FRIENDS: [Friend; 34] = [
    Friend {
        id: "rust-ferris",
        name: "Ferris",
        ecosystem: "Rust",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::UnofficialMascot,
        figure_kind: FigureKind::Animal,
        form: Some("crab"),
        tags: &["rust", "crab", "community", "mascot"],
        notes: "Friendly crab associated with the Rust community.",
    },
    Friend {
        id: "go-gopher",
        name: "Go Gopher",
        ecosystem: "Go",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("gopher"),
        tags: &["go", "gopher", "community", "mascot"],
        notes: "Gopher character commonly associated with the Go ecosystem.",
    },
    Friend {
        id: "linux-tux",
        name: "Tux",
        ecosystem: "Linux",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("penguin"),
        tags: &["linux", "penguin", "operating-system", "mascot"],
        notes: "Penguin mascot associated with Linux examples and communities.",
    },
    Friend {
        id: "postgresql-slonik",
        name: "Slonik",
        ecosystem: "PostgreSQL",
        technology_kind: TechnologyKind::Database,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Animal,
        form: Some("elephant"),
        tags: &["postgresql", "database", "elephant", "logo-character"],
        notes: "Elephant character associated with PostgreSQL sample data.",
    },
    Friend {
        id: "docker-moby-dock",
        name: "Moby Dock",
        ecosystem: "Docker",
        technology_kind: TechnologyKind::Platform,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Animal,
        form: Some("whale"),
        tags: &["docker", "container", "whale", "logo-character"],
        notes: "Whale character associated with Docker container examples.",
    },
    Friend {
        id: "github-octocat",
        name: "Octocat",
        ecosystem: "GitHub",
        technology_kind: TechnologyKind::Platform,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Creature,
        form: Some("cat/octopus hybrid"),
        tags: &["github", "platform", "hybrid", "logo-character"],
        notes: "Hybrid character associated with GitHub-themed fixture examples.",
    },
    Friend {
        id: "firefox-fox",
        name: "Firefox Fox",
        ecosystem: "Firefox",
        technology_kind: TechnologyKind::Browser,
        identity_kind: IdentityKind::LogoSymbol,
        figure_kind: FigureKind::Animal,
        form: Some("fox"),
        tags: &["firefox", "browser", "fox", "logo-symbol"],
        notes: "Fox-like symbol associated with Firefox browser examples.",
    },
    Friend {
        id: "python-snakes",
        name: "Python snakes",
        ecosystem: "Python",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::LogoSymbol,
        figure_kind: FigureKind::Animal,
        form: Some("snakes"),
        tags: &["python", "snakes", "programming-language", "logo-symbol"],
        notes: "Snake symbols associated with Python ecosystem examples.",
    },
    Friend {
        id: "openbsd-puffy",
        name: "Puffy",
        ecosystem: "OpenBSD",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("pufferfish"),
        tags: &["openbsd", "pufferfish", "operating-system", "mascot"],
        notes: "Pufferfish mascot associated with OpenBSD examples.",
    },
    Friend {
        id: "bsd-beastie",
        name: "Beastie",
        ecosystem: "BSD",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Creature,
        form: Some("daemon"),
        tags: &["bsd", "daemon", "operating-system", "mascot"],
        notes: "Daemon character associated with BSD-themed fixture data.",
    },
    Friend {
        id: "gimp-wilber",
        name: "Wilber",
        ecosystem: "GIMP",
        technology_kind: TechnologyKind::DevTool,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Creature,
        form: Some("animal-like mascot"),
        tags: &["gimp", "graphics", "dev-tool", "mascot"],
        notes: "Animal-like mascot associated with GIMP examples.",
    },
    Friend {
        id: "kde-konqi",
        name: "Konqi",
        ecosystem: "KDE",
        technology_kind: TechnologyKind::Platform,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Creature,
        form: Some("dragon"),
        tags: &["kde", "desktop", "dragon", "mascot"],
        notes: "Dragon mascot associated with KDE community examples.",
    },
    Friend {
        id: "java-duke",
        name: "Duke",
        ecosystem: "Java",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Creature,
        form: Some("creature"),
        tags: &["java", "jvm", "language", "mascot"],
        notes: "Mascot associated with Java.",
    },
    Friend {
        id: "php-elephpant",
        name: "elePHPant",
        ecosystem: "PHP",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("elephant"),
        tags: &["php", "web", "language", "elephant", "mascot"],
        notes: "Elephant mascot associated with PHP.",
    },
    Friend {
        id: "mysql-sakila",
        name: "Sakila",
        ecosystem: "MySQL",
        technology_kind: TechnologyKind::Database,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Animal,
        form: Some("dolphin"),
        tags: &["mysql", "database", "sql", "dolphin", "logo-character"],
        notes: "Dolphin logo character associated with MySQL.",
    },
    Friend {
        id: "mariadb-sea-lion",
        name: "MariaDB Sea Lion",
        ecosystem: "MariaDB",
        technology_kind: TechnologyKind::Database,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Animal,
        form: Some("sea lion"),
        tags: &["mariadb", "database", "sql", "sea-lion", "logo-character"],
        notes: "Sea lion logo character associated with MariaDB.",
    },
    Friend {
        id: "zig-zero-ziguana",
        name: "Zero the Ziguana",
        ecosystem: "Zig",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("iguana"),
        tags: &["zig", "systems", "language", "iguana", "mascot"],
        notes: "Iguana mascot associated with Zig.",
    },
    Friend {
        id: "zig-ziggy-ziguana",
        name: "Ziggy the Ziguana",
        ecosystem: "Zig",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("iguana"),
        tags: &["zig", "systems", "language", "iguana", "mascot"],
        notes: "Iguana mascot associated with Zig.",
    },
    Friend {
        id: "zig-carmen-allocgator",
        name: "Carmen the Allocgator",
        ecosystem: "Zig",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("alligator"),
        tags: &[
            "zig",
            "systems",
            "language",
            "allocator",
            "alligator",
            "mascot",
        ],
        notes: "Alligator mascot associated with Zig allocation examples.",
    },
    Friend {
        id: "dart-flutter-dash",
        name: "Dash",
        ecosystem: "Dart / Flutter",
        technology_kind: TechnologyKind::Framework,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("hummingbird"),
        tags: &["dart", "flutter", "ui", "mobile", "hummingbird", "mascot"],
        notes: "Mascot associated with Dart and Flutter.",
    },
    Friend {
        id: "dotnet-bot",
        name: "dotnet bot",
        ecosystem: ".NET",
        technology_kind: TechnologyKind::Framework,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Creature,
        form: Some("robot"),
        tags: &["dotnet", "framework", "runtime", "robot", "mascot"],
        notes: "Robot mascot associated with .NET.",
    },
    Friend {
        id: "plan-9-glenda",
        name: "Glenda",
        ecosystem: "Plan 9",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("bunny"),
        tags: &["plan-9", "operating-system", "bell-labs", "bunny", "mascot"],
        notes: "Bunny mascot associated with Plan 9.",
    },
    Friend {
        id: "gnu-head",
        name: "GNU Head",
        ecosystem: "GNU",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::LogoSymbol,
        figure_kind: FigureKind::Animal,
        form: Some("gnu"),
        tags: &["gnu", "free-software", "operating-system", "symbol"],
        notes: "GNU head symbol associated with the GNU project.",
    },
    Friend {
        id: "android-bot",
        name: "Android Bot",
        ecosystem: "Android",
        technology_kind: TechnologyKind::Platform,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Creature,
        form: Some("robot"),
        tags: &["android", "mobile", "platform", "robot", "logo-character"],
        notes: "Robot logo character associated with Android.",
    },
    Friend {
        id: "raku-camelia",
        name: "Camelia",
        ecosystem: "Raku",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("butterfly"),
        tags: &["raku", "language", "butterfly", "mascot"],
        notes: "Butterfly mascot associated with Raku.",
    },
    Friend {
        id: "perl-camel",
        name: "Perl Camel",
        ecosystem: "Perl",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Symbol,
        figure_kind: FigureKind::Animal,
        form: Some("camel"),
        tags: &["perl", "language", "camel", "symbol", "historical"],
        notes: "Camel symbol commonly associated with Perl.",
    },
    Friend {
        id: "deno-dinosaur",
        name: "Deno Dinosaur",
        ecosystem: "Deno",
        technology_kind: TechnologyKind::Runtime,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("dinosaur"),
        tags: &[
            "deno",
            "runtime",
            "typescript",
            "javascript",
            "dinosaur",
            "mascot",
        ],
        notes: "Dinosaur mascot associated with Deno.",
    },
    Friend {
        id: "crystal-crow",
        name: "Crystal Crow",
        ecosystem: "Crystal",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("crow"),
        tags: &["crystal", "language", "compiled", "crow", "mascot"],
        notes: "Crow mascot associated with Crystal.",
    },
    Friend {
        id: "opensuse-geeko",
        name: "Geeko",
        ecosystem: "openSUSE",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("gecko"),
        tags: &["opensuse", "suse", "linux", "gecko", "mascot"],
        notes: "Gecko mascot associated with openSUSE.",
    },
    Friend {
        id: "linux-libre-freedo",
        name: "Freedo",
        ecosystem: "Linux-libre",
        technology_kind: TechnologyKind::OperatingSystem,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::Animal,
        form: Some("penguin"),
        tags: &[
            "linux-libre",
            "free-software",
            "kernel",
            "penguin",
            "mascot",
        ],
        notes: "Penguin mascot associated with Linux-libre.",
    },
    Friend {
        id: "d-d-man",
        name: "D-Man",
        ecosystem: "D",
        technology_kind: TechnologyKind::ProgrammingLanguage,
        identity_kind: IdentityKind::Mascot,
        figure_kind: FigureKind::HumanLike,
        form: Some("character"),
        tags: &["d", "language", "systems", "mascot"],
        notes: "Mascot character associated with the D programming language.",
    },
    Friend {
        id: "nix-nixos-snowflake",
        name: "Nix Snowflake",
        ecosystem: "Nix / NixOS",
        technology_kind: TechnologyKind::PackageManager,
        identity_kind: IdentityKind::LogoSymbol,
        figure_kind: FigureKind::Symbol,
        form: Some("snowflake"),
        tags: &[
            "nix",
            "nixos",
            "package-manager",
            "builds",
            "snowflake",
            "symbol",
        ],
        notes: "Snowflake symbol associated with Nix and NixOS.",
    },
    Friend {
        id: "jenkins-butler",
        name: "Jenkins Butler",
        ecosystem: "Jenkins",
        technology_kind: TechnologyKind::DevTool,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::HumanLike,
        form: Some("butler"),
        tags: &[
            "jenkins",
            "ci",
            "cd",
            "automation",
            "butler",
            "logo-character",
        ],
        notes: "Butler logo character associated with Jenkins.",
    },
    Friend {
        id: "apache-hadoop-elephant",
        name: "Hadoop Elephant",
        ecosystem: "Apache Hadoop",
        technology_kind: TechnologyKind::Platform,
        identity_kind: IdentityKind::LogoCharacter,
        figure_kind: FigureKind::Animal,
        form: Some("elephant"),
        tags: &["hadoop", "apache", "big-data", "elephant", "logo-character"],
        notes: "Elephant logo character associated with Apache Hadoop.",
    },
];

#[cfg(test)]
mod tests {
    use super::FriendFixtures;
    use use_friend::{FigureKind, IdentityKind, TechnologyKind};

    #[test]
    fn exposes_expected_fixture_lengths() {
        assert_eq!(FriendFixtures::tiny().len(), 3);
        assert_eq!(FriendFixtures::small().len(), 6);
        assert_eq!(FriendFixtures::all().len(), 34);
    }

    #[test]
    fn preserves_deterministic_order() {
        let friends = FriendFixtures::all();

        assert_eq!(friends[0].id, "rust-ferris");
        assert_eq!(friends[1].id, "go-gopher");
        assert_eq!(friends[2].id, "linux-tux");
        assert_eq!(friends[11].id, "kde-konqi");
        assert_eq!(friends[33].id, "apache-hadoop-elephant");
    }

    #[test]
    fn includes_conservative_expected_metadata() {
        let ferris = &FriendFixtures::all()[0];
        let octocat = &FriendFixtures::all()[5];

        assert_eq!(ferris.ecosystem, "Rust");
        assert_eq!(ferris.form, Some("crab"));
        assert_eq!(ferris.identity_kind, IdentityKind::UnofficialMascot);
        assert_eq!(ferris.technology_kind, TechnologyKind::ProgrammingLanguage);
        assert_eq!(octocat.figure_kind, FigureKind::Creature);
        assert!(FriendFixtures::all()
            .iter()
            .all(|friend| !friend.notes.contains("http")));
    }

    #[test]
    fn includes_new_fixture_batch() {
        let ids: Vec<_> = FriendFixtures::all()
            .iter()
            .map(|friend| friend.id)
            .collect();
        let expected_ids = [
            "java-duke",
            "php-elephpant",
            "mysql-sakila",
            "mariadb-sea-lion",
            "zig-zero-ziguana",
            "zig-ziggy-ziguana",
            "zig-carmen-allocgator",
            "dart-flutter-dash",
            "dotnet-bot",
            "plan-9-glenda",
            "gnu-head",
            "android-bot",
            "raku-camelia",
            "perl-camel",
            "deno-dinosaur",
            "crystal-crow",
            "opensuse-geeko",
            "linux-libre-freedo",
            "d-d-man",
            "nix-nixos-snowflake",
            "jenkins-butler",
            "apache-hadoop-elephant",
        ];

        for expected_id in expected_ids {
            assert!(ids.contains(&expected_id));
        }
    }

    #[test]
    fn keeps_zig_records_in_request_order() {
        let zig_names: Vec<_> = FriendFixtures::all()
            .iter()
            .filter(|friend| friend.matches_ecosystem("Zig"))
            .map(|friend| friend.name)
            .collect();

        assert_eq!(
            zig_names,
            [
                "Zero the Ziguana",
                "Ziggy the Ziguana",
                "Carmen the Allocgator",
            ]
        );
    }
}

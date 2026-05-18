#![doc = include_str!("../README.md")]

//! Simple deterministic markdown rendering for Fellow Friends records.

use use_friend::Friend;

/// Markdown rendering helpers for Fellow Friends records.
pub struct FriendMarkdown;

impl FriendMarkdown {
    /// Renders one friend as a compact markdown card.
    #[must_use]
    pub fn card(friend: &Friend) -> String {
        let mut markdown = format!(
            "### {}\n\n- Ecosystem: {}\n- Technology kind: {}\n- Identity kind: {}\n- Figure: {}",
            friend.name,
            friend.ecosystem,
            friend.technology_kind,
            friend.identity_kind,
            friend.figure_kind
        );

        if let Some(form) = friend.form {
            markdown.push_str(&format!("\n- Form: {form}"));
        }

        markdown.push('\n');
        markdown
    }

    /// Renders friends as a simple markdown table.
    #[must_use]
    pub fn table(friends: &[Friend]) -> String {
        let mut markdown = String::from(
            "| Name | Ecosystem | Technology kind | Identity kind | Figure | Form |\n| --- | --- | --- | --- | --- | --- |\n",
        );

        for friend in friends {
            markdown.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                friend.name,
                friend.ecosystem,
                friend.technology_kind,
                friend.identity_kind,
                friend.figure_kind,
                friend.form.unwrap_or("unknown")
            ));
        }

        markdown
    }

    /// Renders friends as a simple markdown list.
    #[must_use]
    pub fn list(friends: &[Friend]) -> String {
        let mut markdown = String::new();

        for friend in friends {
            markdown.push_str(&format!("- {} ({})\n", friend.name, friend.ecosystem));
        }

        markdown
    }
}

#[cfg(test)]
mod tests {
    use super::FriendMarkdown;
    use use_friend_fixture::FriendFixtures;

    #[test]
    fn renders_card_with_expected_lines() {
        let card = FriendMarkdown::card(&FriendFixtures::tiny()[0]);

        assert_eq!(
            card,
            "### Ferris\n\n- Ecosystem: Rust\n- Technology kind: Programming language\n- Identity kind: Unofficial mascot\n- Figure: Animal\n- Form: crab\n"
        );
    }

    #[test]
    fn renders_card_for_new_friend() {
        let duke = FriendFixtures::all()
            .iter()
            .find(|friend| friend.id == "java-duke")
            .expect("Duke should be in the fixture set");
        let card = FriendMarkdown::card(duke);

        assert_eq!(
            card,
            "### Duke\n\n- Ecosystem: Java\n- Technology kind: Programming language\n- Identity kind: Mascot\n- Figure: Creature\n- Form: creature\n"
        );
    }

    #[test]
    fn renders_deterministic_table() {
        let table = FriendMarkdown::table(FriendFixtures::tiny());

        assert!(table
            .starts_with("| Name | Ecosystem | Technology kind | Identity kind | Figure | Form |"));
        assert!(table.contains(
            "| Ferris | Rust | Programming language | Unofficial mascot | Animal | crab |"
        ));
        assert!(table.contains("| Tux | Linux | Operating system | Mascot | Animal | penguin |"));
    }

    #[test]
    fn renders_deterministic_list() {
        assert_eq!(
            FriendMarkdown::list(FriendFixtures::tiny()),
            "- Ferris (Rust)\n- Go Gopher (Go)\n- Tux (Linux)\n"
        );
    }
}

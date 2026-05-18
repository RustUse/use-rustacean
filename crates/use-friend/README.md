# use-friend

`use-friend` defines the core Fellow Friends data model for RustUse fixture records.

It provides a small `Friend` struct, compact taxonomy enums, helper methods, display labels, and optional serde support for serialization fixtures. The crate contains metadata primitives only; no artwork, logos, SVGs, images, or binary assets are included.

## Example

```rust
use use_friend::{FigureKind, Friend, IdentityKind, TechnologyKind};

let ferris = Friend {
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

assert!(ferris.matches_ecosystem("rust"));
assert!(ferris.has_tag("community"));
assert_eq!(ferris.slug(), "rust-ferris");
```

## License

Licensed under `MIT OR Apache-2.0`.
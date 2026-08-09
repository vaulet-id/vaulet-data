//! Vaulet's reference data, embedded at compile time.
//!
//! Two kinds, and both are public for the same reason: a person has to be able
//! to check what was done in their name. A verifier reading a signature years
//! from now must see the exact sentence it covered, and somebody handed an
//! address credential must be able to read the list it was checked against.
//!
//! **This crate is the one copy.** The issuer used to carry its own and serve
//! that; it takes this by revision instead, because two datasets agree until
//! one of them does not.

/// One country's address tree, as authored — the bytes, not a re-serialised
/// copy of them.
///
/// The distinction matters: the issuer signs a digest of what it sends, and the
/// wallet checks that digest against the bytes it received (ADR 0031). Parsing
/// and re-emitting anywhere in that path would break a signature over data that
/// nobody had altered.
pub fn address(country: &str) -> Option<&'static str> {
    match country {
        "TH" => Some(include_str!("../address/TH.json")),
        _ => None,
    }
}

/// Every country with a tree here.
pub const ADDRESS_COUNTRIES: &[&str] = &["TH"];

/// A statement template, by act and version — the wording a person reads before
/// signing, per language.
///
/// A template that has been signed against is never edited; a correction is a
/// new version, so statements signed under the old wording stay bound to the
/// words they were signed under and those words stay readable.
pub fn statement_template(act: &str, version: u32) -> Option<&'static str> {
    match (act, version) {
        ("authorise", 1) => Some(include_str!("../statements/authorise.v1.json")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_listed_country_has_a_tree() {
        for c in super::ADDRESS_COUNTRIES {
            let body = super::address(c).unwrap_or_else(|| panic!("{c} is listed and absent"));
            let v: serde_json::Value = serde_json::from_str(body).expect("parses");
            assert_eq!(v["country"], *c, "the file disagrees with its own name");
            assert!(!v["version"].as_str().unwrap().is_empty(), "a tree states its version");
        }
    }

    #[test]
    fn an_unknown_country_is_none_rather_than_a_panic() {
        assert!(super::address("XX").is_none());
        assert!(super::statement_template("authorise", 99).is_none());
    }
}

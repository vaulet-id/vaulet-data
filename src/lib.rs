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
        "US" => Some(include_str!("../address/US.json")),
        "JP" => Some(include_str!("../address/JP.json")),
        "CN" => Some(include_str!("../address/CN.json")),
        _ => None,
    }
}

/// Every country with a tree here.
pub const ADDRESS_COUNTRIES: &[&str] = &["TH", "US", "JP", "CN"];

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

            // `levels` is what a reader consults instead of inferring depth
            // from whether `l` happens to be present. A file that omitted it
            // would be read as three levels deep, which is true of one country
            // here and of none of the others.
            let levels = v["levels"].as_array().expect("a tree states its depth");
            assert!(!levels.is_empty(), "{c}: levels is empty");
            assert!(!v["require"].as_array().expect("require").is_empty(), "{c}: require is empty");
            let role = v["postal"]["role"].as_str().expect("postal.role");
            assert!(
                matches!(role, "leaf" | "pattern" | "determines"),
                "{c}: postal.role is {role}, which nothing knows how to read"
            );

            // The depth the file claims is the depth it has.
            let deepest = v["regions"].as_array().unwrap().iter()
                .map(|r| {
                    let l = r["l"].as_array().map(|x| x.len()).unwrap_or(0);
                    let d: usize = r["l"].as_array().map(|xs| xs.iter()
                        .map(|x| x["d"].as_array().map(|y| y.len()).unwrap_or(0)).sum()).unwrap_or(0);
                    if d > 0 { 3 } else if l > 0 { 2 } else { 1 }
                })
                .max().unwrap_or(1);
            assert_eq!(
                deepest, levels.len(),
                "{c}: says {} levels and carries {deepest}", levels.len()
            );

            // What each level is called here, as a token a reader translates —
            // never a word, because a consumer of this data is read in more
            // than one language and "State" is already English. The set is
            // closed on purpose: a country introducing a token nobody has a
            // word for would ship a picker with a blank heading over it.
            const NAMES: &[&str] = &[
                "province", "state", "prefecture", "district", "city",
                "sub_district", "postcode", "postal_code", "zip",
            ];
            let named = v["level_names"].as_object().expect("level_names");
            for (level, name) in named {
                let name = name.as_str().expect("a level name is a token");
                assert!(NAMES.contains(&name), "{c}: {level} is called {name}, which nothing can say");
            }
            for level in levels {
                let level = level.as_str().unwrap();
                assert!(named.contains_key(level), "{c}: walks {level} and does not name it");
            }
            assert!(named.contains_key("postal_code"), "{c}: has no word for its postal code");
        }
    }

    #[test]
    fn an_unknown_country_is_none_rather_than_a_panic() {
        assert!(super::address("XX").is_none());
        assert!(super::statement_template("authorise", 99).is_none());
    }
}

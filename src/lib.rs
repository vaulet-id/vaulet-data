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

/// Every country there is, by both ISO 3166-1 codes, named in each language
/// this system is read in.
///
/// **Two codes, because two things ask.** A passport's MRZ names its issuing
/// country in alpha-3 and an address names one in alpha-2, and a wallet that
/// held only one of them had to guess at the other. The alpha-3 English names
/// are ISO's own formal ones — the wording a travel document uses, so what a
/// screen shows beside a scanned passport matches what the passport says.
/// Thai comes from CLDR, which publishes no such formal register.
///
/// Bytes, unparsed, for the same reason as [`address`]: what is signed is what
/// is sent.
pub fn countries() -> &'static str {
    include_str!("../countries/countries.json")
}

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

    /// **English travels with every place name**, because a credential crosses
    /// borders and a name nobody at the far end can read is not a claim.
    ///
    /// The counts are pinned rather than asserted as "most of them": each one
    /// is what a named source published on 2026-08-09, and the gaps are named
    /// in SCHEMA.md. A drop means an import lost rows; a rise means somebody
    /// filled a gap and should say from where.
    #[test]
    fn every_level_carries_english_as_far_as_a_source_goes() {
        fn counted(cc: &str) -> (usize, usize, usize, usize, usize, usize) {
            let v: serde_json::Value =
                serde_json::from_str(super::address(cc).unwrap()).unwrap();
            let (mut l2, mut l2en, mut l3, mut l3en) = (0, 0, 0, 0);
            let regions = v["regions"].as_array().unwrap();
            for r in regions {
                for c in r["l"].as_array().unwrap_or(&vec![]) {
                    l2 += 1;
                    if !c["en"].as_str().unwrap_or("").is_empty() { l2en += 1 }
                    for d in c["d"].as_array().unwrap_or(&vec![]) {
                        l3 += 1;
                        if !d["en"].as_str().unwrap_or("").is_empty() { l3en += 1 }
                    }
                }
            }
            let r1 = regions.len();
            let r1en = regions.iter()
                .filter(|r| !r["en"].as_str().unwrap_or("").is_empty()).count();
            (r1, r1en, l2, l2en, l3, l3en)
        }

        // Thailand is complete at every level and is the shape the others are
        // measured against.
        assert_eq!(counted("TH"), (77, 77, 928, 928, 7436, 7436));
        assert_eq!(counted("US"), (62, 62, 0, 0, 0, 0));
        // One municipality is absent from the romaji source altogether.
        assert_eq!(counted("JP"), (47, 47, 1895, 1894, 0, 0));
        // Development zones and county-level cities the latin source does not
        // publish; see SCHEMA.md, where each gap is named.
        assert_eq!(counted("CN"), (31, 31, 342, 324, 2978, 2431));
    }

    /// Both codes on every row, both languages on every row.
    ///
    /// A country with one code is a country one of the two callers cannot look
    /// up, and a missing Thai name would fall back to English on a Thai screen
    /// — quietly, and only for the countries nobody tested.
    #[test]
    fn every_country_has_both_codes_and_both_names() {
        let v: serde_json::Value = serde_json::from_str(super::countries()).expect("parses");
        let rows = v["countries"].as_array().expect("countries");
        // ISO 3166-1 currently assigns 249 codes. Pinned so a truncated import
        // fails here rather than by silently not offering somewhere.
        assert_eq!(rows.len(), 249);
        let mut a2 = std::collections::HashSet::new();
        let mut a3 = std::collections::HashSet::new();
        for c in rows {
            let two = c["a2"].as_str().expect("alpha-2");
            let three = c["a3"].as_str().expect("alpha-3");
            assert_eq!(two.len(), 2, "{two} is not an alpha-2 code");
            assert_eq!(three.len(), 3, "{three} is not an alpha-3 code");
            assert!(a2.insert(two), "{two} appears twice");
            assert!(a3.insert(three), "{three} appears twice");
            for lang in v["languages"].as_array().unwrap() {
                let lang = lang.as_str().unwrap();
                assert!(
                    c["names"][lang].as_str().is_some_and(|s| !s.is_empty()),
                    "{two} has no {lang} name"
                );
            }
        }
        // The four with address trees must be in here, or a country picker
        // could offer one it cannot name.
        for cc in super::ADDRESS_COUNTRIES {
            assert!(a2.contains(cc), "{cc} has a tree and no name");
        }
    }

    #[test]
    fn an_unknown_country_is_none_rather_than_a_panic() {
        assert!(super::address("XX").is_none());
        assert!(super::statement_template("authorise", 99).is_none());
    }
}

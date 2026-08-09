# The address format, and why one shape will not do

`TH.json` was the only country here, and its shape is Thailand's: three
administrative levels, a postal code on every leaf, and every leaf named twice.
Adding the United States, Japan or China in that shape would have produced data
that parses, passes every test, and is wrong.

So the format carries the difference instead, and the four files declare it.
This document is why, and what a fifth country needs. It is written from what
the authorities publish — read on 2026-08-09, not from memory.

## What the four countries actually require

`require` is the set of fields an address is invalid without. `A` street,
`C` city or locality, `D` dependent locality, `S` state or province,
`Z` postal code.

| | requires | levels published | where the postal code sits |
|---|---|---|---|
| **TH** | — | 77 provinces → districts → subdistricts | on the leaf |
| **CN** | A C S Z | 31 provinces → cities → districts | separate from the tree |
| **US** | A C S Z | **62 states, and nothing below** | the ZIP determines the city |
| **JP** | A S Z | **47 prefectures, and nothing below** | the code determines everything below |

Two of the three break the Thai model outright:

**Japan does not require a city.** `require` is `ASZ` — there is no `C`. A
seven-digit code names the prefecture, the municipality and the town area
together; a person types the code and the rest fills itself. Thailand's tree
answers "which places are inside this district"; Japan's data answers "what
place is this code", which is the same information indexed the other way and not
convertible by rearranging it.

**The United States publishes no city list.** A city is free text and the ZIP
decides whether it is right. Building a tree would mean importing about 41,000
ZIP codes from the Census Bureau and inventing a hierarchy that the postal
service does not use — ZIPs cross county lines, and some cross state lines.

**China fits.** Province, city and district all exist and are published; the
postal code simply is not a property of a district the way it is of a Thai
subdistrict, so it is validated rather than looked up.

## What a format has to carry, therefore

Three things `TH.json` currently implies rather than states:

1. **How deep the tree goes**, per country — three levels, two, or none.
2. **Which fields are required**, so a form can refuse an address that is
   missing one rather than accepting something nobody can post to.
3. **Which direction the postal code runs** — a property of the leaf (TH), a
   free field validated by a pattern (US, CN), or the key that determines the
   place (JP).

`fields` already exists for the first and is doing half the job. `require`,
`postal_pattern` and something naming the postal code's role are new, and every
one of them is copied from an authority rather than decided here.

## The shape

```json
{
  "country": "JP",
  "languages": ["ja"],
  "name": { "en": "Japan", "local": "日本" },
  "version": "JP-…",

  "require": ["street_address", "region", "postal_code"],
  "fields":  ["region", "locality", "street_address", "postal_code"],
  "postal": { "pattern": "\\d{3}-?\\d{4}", "role": "determines" },

  "levels": ["region"],
  "regions": [ { "c": "13", "local": "東京都", "en": "Tokyo" } ]
}
```

- **`levels`** names the depth actually present, so a reader does not infer it
  from whether `l` happens to be there. `TH` and `CN` are
  `["region","locality","dependent_locality"]`, `JP` is `["region","locality"]`,
  `US` is `["region"]`. **A test asserts that the depth a file claims is the
  depth it has** — the alternative is a file that lies quietly.
- **`structure_levels`** is what the administrative hierarchy has, which is not
  always what an address uses. Only the United States differs today, and that
  difference is the whole reason the key exists.
- **`postal.role`** is `leaf` (TH: the code hangs off the last level),
  `pattern` (US, CN: validated, not looked up) or `determines` (JP: the code is
  the lookup key and the tree below `region` does not exist here).
- **`require`** is the authority's, spelled out instead of encoded.
- `regions`/`l`/`d` keep their current shape where they exist, so `TH.json`
  needs only the three new keys and no restructuring.

## What this does not solve, and should not pretend to

A wallet that can render an address form for four countries is a different piece
of work from a repository that holds four datasets. The renderer needs the field
*order* too — `libaddressinput`'s `fmt` string, which is where the difference
between `%C, %S %Z` and `〒%Z%n%S%n%A` lives — and ADR 0031's three tiers are
about which claims a credential carries, not how a form is laid out.

**The checker in `vaulet-core` and the form in the wallet do not read `levels`
or `require` yet.** Until they do, the four files are correct and only Thailand
is consulted correctly — a dataset whose depth nothing reads will be treated as
three levels deep. That work is the next thing, not this.

## What was actually added, 2026-08-09

| | levels | level 1 | level 2 | level 3 | postal role |
|---|---|---|---|---|---|
| TH | 3 | 77 provinces | 928 districts | 7,436 subdistricts | `leaf` |
| CN | 3 | 31 provinces | 342 cities | 2,978 districts | `pattern` |
| JP | 2 | 47 prefectures | 1,895 municipalities | — | `determines` |
| US | 1 | 62 states | — | — | `pattern` |

**The United States carries counties, and they are not part of the address.**
3,234 of them, under `counties` rather than `l`, and `levels` says `["region"]`
while `structure_levels` says `["region","county"]`. The distinction is the
point: a county is real, it decides voting districts and property tax and court
jurisdiction, and **it is not written on an envelope**. A form that asked for it
would be asking most Americans for something they do not read on their own post,
and a wrong county on a credential is worse than an absent one. Deriving it from
a ZIP does not rescue it either: a ZIP can span two counties and occasionally
two states, so the derivation is a guess wearing a fact's clothes.

If a verifier ever needs a county, it should be a claim with its own provenance
and its own confidence, not a field smuggled into an address.

**Japan's municipalities have no romaji here, and China's have partial pinyin.**
1,895 Japanese entries carry `local` and an empty `en`; 324 of 342 Chinese cities
have a latin name and none of the 2,978 districts do. The gap is left visible
rather than filled by transliterating locally — a name this repository invented
would be indistinguishable from one an authority published, and the whole
argument for publishing this data is that somebody can check it.

## Sources

- Field requirements, postal patterns and level-1 lists:
  Google `libaddressinput`, `https://chromium-i18n.appspot.com/ssl-address/data/<CC>`.
- Full trees, if they are ever wanted: Japan Post `KEN_ALL` (~124,000 rows),
  US Census Bureau ZIP data (~41,000), China's National Bureau of Statistics
  division codes (~3,000). All larger than Thailand's 7,436 by a lot, and none
  of them shaped like it.

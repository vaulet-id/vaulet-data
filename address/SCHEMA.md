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
  "level_names": { "region": "prefecture", "postal_code": "postal_code" },
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
- **`level_names`** is what this country calls each level, **as a token and
  never as a word** — `state`, `prefecture`, `sub_district`. A consumer of this
  data is read in more than one language, so a file carrying "State" would be
  carrying English onto a Thai screen. The token set is closed and a test says
  so: a country introducing one nobody has a word for would ship a picker with a
  blank heading over its list. The values come from `libaddressinput`'s
  `state_name_type` and its siblings, **except Thailand's**, which that source
  leaves null — meaning its generic defaults, "City" and "Suburb", where Thai
  addresses use district and sub-district and the wallet already said so.
- **`require`** is the authority's, spelled out instead of encoded.
- `regions`/`l`/`d` keep their current shape where they exist, so `TH.json`
  needs only the three new keys and no restructuring.

## What this does not solve, and should not pretend to

A wallet that can render an address form for four countries is a different piece
of work from a repository that holds four datasets. The renderer needs the field
*order* too — `libaddressinput`'s `fmt` string, which is where the difference
between `%C, %S %Z` and `〒%Z%n%S%n%A` lives — and ADR 0031's three tiers are
about which claims a credential carries, not how a form is laid out.

**The wallet reads `levels`, `postal.role` and `level_names` since 2026-08-09.**
Its picker walks the steps a country declares and calls them what that country
calls them, and where the postal code is typed rather than picked it offers a
field for it. `require` and `postal.pattern` still have no consumer: nothing
validates an address against them, in the wallet or in `vaulet-core`. They are
carried rather than parsed, so that whoever writes that validation reads the
authority instead of re-deriving it.

The renderer's field *order* is still missing — `libaddressinput`'s `fmt`, which
is where `%C, %S %Z` differs from `〒%Z%n%S%n%A`. The wallet does not lay an
address out; it collects the parts and keeps a free-text block beside them.

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

## English on every level, 2026-08-09

**A credential crosses borders, so every place name carries English** — that is
the rule the wallet is built on, and Japan and China were shipped breaking it:
all 1,895 Japanese municipalities and all 2,978 Chinese districts had `local`
and an empty `en`.

Filled from the same authorities the trees came from, never by transliterating
here. A name this repository invented would be indistinguishable from one an
authority published, and the argument for publishing this data at all is that
somebody can check it.

| | filled | from | left empty |
|---|---|---|---|
| JP municipalities | 1,894 / 1,895 | geolonia `市区町村名ローマ字` | 熊本県 球磨郡湯前町 |
| CN districts | 2,431 / 2,978 | `libaddressinput` `sub_lnames` | 547 |
| CN cities | 324 / 342 | as before | 18 |

**Named, so the gaps are checkable rather than approximate.** The one Japanese
municipality is absent from that CSV altogether — Kuma-gun has nine and it lists
eight — and `libaddressinput` publishes nothing below the prefecture, so there
is no second source to ask.

China's 547 are mostly development zones (`开发区`, `园区`, `管理区`) and a few
recently promoted county-level cities that the National Bureau of Statistics
carries and `libaddressinput` does not; 209 more sit inside eighteen cities that
source has no entry for at all, which are the same eighteen with no latin name of
their own — `市辖区` and `省直辖县级行政区划`, administrative categories rather
than places.

## Sources

- Field requirements, postal patterns and level-1 lists:
  Google `libaddressinput`, `https://chromium-i18n.appspot.com/ssl-address/data/<CC>`.
- Full trees, if they are ever wanted: Japan Post `KEN_ALL` (~124,000 rows),
  US Census Bureau ZIP data (~41,000), China's National Bureau of Statistics
  division codes (~3,000). All larger than Thailand's 7,436 by a lot, and none
  of them shaped like it.

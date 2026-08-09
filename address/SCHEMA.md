# The address format, and why one shape will not do

`TH.json` is the only country here, and its shape is Thailand's: three
administrative levels, a postal code on every leaf, and every leaf named twice.
Adding the United States, Japan or China in that shape would produce data that
parses, passes every test, and is wrong.

This document is what a second country needs before it is added. It is written
from Google's `libaddressinput` — the dataset Chrome and Android use to lay out
address forms — read on 2026-08-09, not from memory.

## What the four countries actually require

`require` is the set of fields an address is invalid without. `A` street,
`C` city or locality, `D` dependent locality, `S` state or province,
`Z` postal code.

| | requires | levels published | where the postal code sits |
|---|---|---|---|
| **TH** | — | 77 provinces → districts → subdistricts | on the leaf |
| **CN** | A C S Z | 34 provinces → districts | separate from the tree |
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

**China fits, one level shallower.** Province and district exist; the postal code
is not a property of a district the way it is of a Thai subdistrict.

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

## The shape being proposed

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
  from whether `l` happens to be there. `TH` is
  `["region","locality","dependent_locality"]`, `CN` is `["region","locality"]`,
  `US` and `JP` are `["region"]`.
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

**Do not add a country's data before the checker in `vaulet-core` and the form
in the wallet both read `levels` and `require`.** A dataset whose depth nothing
consults is a dataset that will be trusted to be three levels deep.

## Sources

- Field requirements, postal patterns and level-1 lists:
  Google `libaddressinput`, `https://chromium-i18n.appspot.com/ssl-address/data/<CC>`.
- Full trees, if they are ever wanted: Japan Post `KEN_ALL` (~124,000 rows),
  US Census Bureau ZIP data (~41,000), China's National Bureau of Statistics
  division codes (~3,000). All larger than Thailand's 7,436 by a lot, and none
  of them shaped like it.

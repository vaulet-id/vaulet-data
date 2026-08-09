# The countries, and why both codes are here

Every country ISO 3166-1 assigns a code to — 249 of them — with **both** its
alpha-2 and alpha-3 code, and its name in each language this system is read in.

## Two codes, because two things ask

A passport's MRZ names its issuing country and the holder's nationality in
**alpha-3** (`THA`). An address names its country in **alpha-2** (`TH`), which
is what OpenID Connect's `address` claim and `libaddressinput` both use.

A wallet holding one of the two had to derive the other, and there is no rule
that derives them: `DEU` is `DE`, `CHE` is `CH`, and nothing about the letters
says so. So both are carried, from `codeMappings` in CLDR's supplemental data.

## Where the names come from, and why not from one place

| | source | why |
|---|---|---|
| `en` | ISO 3166-1 | The formal register — "Bolivia, Plurinational State of". This name appears beside a scanned passport, and it should read as the document reads. |
| `th` | CLDR 48.2.0 | ISO publishes no Thai. CLDR is the reference every operating system localises territories from. |

Mixed on purpose, and the mixture is the point: CLDR's English is written for
interfaces — "Congo - Kinshasa", "Antigua & Barbuda" — which is right for a
country dropdown and wrong next to a travel document that says
`DEMOCRATIC REPUBLIC OF THE CONGO`. Adopting it wholesale would have silently
reworded 49 of the 249.

## What is not here

**Codes that are not ISO 3166-1.** ICAO issues its own for travel documents that
belong to no country — `XXA` stateless, `XXB` and `XXC` refugees, `GBD`/`GBN`
and the other British subsets. A reader that does not find a code should show
the code, which is what a passport prints anyway.

**Any notion of which countries are supported.** This is the list of countries
that exist. `ADDRESS_COUNTRIES` is the list with address trees, and it is four.

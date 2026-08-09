# vaulet-data

Reference data for [Vaulet](https://vaulet.id): the sentences people are asked
to sign, and the place names an address is checked against.

It is public because it has to be. A person signs a sentence, and a verifier
reading that signature years later must be able to see the exact wording it
covered — including someone who has never heard of us, working offline, after we
are gone. Wording that can only be read by asking us is wording nobody can
audit.

## Statement templates

`statements/<act>.v<n>.json` — the wording a statement renders, per language.

```json
{
  "act": "authorise",
  "version": 1,
  "wording": {
    "en": "I authorise Vaulet to {scope} for {about} …",
    "th": "ข้าพเจ้ามอบอำนาจให้ Vaulet {scope} สำหรับ {about} …"
  }
}
```

A signed statement carries two things that are not copies of each other: a
symbol (`{"act":"authorise", …}`) that a program acts on, and the text a person
read. **Where the two disagree the statement is void** — not "the text wins",
because a program that acted on the symbol has already acted.

Two rules follow, and both are why this is data rather than code:

- **A template that has been signed against is never edited.** A correction is a
  new version. Statements signed under the old wording stay bound to the words
  they were signed under, and those words must remain readable.
- **Vaulet writes these.** A tenant free to author the wording is a tenant free
  to put something unlawful in front of a person, under our name. The catalogue
  is ours; that it is legible to everyone is the point.

The template travels *inside* each statement, so verification needs no network
and no lookup here. This repository is where the catalogue is authored and
where a human goes to read it.

## Where the authoritative copy is, today

**`vaulet-core` still carries the `authorise` wording in its source**, and this
repository is the authored, readable copy of it. That is one copy too many, and
it is stated here rather than hidden, because the rule this catalogue exists to
serve is that there is exactly one.

Collapsing it needs a delivery step that does not exist yet: the wallet builds a
mandate statement on the device, so it needs the wording locally, and the only
honest way to hand it over is the way the address dataset is handed over —
signed bytes, checked against a key the wallet already pins. Until that lands,
the library keeps its copy and this one must be kept byte-identical to it.

## The address dataset

`address/<ISO-3166-1 alpha-2>.json` — the administrative tree an address is
checked against. Thailand, the United States, Japan and China.

**They are not the same shape, and each file says so.** `levels` names the depth
actually present — three for Thailand and China, two for Japan, one for the
United States — because Japan's postal code determines the municipality and
America publishes no city list at all. `postal.role` says which of those a code
is: `leaf`, `pattern` or `determines`. [`SCHEMA.md`](address/SCHEMA.md) has the
evidence, and the reason the United States carries counties that are deliberately
not part of its address.

**Nobody trusts the connection it arrives over.** The issuer serves a country's
tree whole, with an attestation over the digest of *the bytes as sent*, and the
wallet checks two separate things (ADR 0031):

- **are these the bytes that were signed** — a digest of the data exactly as it
  arrived, never of a re-serialised copy
- **did we sign them** — against a key the wallet already pins, so a list of
  place names introduces no new trust

That is the whole reason this is data and not a database: a dataset that mapped
a sub-locality to the wrong postcode would put a wrong address on a credential
that stays true for years, and it would look exactly like the right one.

**The service packs it.** This repository is where a country's tree is authored;
`vaulet-services` takes this crate as a dependency, pinned by revision, and
serves the bytes over `/api/v1/address/{country}` with the attestation. It used
to carry its own copy — two datasets that agree until one of them does not.

### The shape, for whoever adds the second country

```json
{
  "country":   "TH",
  "languages": ["th"],
  "name":      { "en": "Thailand", "local": "ประเทศไทย" },
  "version":   "TH-925cf27e8170",
  "fields":    ["region", "locality", "dependent_locality", "postal_code",
                "street_address", "th.region", "th.locality", "th.dependent_locality"],
  "regions": [
    { "c": 10, "local": "กรุงเทพมหานคร", "en": "Bangkok",
      "l": [
        { "c": 1001, "local": "พระนคร", "en": "Phra Nakhon",
          "d": [
            { "c": 100101, "local": "พระบรมมหาราชวัง",
              "en": "Phra Borom Maha Ratchawang", "p": 10200 }
          ] } ] } ]
}
```

| key | is |
|---|---|
| `c` | the official code for that place, at that level |
| `local` | the name in the country's own language — **this is the address** |
| `en` | a transliteration, so a foreigner can find the row. Not the legal name |
| `l` | localities inside a region — district, county, whatever the country calls it |
| `d` | dependent localities inside a locality — sub-district, ward |
| `p` | postal code, on the leaf, because that is the level that determines it |

`languages` says which language the address is written in officially; `en` sits
beside every row regardless, and is for reading rather than for record. Thailand
is `["th"]`: a Thai address in English is a convenience, not the address.

`fields` declares what this country's addresses are made of, in the three tiers
ADR 0031 sets out — OIDC Core §5.1.1 names, then `dependent_locality`, then
anything prefixed with the country code for what only that country has.

`version` travels with the data and changes when the data does. The wallet
compares it, so it is not decoration.

Thailand today: 77 regions, 928 localities, 7,436 dependent localities, every
one of them carrying both names and a postal code.

**Every level needs both names and a code.** A tree with holes will pass this
repository's tests and produce an address nobody can post to.

**And this shape is Thailand's.** Three levels with a postal code on every leaf
is how Thai addresses work; it is not how American, Japanese or Chinese ones do.
Japan does not require a city at all — a seven-digit code names the prefecture,
the municipality and the town area together. The United States publishes no city
list; a ZIP decides. Adding either in this shape would produce a file that
parses, passes every test here, and is wrong.

[`address/SCHEMA.md`](address/SCHEMA.md) has the evidence and the format that
would carry the difference. **Read it before adding a country.**

## Why these do not belong in the library

The cryptography that binds a statement to a signature belongs in
[`vaulet-core`](https://github.com/vaulet-id/vaulet-core). A sentence in Thai
about Thai company law does not: it is a jurisdiction's product, not a protocol,
and a library that carries one country's legal phrasing has quietly become that
country's library.

The address dataset is the precedent. It has never lived in the code — it
arrives as signed bytes and the wallet checks two separate things: that these are
the bytes that were signed, and that we signed them. Templates follow the same
shape.

## Licence

Apache-2.0. See [LICENSE](LICENSE).

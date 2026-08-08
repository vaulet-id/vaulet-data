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

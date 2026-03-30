{% import "tables.md.j2" as tables %}

# Downloads

## Watchexec CLI

Latest release: [{{ watchexec.latest.version }}](./{{ watchexec.app.slug }}/{{ watchexec.latest.version}}/index.md) ({{ watchexec.latest.published | date }})

{{ tables::notes(notes=watchexec.latest.notes, level='###') }}

**[→ Download this release](./{{ watchexec.app.slug }}/{{ watchexec.latest.version}}/index.md)**

[→ Previous releases](./{{ watchexec.app.slug }}/index.md)

## Cargo Watch

Latest release: [{{ cargo_watch.latest.version }}](./{{ cargo_watch.app.slug }}/{{ cargo_watch.latest.version}}/index.md) ({{ cargo_watch.latest.published | date }})

{{ tables::notes(notes=cargo_watch.latest.notes, level='###') }}

**[→ Download this release](./{{ cargo_watch.app.slug }}/{{ cargo_watch.latest.version}}/index.md)**

[→ Previous releases](./{{ cargo_watch.app.slug }}/index.md)


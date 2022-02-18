{% import "tables.md.j2" as tables %}

# Downloads

## Watchexec CLI

Latest release: {{ watchexec.latest.version }} ({{ watchexec.latest.published | date }})

{{ tables::notes(notes=watchexec.latest.notes) }}

{{ tables::downloads(downloads=watchexec.latest.downloads, sums=watchexec.latest.sums) }}

{{ tables::signatures(sums=watchexec.latest.sums) }}

**[→ Previous releases](/downloads/{{ watchexec.app.slug }}/)**

## Cargo Watch

Latest release: {{ cargo_watch.latest.version }} ({{ cargo_watch.latest.published | date }})

{{ tables::notes(notes=cargo_watch.latest.notes) }}

{{ tables::downloads(downloads=cargo_watch.latest.downloads, sums=cargo_watch.latest.sums) }}

{{ tables::signatures(sums=cargo_watch.latest.sums) }}

**[→ Previous releases](/downloads/{{ cargo_watch.app.slug }}/)**


{% import "tables.md.j2" as tables %}

# {{ app.name }} {{ meta.version }}

{{ tables::notes(notes=meta.notes) }}

{{ tables::downloads(downloads=meta.downloads, sums=meta.sums) }}

{{ tables::signatures(sums=meta.sums) }}

View release [on GitHub](https://github.com/{{ app.repo.owner }}/{{ app.repo.repo }}/releases/{{ tag }})

{{ tables::generator(genver=genver, pub=meta.published, json="meta.json", json_name="json metadata") }}

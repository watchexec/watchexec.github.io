{% import "tables.md.j2" as tables %}

# Download {{ app.name }}
{% raw %}{{#title Download {% endraw %}{{ app.name }}{% raw %} - Watchexec}}{% endraw %}

## Latest release: {{ latest.version }}

{{ tables::downloads(downloads=latest.downloads, sums=[]) }}

## {{ versions | length }} releases available:

{% for v in versions | reverse %}
- [{{ app.name }} {{ v.version }}](./{{ v.version }}/index.md) {{ v.published | date }}
{%- endfor %}

{{ tables::generator(genver=genver, json="latest.json", json_name="latest meta") }}

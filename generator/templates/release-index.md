{% import "tables.md.j2" as tables %}

# Download {{ app.name }}
{% raw %}{{#title Download {% endraw %}{{ app.name }}{% raw %}- Watchexec}}{% endraw %}

{{ versions | length }} releases available:

{% for v in versions | reverse %}
## [{{ app.name }} {{ v.version }}](./{{ v.version }}/index.md) {% if loop.first %}(latest){% endif %}

{%- if v.notes -%}
<details {% if loop.first %}open{% endif %}>
<summary>
Released on **{{ v.published | date }}**.
{{ v.downloads | length }} assets.
Release notes ↓
</summary>

> {{ v.notes | safe }}
</details>
{%- else -%}
Released on {{ v.published | date }}.
{{ v.downloads | length }} assets.
{%- endif -%}

------------
{% endfor %}

{{ tables::generator(genver=genver, json="latest.json", json_name="latest meta") }}

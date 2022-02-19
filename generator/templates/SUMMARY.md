# Summary

- [Home](./index.html)

# Documentation

- [Glob pattern syntax and issues](./docs/glob-patterns.md)
- [Linux inotify limits](./docs/inotify-limits.md)
- [Mac FSEvents limitations](./docs/macos-fsevents.md)

# Project Updates

- [The road to Watchexec 2: Tokio](./blog/road-to-watchexec-2-tokio.md)

# Releases

- [Downloads](./downloads/index.md)
  - [{{ watchexec.app.name }}](./downloads/{{ watchexec.app.slug }}/index.md)
    {% for v in watchexec.versions | reverse %}
    - [{{ watchexec.app.name }} {{ v.version }}](./downloads/{{ watchexec.app.slug }}/{{ v.version }}/index.md)
    {%- endfor %}
  - [{{ cargo_watch.app.name }}](./downloads/{{ cargo_watch.app.slug }}/index.md)
    {% for v in cargo_watch.versions | reverse %}
    - [{{ cargo_watch.app.name }} {{ v.version }}](./downloads/{{ cargo_watch.app.slug }}/{{ v.version }}/index.md)
    {%- endfor %}

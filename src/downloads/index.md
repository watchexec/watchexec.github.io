# Downloads

## Watchexec CLI

Latest release: [1.18.6](./watchexec/1.18.6/index.md) (2022-02-07)

### Release notes

<ul dir="auto">
<li>Paths in <code>WATCHEXEC_*_PATH</code>s are deduplicated (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1117937837" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/253" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/253/hovercard" href="https://github.com/watchexec/watchexec/issues/253">#253</a>)</li>
<li>If the filesystem watcher fails to instantiate, Watchexec will error and stop. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1117919945" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/251" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/251/hovercard" href="https://github.com/watchexec/watchexec/issues/251">#251</a>)</li>
<li>Runtime errors are pretty-printed</li>
<li>Bugfix: global ignores would not be loaded if the working directory wasn't a VCS (Git etc) project (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1125298986" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/255" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/255/hovercard" href="https://github.com/watchexec/watchexec/issues/255">#255</a>)</li>
<li>Reverted change from 1.18.0: <code>.git</code> folders (and so on for other VCS) are ignored by default regardless of whether the watched project is detected as Git (and so on).</li>
</ul>

**[→ Download this release](./watchexec/1.18.6/index.md)**

[→ Previous releases](./watchexec/index.md)

## Cargo Watch

Latest release: [8.1.2](./cargo-watch/8.1.2/index.md) (2021-11-16)

### Release notes

<p dir="auto"><strong>Never released to crates.io</strong></p>

**[→ Download this release](./cargo-watch/8.1.2/index.md)**

[→ Previous releases](./cargo-watch/index.md)


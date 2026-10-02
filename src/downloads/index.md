# Downloads

## Watchexec CLI

Latest release: [2.7.4](./watchexec/2.7.4/index.md) (2026-10-02)

### Release notes

<ul dir="auto">
<li>docs: clarify opt-in event emission in the CLI README (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/Likio3000/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/Likio3000">@Likio3000</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5545099897" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1123" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1123/hovercard" href="https://github.com/watchexec/watchexec/pull/1123">#1123</a>)</li>
<li>fix: error instead of panicking on an unreadable <code class="notranslate">@argfile</code> (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/00200200/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/00200200">@00200200</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5629187879" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1129" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1129/hovercard" href="https://github.com/watchexec/watchexec/pull/1129">#1129</a>)</li>
<li>fix: infer file type of removed paths from the event kind (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/lsh4711/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/lsh4711">@lsh4711</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5636185762" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1130" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1130/hovercard" href="https://github.com/watchexec/watchexec/pull/1130">#1130</a>)</li>
<li>fix: eliminate a potential race in the keyboard watcher (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5676528218" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1133" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1133/hovercard" href="https://github.com/watchexec/watchexec/pull/1133">#1133</a>)</li>
<li>fix: exclude more shell constructs from the exec optimisation (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5676566571" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1135" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1135/hovercard" href="https://github.com/watchexec/watchexec/pull/1135">#1135</a>)</li>
<li>deps: gix-config 0.61, process-wrap 9.1.1, clearscreen 5.0.0 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5679077608" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1136" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1136/hovercard" href="https://github.com/watchexec/watchexec/pull/1136">#1136</a>)</li>
</ul>

**[→ Download this release](./watchexec/2.7.4/index.md)**

[→ Previous releases](./watchexec/index.md)

## Cargo Watch

Latest release: [8.5.3](./cargo-watch/8.5.3/index.md) (2024-10-02)

### Release notes

<p dir="auto">This is the final release of Cargo Watch.</p>
<hr>
<p dir="auto">Cargo Watch is now dormant: it will not receive further updates, but does remain available.</p>
<p dir="auto">I (<a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/passcod/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/passcod">@passcod</a>) currently have very little time to dedicate to unpaid OSS. There is a significant amount of work I deem required to get Watchexec (the library) to a good-enough state to bring its improvements to Cargo Watch, and that has been the case for years without a realistic end in sight. I have had dwindling motivation in the face of having spent 10 years on or around this project and its dependencies (it was a long while ago, but once upon a time the Notify library was spun off from Cargo Watch!), when at the very start, this tool was only made to clear a quick hurdle that I'd encountered while trying to code <em>other, probably more interesting, yet now long-forgotten</em> Rust adventures.</p>
<p dir="auto">However, not all is lost, dear users. For almost the entire life of the project, I have had a thought: that someone with more resources, skill, time, and/or the benefit of hindsight would come around and make something <em>better</em>. Granted, I thought this would happen to Notify. But Notify has persisted, has been passed on to live a long life, and instead the contender is <a href="https://dystroy.org/bacon/" rel="nofollow">Bacon</a>.</p>
<p dir="auto">I have had no involvement in Bacon. Yet it is everything I have wanted to achieve in Cargo Watch. Indeed some five years ago I started development on a Cargo Watch replacement I called "Overwatch", which would have a TUI, a tasks file, a rich pager, and more long-desired features. That never eventuated, though a lot of the low-level improvements that I wrote in preparation for Overwatch "made it" into Notify version 5 and the Watchexec library version 2.<br>
Bacon today is what I wanted Overwatch to be.</p>
<p dir="auto">Let's face it: Cargo Watch has gone through too many incremental changes, with too little overarching design. It sports no less than four different syntaxes to run commands. Its lackluster filtering options can be obnoxious to use. Pager support is non-existent, sometimes requiring arcane invocations to get right. It can conflict with Rust Analyzer (which didn't exist 10 years ago!), though that has improved a lot over the years.</p>
<p dir="auto">It's time to let it go.<br>
Use <a href="https://dystroy.org/bacon/" rel="nofollow">Bacon</a>.<br>
Remember Cargo Watch.</p>
<hr>
<p dir="auto"><a href="https://github.com/watchexec/watchexec">Watchexec</a> is also available for a similar experience that will continue to be maintained, albeit slowly.</p>
<p dir="auto">Discuss at <a href="https://www.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/" rel="nofollow">https://www.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/</a></p>

**[→ Download this release](./cargo-watch/8.5.3/index.md)**

[→ Previous releases](./cargo-watch/index.md)


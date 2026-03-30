# Downloads

## Watchexec CLI

Latest release: [2.5.1](./watchexec/2.5.1/index.md) (2026-03-30)

### Release notes

<p dir="auto">No changes, but two new builds:</p>
<ul dir="auto">
<li>RISC-V (RV64GC)</li>
<li>FreeBSD (x86-64)</li>
</ul>
<p dir="auto">The website has also been refreshed, it's also lot easier to figure out which file to download there: <a href="https://watchexec.github.io/downloads/watchexec/2.5.1/" rel="nofollow">https://watchexec.github.io/downloads/watchexec/2.5.1/</a>.</p>

**[→ Download this release](./watchexec/2.5.1/index.md)**

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


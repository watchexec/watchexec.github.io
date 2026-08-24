# Downloads

## Watchexec CLI

Latest release: [2.7.0](./watchexec/2.7.0/index.md) (2026-08-24)

### Release notes

<p dir="auto">Watchexec now only traverses and watches directories that are expected to produce events (according to filter/ignore files and patterns). This means that if you have a very large directory tree, and 90% of it is gitignored, Watchexec will no longer go through and add watches to that 90% at all. Thus:</p>
<ul dir="auto">
<li>startup in those cases should be much faster and less resource-intensive</li>
<li>ongoing performance should also improve (as there will be vastly fewer events to process), though that may be less perceptible</li>
</ul>
<p dir="auto">The way this is achieved is by handling all directory traversal/recursion within Watchexec, instead of letting the Notify library handle it. This is a large amount of work and code, and it's expected that there will be new bugs as a result. However, this limitation was also the underlying source of a number of long-standing bugs, and I quite look forward to close some very old issues as a result.</p>
<pre class="notranslate"><code class="notranslate">:; hyperfine 'watchexec-2.7.0 -1 echo' 'watchexec-2.6.1 -1 echo'
Benchmark 1: watchexec-2.7.0 -1 echo
  Time (mean ± σ):     158.9 ms ±   2.2 ms    [User: 199.6 ms, System: 63.3 ms]
  Range (min … max):   155.8 ms … 163.1 ms    18 runs

Benchmark 2: watchexec-2.6.1 -1 echo
  Time (mean ± σ):     387.1 ms ±   2.4 ms    [User: 157.5 ms, System: 366.3 ms]
  Range (min … max):   382.0 ms … 390.0 ms    10 runs

Summary
  watchexec-2.7.0 -1 echo ran
    2.44 ± 0.04 times faster than watchexec-2.6.1 -1 echo
</code></pre>
<p dir="auto"><sup>(The <code class="notranslate">-1</code> option is a hidden test-only option which starts watchexec normally, runs the command, and exits.)</sup></p>
<p dir="auto">On BSD, the kqueue backend has been disabled; those platforms will use polling instead. kqueue is not designed for this kind of thing, and using it thus has significant downsides, such as heavy kernel resource usage, as well as correctness bugs. FreeBSD 14 and up have native inotify support; Watchexec does not yet take advantage of this but will in the future (it comes in with Notify 9, which is currently in RC).</p>
<p dir="auto">Watching a single file no longer stops watching it when it is replaced (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="860633319" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/190" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/190/hovercard" href="https://github.com/watchexec/watchexec/issues/190">#190</a>). Similarly, though this is less common, if a folder being watched (using <code class="notranslate">-W</code> or <code class="notranslate">-w</code>) is deleted and recreated, Watchexec used to lose the watch and no longer produce events. That was because the watch was placed on the file (or folder) object, not on a pathname string, and replacing the object naturally makes the watch disappear along with the old version of the object. Watchexec now watches from one path segment above, so it can notice when a watch gets replaced; because it also does directory tree filtering, this is not significantly more expensive.</p>

**[→ Download this release](./watchexec/2.7.0/index.md)**

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


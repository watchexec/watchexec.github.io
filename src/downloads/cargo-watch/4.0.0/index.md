# Cargo Watch 4.0.0

## Release notes

<p>Breaking changes:</p>
<ul>
<li>The command is now run immediately, instead of waiting for changes first (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="141091694" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/37" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/37/hovercard" href="https://github.com/watchexec/cargo-watch/issues/37">#37</a>)</li>
<li>New CLI usage. The simple invocation hasn't changed, but everything else has.</li>
<li>You now have to use <code>-x &lt;cmd&gt;</code> to run custom cargo commands (multiple times as needed).</li>
</ul>
<p>New features:</p>
<ul>
<li>Watch everything: the entire project is watched, but the target/ folder is ignored by default. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="129982042" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/31" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/31/hovercard" href="https://github.com/watchexec/cargo-watch/issues/31">#31</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="140781236" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/35" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/35/hovercard" href="https://github.com/watchexec/cargo-watch/issues/35">#35</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="157529618" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/39" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/39/hovercard" href="https://github.com/watchexec/cargo-watch/issues/39">#39</a>)</li>
<li>You can decide to instead watch specific folders with <code>-w &lt;folder&gt;</code> (multiple times as needed).</li>
<li>Gitignore support: <code>.gitignore</code> files are found and parsed recursively through the repo. This can be disabled with <code>--no-gitignore</code>.</li>
<li>Custom ignore rules can be specified with <code>-i &lt;pattern&gt;</code> (multiple times as needed).</li>
<li>If you absolutely need to, you can explicitly require that no ignoring is applied: <code>--ignore-nothing</code>.</li>
<li>You can suppress all of cargo-watch's output with <code>-q</code>, leaving only the output from the commands.</li>
<li>You can run arbitrary commands with <code>-s &lt;cmd&gt;</code> (for <code>--shell</code>) (multiple times as needed).</li>
<li>You can customise the debouncing delay with <code>--delay</code> or <code>-d</code>, in seconds.</li>
</ul>
<p>Features from 3.2.0 (which was just a few days ago so might be shadowed by this release):</p>
<ul>
<li>You can force cargo-watch to use polling instead of the native API with <code>--poll</code>.</li>
<li>It will automatically fallback to polling if the native API fails to start.</li>
<li>You can clear the screen between runs with <code>--clear</code> or <code>-c</code>.</li>
</ul>
<p>Server development (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="126306738" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/25" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/25/hovercard" href="https://github.com/watchexec/cargo-watch/issues/25">#25</a>) is still not quite supported, but is expected to make it in soon in a point release.</p>

## Packages

<table class="downloads">
<thead>
<tr>
<th>OS</th>
<th>Arch</th>
<th>Variant</th>
<th>Download</th>

</tr>
</thead>
<tbody></tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v4.0.0).

## Checksums





>	 version released on 2017-03-29
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


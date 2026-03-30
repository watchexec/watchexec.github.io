# Cargo Watch 5.0.0

## Release notes

<p>This is the proper way to implement <a href="https://github.com/passcod/cargo-watch/releases/tag/v5.0.0-justwrapit">Version "5"</a>.</p>
<p>In a major break from the past, Cargo Watch now depends directly on <a href="https://github.com/mattgreen/watchexec">Watchexec</a>. It uses the newly-landed (contributed by yours truly) library support in watchexec to directly and statically embed it, parsing our own options and figuring out our own defaults and then running watchexec's main loop with the relevant arguments.</p>
<p>This means that we take advantage of Watchexec's excellent features and fixes in this domain, and can focus entirely on the cargo experience and the specific concerns around it. Notably, this fixes:</p>
<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="126306738" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/25" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/25/hovercard" href="https://github.com/watchexec/cargo-watch/issues/25">#25</a></li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="221915328" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/63" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/63/hovercard" href="https://github.com/watchexec/cargo-watch/issues/63">#63</a></li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="222031577" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/64" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/64/hovercard" href="https://github.com/watchexec/cargo-watch/issues/64">#64</a></li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="219287165" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/62" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/62/hovercard" href="https://github.com/watchexec/cargo-watch/issues/62">#62</a></li>
</ul>
<p>You can install as usual with</p>
<pre><code>$ cargo install cargo-watch
</code></pre>
<p>And upgrade an existing install with:</p>
<pre><code>$ cargo install --force cargo-watch
</code></pre>
<p>If you had previously installed the watchexec binary to make the wrappy version work, and you do not require it anymore, you can safely uninstall it. However, watchexec itself is a great tool and may suit other non-cargo usecases you have well, so consider keeping it.</p>

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


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v5.0.0).

## Checksums





>	 version released on 2017-04-27
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


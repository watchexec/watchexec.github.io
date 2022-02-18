# Cargo Watch 7.5.0

## Release notes

<ul>
<li>Log format (debugs and warnings) changed as part of the upgrade to <a href="https://github.com/watchexec/watchexec/releases/tag/1.14.0">watchexec 1.14</a></li>
<li><code>WATCHEXEC_*</code> environment variables are not set for subprocesses anymore. I didn't actually intend to have them in the first place, but chime in if you rely on them!</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="640629452" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/152" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/152/hovercard" href="https://github.com/watchexec/cargo-watch/pull/152">#152</a> New <code>--features</code> option to pass the <code>--features</code> flag to a selection of cargo commands that support them. E.g. <code>cargo watch --features feat -x check -x test</code> can now be used instead of <code>cargo watch -x 'check --features feat' -x 'test --features feat'</code>.</li>
<li>Also fixes a bug where a polling message was shown even when <code>--quiet</code> was given.</li>
</ul>
<p><a target="_blank" rel="noopener noreferrer" href="https://user-images.githubusercontent.com/155787/86506751-94a38300-be26-11ea-8a73-72f982c699f0.png"><img src="https://user-images.githubusercontent.com/155787/86506751-94a38300-be26-11ea-8a73-72f982c699f0.png" alt="image" style="max-width:100%;"></a></p>

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
<tbody>
<tr>
						<td rowspan="3">Linux</td>
						
<td rowspan="3">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-unknown-linux-gnu.deb">DEB</a> (645 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (646 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (665 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="2">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-pc-windows-msvc.zip">Zip</a> (799 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">MingW</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-pc-windows-gnu.zip">Zip</a> (824 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.5.0/cargo-watch-v7.5.0-x86_64-apple-darwin.tar.xz">XZ</a> (592 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v7.5.0).

## Checksums





>	 version released on 2020-07-04
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


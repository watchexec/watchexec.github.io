# Cargo Watch 7.1.0

## Release notes

<ul>
<li>No more quotes or escapes for the start/finish messages, fixing <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="403453075" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/112" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/112/hovercard" href="https://github.com/watchexec/cargo-watch/issues/112">#112</a>.</li>
<li>The start message was brought into Rust code rather than shell, so quoting doesn't affect it anymore! This may help a few other issues (e.g. <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="391494352" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/107" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/107/hovercard" href="https://github.com/watchexec/cargo-watch/issues/107">#107</a>).</li>
<li>The finish message is printed regardless of whether the command is successful or not.</li>
<li>On unixes only for now, the exit status is now reported as part of the finish message.</li>
<li>LTO and codegen optimisations are now enabled for release builds.</li>
</ul>

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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.1.0/cargo-watch-v7.1.0-x86_64-unknown-linux-gnu.deb">DEB</a> (594 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.1.0/cargo-watch-v7.1.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (596 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.1.0/cargo-watch-v7.1.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (603 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.1.0/cargo-watch-v7.1.0-x86_64-pc-windows-msvc.zip">Zip</a> (691 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.1.0/cargo-watch-v7.1.0-x86_64-apple-darwin.tar.xz">XZ</a> (554 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v7.1.0).

## Checksums





>	 version released on 2019-01-27
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


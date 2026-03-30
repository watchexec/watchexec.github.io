# Watchexec 1.16.0

## Release notes

<ul>
<li><g-emoji class="g-emoji" alias="green_apple" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f34f.png">🍏</g-emoji> New: build config for Apple M1 builds, which will become available <a href="https://github.com/actions/virtual-environments/issues/2486" data-hovercard-type="issue" data-hovercard-url="/actions/virtual-environments/issues/2486/hovercard">sometime in May/June</a></li>
<li><g-emoji class="g-emoji" alias="mechanical_arm" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f9be.png">🦾</g-emoji> New: Windows ARM builds</li>
<li><g-emoji class="g-emoji" alias="pen" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f58a.png">🖊️</g-emoji> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="368405131" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/99" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/99/hovercard" href="https://github.com/watchexec/watchexec/issues/99">#99</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="855280397" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/185" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/185/hovercard" href="https://github.com/watchexec/watchexec/issues/185">#185</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="738206355" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/171" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/171/hovercard" href="https://github.com/watchexec/watchexec/issues/171">#171</a> Use <a href="https://github.com/watchexec/clearscreen"><code>clearscreen</code></a> library. This should in particular help with the <code>-c</code> flag on Windows, and provide a more consistent experience (as much as is possible) everywhere.</li>
<li><g-emoji class="g-emoji" alias="rotating_light" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f6a8.png">🚨</g-emoji> Change: project split into two-package workspace.
<ul>
<li><g-emoji class="g-emoji" alias="warning" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/26a0.png">⚠️</g-emoji> Use <code>cargo install watchexec-cli</code> when installing via cargo.</li>
<li><g-emoji class="g-emoji" alias="package" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f4e6.png">📦</g-emoji> Prefer <code>cargo binstall watchexec-cli</code> when installing via <a href="https://github.com/ryankurte/cargo-binstall">cargo-binstall</a>, though <code>cargo binstall watchexec</code> will still work for now but will rapidly stop working well as the versions diverge.</li>
<li><g-emoji class="g-emoji" alias="arrow_right_hook" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/21aa.png">↪️</g-emoji> A stub <code>main.rs</code> is in place which will direct you to install <code>watchexec-cli</code> when using <code>cargo install watchexec</code>. This stub will go away in a future release.</li>
<li><g-emoji class="g-emoji" alias="books" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f4da.png">📚</g-emoji> The <code>watchexec</code> crate is now the library only and is versioned separately from the binary, which means <strong>semver now applies</strong>.</li>
</ul>
</li>
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
						<td rowspan="10">Linux</td>
						
<td rowspan="2">AArch64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-aarch64-unknown-linux-gnu.deb">DEB</a> (798 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (802 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (858 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (862 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">x86</td>
            
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-i686-unknown-linux-musl.deb">DEB</a> (623 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-i686-unknown-linux-musl.tar.xz">XZ</a> (627 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-unknown-linux-gnu.deb">DEB</a> (591 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (595 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-unknown-linux-musl.deb">DEB</a> (619 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (623 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-aarch64-pc-windows-msvc.zip">Zip</a> (624 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-pc-windows-msvc.zip">Zip</a> (689 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.16.0/watchexec-1.16.0-x86_64-apple-darwin.tar.xz">XZ</a> (560 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.16.0).

## Checksums





>	 version released on 2021-05-08
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


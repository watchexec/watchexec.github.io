# Watchexec 1.20.6

## Release notes

<p dir="auto"><em>Software development often involves running the same commands over and over. Boring! Watchexec is a simple, standalone tool that watches a path and runs a command whenever it detects modifications. Install it today with <a href="https://github.com/cargo-bins/cargo-binstall"><code class="notranslate">cargo-binstall watchexec-cli</code></a>, from the binaries below, find it <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">in your favourite package manager</a>, or build it from source with <code class="notranslate">cargo install watchexec-cli</code>.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>Origins: detect Go, Zig, Pip (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1300586648" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/370" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/370/hovercard" href="https://github.com/watchexec/watchexec/pull/370">#370</a>)</li>
<li>Docs: document <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">known packages</a>, and prioritise package managers in documentation</li>
<li>Deps: update notify to 5.0.0 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1348326448" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/382" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/382/hovercard" href="https://github.com/watchexec/watchexec/issues/382">#382</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1348966613" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/384" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/384/hovercard" href="https://github.com/watchexec/watchexec/pull/384">#384</a>)</li>
<li>Releng: use <a href="https://github.com/cargo-bins/release-pr">cargo-bins/release-pr</a></li>
<li>Releng: change tag scheme. CLI versions are tagged <code class="notranslate">v1.2.3</code>, all other crates are strictly <code class="notranslate">{crate-name}-v1.2.3</code>. Historical tag names are preserved as aliases.</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>Deps: update miette to 5.3.0 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1364012865" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/406" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/406/hovercard" href="https://github.com/watchexec/watchexec/pull/406">#406</a>)</li>
<li>Deps: update git-config to 0.7.1 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1364012865" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/406" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/406/hovercard" href="https://github.com/watchexec/watchexec/pull/406">#406</a>)</li>
<li>Releng: enable dependency updates (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1363967635" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/394" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/394/hovercard" href="https://github.com/watchexec/watchexec/pull/394">#394</a>)</li>
<li>Releng: remove bors</li>
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
						<td rowspan="24">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-gnu.deb">DEB</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-gnu.rpm">RPM</a> (6.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-musl.deb">DEB</a> (6.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-musl.rpm">RPM</a> (6.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-aarch64-unknown-linux-musl.tar.xz">XZ</a> (6.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-armv7-unknown-linux-gnueabihf.deb">DEB</a> (6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (6.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-s390x-unknown-linux-gnu.deb">DEB</a> (6.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-s390x-unknown-linux-gnu.rpm">RPM</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-s390x-unknown-linux-gnu.tar.xz">XZ</a> (6.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-powerpc64le-unknown-linux-gnu.deb">DEB</a> (6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (6.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-i686-unknown-linux-musl.deb">DEB</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-i686-unknown-linux-musl.rpm">RPM</a> (9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-i686-unknown-linux-musl.tar.xz">XZ</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-gnu.deb">DEB</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-gnu.rpm">RPM</a> (8.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-musl.deb">DEB</a> (8.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-musl.rpm">RPM</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-unknown-linux-musl.tar.xz">XZ</a> (8.7 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-pc-windows-msvc.zip">Zip</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/watchexec-1.20.6-x86_64-apple-darwin.tar.xz">XZ</a> (1.4 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v1.20.6).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/v1.20.6/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/v1.20.6/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/v1.20.6/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-09-07
>	|
>	this page built on 2026-03-30 at 18:52
>	| generator v0.0.2
>	| [json metadata](meta.json)


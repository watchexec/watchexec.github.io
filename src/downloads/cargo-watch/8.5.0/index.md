# Cargo Watch 8.5.0

## Release notes

<p dir="auto"><em>Cargo Watch is a tool to watch your Cargo-based project and run commands when files change. It focuses on the Rust development experience and aims to be flexible enough to suit most without becoming complicated to use. Install or upgrade it today with <code class="notranslate">cargo binstall cargo-watch</code>, or <code class="notranslate">cargo install cargo-watch</code> if you don't have <a href="https://github.com/ryankurte/cargo-binstall">Binstall</a> yet.</em></p>
<p dir="auto">Announce: <a href="https://cohost.org/watchexec/post/4074115-cargo-watch-8-5-0" rel="nofollow">https://cohost.org/watchexec/post/4074115-cargo-watch-8-5-0</a></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>New <code class="notranslate">-x</code>-less syntax for select cargo subcommands, e.g. <code class="notranslate">cargo watch clippy</code> (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2065057164" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/297" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/297/hovercard" href="https://github.com/watchexec/cargo-watch/pull/297">#297</a>)</li>
<li>Add hint to bash completion for <code class="notranslate">-x</code> (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2052095344" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/296" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/296/hovercard" href="https://github.com/watchexec/cargo-watch/pull/296">#296</a> by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/pseyfert/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/pseyfert">@pseyfert</a>)</li>
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
						<td rowspan="12">Linux</td>
						
<td rowspan="3">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1.6 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.8 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.6 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.7 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.9 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.7 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-gnu.deb">DEB</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-musl.deb">DEB</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-musl.rpm">RPM</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-aarch64-pc-windows-msvc.zip">Zip</a> (953 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-pc-windows-msvc.zip">Zip</a> (1 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-aarch64-apple-darwin.tar.xz">XZ</a> (701 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/cargo-watch-v8.5.0-x86_64-apple-darwin.tar.xz">XZ</a> (850 KB)</td>
						
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.5.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.5.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.5.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2024-01-04
>	|
>	this page built on 2026-03-30 at 18:20
>	| generator v0.0.2
>	| [json metadata](meta.json)


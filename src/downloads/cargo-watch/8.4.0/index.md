# Cargo Watch 8.4.0

## Release notes

<p dir="auto"><em>Cargo Watch is a tool to watch your Cargo-based project and run commands when files change. It focuses on the Rust development experience and aims to be flexible enough to suit most without becoming complicated to use. Install or upgrade it today with <code class="notranslate">cargo binstall cargo-watch</code>, or <code class="notranslate">cargo install cargo-watch</code> if you don't have <a href="https://github.com/ryankurte/cargo-binstall">Binstall</a> yet.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>Fix a critical bug where signals sent to commands (including the ones cargo watch would send!) would not be handled due to <a href="https://github.com/rust-lang/rust/pull/101077" data-hovercard-type="pull_request" data-hovercard-url="/rust-lang/rust/pull/101077/hovercard">Rust 1.66 changing its behaviour</a>. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1581339738" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/249" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/249/hovercard" href="https://github.com/watchexec/cargo-watch/issues/249">#249</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1575359102" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/247" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/247/hovercard" href="https://github.com/watchexec/cargo-watch/issues/247">#247</a>)</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>Allow <code class="notranslate">--use-shell=none</code> when using a trailing command (not with <code class="notranslate">-x</code> or <code class="notranslate">-s</code>). Note that this requires the trailing command <em>not</em> be quoted in full.</li>
<li>Add <code class="notranslate">--no-process-group</code> to disable using a process group for the command.</li>
<li>Fix malformed output on Windows. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1556955286" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/244" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/244/hovercard" href="https://github.com/watchexec/cargo-watch/pull/244">#244</a>)</li>
</ul>
<h4 dir="auto">Future changes:</h4>
<ul dir="auto">
<li>Intent to remove: checksums and signatures (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1530004228" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/238" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/238/hovercard" href="https://github.com/watchexec/cargo-watch/issues/238">#238</a>)</li>
<li>Intent to remove: RPM and DEB packages (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1530004228" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/238" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/238/hovercard" href="https://github.com/watchexec/cargo-watch/issues/238">#238</a>)</li>
</ul>
<p dir="auto">Comment on the issue above if this would affect you!</p>

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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.5 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.6 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-gnu.deb">DEB</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-musl.deb">DEB</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-musl.rpm">RPM</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-aarch64-pc-windows-msvc.zip">Zip</a> (779 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-pc-windows-msvc.zip">Zip</a> (858 KB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-aarch64-apple-darwin.tar.xz">XZ</a> (582 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/cargo-watch-v8.4.0-x86_64-apple-darwin.tar.xz">XZ</a> (701 KB)</td>
						
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.4.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.4.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.4.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.4.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2023-02-13
>	|
>	this page built on 2026-03-30 at 18:20
>	| generator v0.0.2
>	| [json metadata](meta.json)


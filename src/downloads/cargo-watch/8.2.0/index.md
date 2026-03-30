# Cargo Watch 8.2.0

## Release notes

<p dir="auto"><em>Cargo Watch is a tool to watch your Cargo-based project and run commands when files change. It focuses on the Rust development experience and aims to be flexible enough to suit most without becoming complicated to use. Install or upgrade it today with <code class="notranslate">cargo binstall cargo-watch</code>, or <code class="notranslate">cargo install cargo-watch</code> if you don't have <a href="https://github.com/cargo-bins/cargo-binstall">Binstall</a> yet.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>Local dependencies are watched even when they fall outside of the current Cargo project. Disable with <code class="notranslate">--skip-local-deps</code>. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="424088870" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/117" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/117/hovercard" href="https://github.com/watchexec/cargo-watch/issues/117">#117</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1446881270" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/216" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/216/hovercard" href="https://github.com/watchexec/cargo-watch/pull/216">#216</a>)</li>
<li><code class="notranslate">-E</code>, <code class="notranslate">--env</code> option to inject environment variables to the commands.</li>
<li><code class="notranslate">-L value</code> is a shorthand for <code class="notranslate">--env RUST_LOG=value</code>.</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>MSRV bumped to 1.60.0.</li>
<li>Notifications are enabled on FreeBSD.</li>
<li><code class="notranslate">--clear</code> falls back to a hardcoded escape sequence when a Terminfo database isn't available. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1503941655" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/218" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/218/hovercard" href="https://github.com/watchexec/cargo-watch/issues/218">#218</a>)</li>
<li>When using a trailing command (eg <code class="notranslate">cargo watch -- command</code>), the <code class="notranslate">[Finished running]</code> message won't be emitted. This is to offer a (temporary) workaround when using a shell that doesn't like the <code class="notranslate">&amp;&amp;</code> joining that this message usually uses. See <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1270181274" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/203" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/203/hovercard" href="https://github.com/watchexec/cargo-watch/issues/203">#203</a>.</li>
<li><code class="notranslate">--no-gitignore</code> is renamed to <code class="notranslate">--no-vcs-ignores</code>, but keeps the old name as an alias.</li>
<li><code class="notranslate">--no-ignore</code> is renamed to <code class="notranslate">--no-dot-ignores</code>, for clarity, but keeps the old name as an alias.</li>
<li>The Binstall config for Windows has been fixed, and downloads our builds instead of QuickInstall's. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1222113612" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/199" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/199/hovercard" href="https://github.com/watchexec/cargo-watch/issues/199">#199</a>)</li>
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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.6 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.5 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.7 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.5 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-gnu.deb">DEB</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1.1 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-musl.deb">DEB</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-musl.rpm">RPM</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-aarch64-pc-windows-msvc.zip">Zip</a> (781 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-pc-windows-msvc.zip">Zip</a> (855 KB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-aarch64-apple-darwin.tar.xz">XZ</a> (582 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/cargo-watch-v8.2.0-x86_64-apple-darwin.tar.xz">XZ</a> (694 KB)</td>
						
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.2.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.2.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.2.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.2.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-12-29
>	|
>	this page built on 2026-03-30 at 18:19
>	| generator v0.0.2
>	| [json metadata](meta.json)


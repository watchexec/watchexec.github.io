# Cargo Watch 8.1.0

## Release notes

<p dir="auto"><strong>Yanked from crates.io on 2022-01-22</strong></p>
<ul dir="auto">
<li>New: the <code>-B</code> option injects <code>RUST_BACKTRACE</code> into the environment, which is a fairly common thing to do when catching panics. Use like: <code>cargo watch -B1 -x run</code> (<a href="https://twitter.com/passcod/status/1440223706236669956" rel="nofollow">tweet</a>)</li>
<li>Fix: quoting issues with Zsh (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1000163767" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/183" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/183/hovercard" href="https://github.com/watchexec/cargo-watch/pull/183">#183</a>)</li>
<li>Metadata: The <code>rust-version</code> field is now used in the Cargo.toml. This will generate a warning when compiling until rustc 1.56.</li>
<li>Releng: this is the first release with native Apple M1 binaries!</li>
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
						<td rowspan="8">Linux</td>
						
<td rowspan="2">AArch64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1021 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1006 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-unknown-linux-gnu.deb">DEB</a> (799 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (784 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-unknown-linux-musl.deb">DEB</a> (816 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (802 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-aarch64-pc-windows-msvc.zip">Zip</a> (755 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-pc-windows-msvc.zip">Zip</a> (820 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-aarch64-apple-darwin.tar.xz">XZ</a> (530 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/cargo-watch-v8.1.0-x86_64-apple-darwin.tar.xz">XZ</a> (632 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.1.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.1.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.1.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-09-21
>	|
>	this page built on 2022-02-19 at 12:26
>	| generator v0.0.2
>	| [json metadata](meta.json)


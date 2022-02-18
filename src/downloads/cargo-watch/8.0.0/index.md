# Cargo Watch 8.0.0

## Release notes

<ul>
<li>
<p><strong>Breaking change</strong>: <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="946827844" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/177" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/cargo-watch/pull/177/hovercard" href="https://github.com/watchexec/cargo-watch/pull/177">#177</a> the order of <code>-x</code> and <code>-s</code> is now respected.</p>
<p>Example: <code>cargo watch -s 'echo before' -x test -s 'echo after'</code><br>
Before: would run <code>cargo test &amp;&amp; echo before &amp;&amp; echo after</code>.<br>
Now: runs <code>echo before &amp;&amp; cargo test &amp;&amp; echo after</code>.</p>
</li>
<li>
<p>Experimental: <code>--notify</code>/<code>-N</code> flag sends a desktop notification when a change is observed (which may or may not trigger a command restart). While objectively the better behaviour would be to notify on command <em>finish</em> and vary the notification on exit status, we just can't do that at the moment with the current architecture.</p>
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
						<td rowspan="8">Linux</td>
						
<td rowspan="2">AArch64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1013 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1000 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-unknown-linux-gnu.deb">DEB</a> (801 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (786 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-unknown-linux-musl.deb">DEB</a> (817 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (803 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-aarch64-pc-windows-msvc.zip">Zip</a> (758 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-pc-windows-msvc.zip">Zip</a> (824 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/cargo-watch-v8.0.0-x86_64-apple-darwin.tar.xz">XZ</a> (634 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.0.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.0.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.0.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.0.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-07-30
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


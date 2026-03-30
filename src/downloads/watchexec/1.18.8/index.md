# Watchexec 1.18.8

## Release notes

<ul dir="auto">
<li>Upgraded to <a href="https://github.com/notify-rs/notify/releases/tag/5.0.0-pre.14">Notify pre.14</a></li>
<li>Internal change: kqueue backend is used on mac. This <em>should</em> reduce or eliminate some old persistent bugs on mac, and improve response times, but please report any issues you have!</li>
<li>Both the library's version and the CLI version are now reported in the <code>-v</code> logs</li>
<li>The library version is now specified with an exact (<code>=</code>) requirement, to avoid breakage</li>
<li>Documentation: ordering and deduplication in <code>*_PATH</code> variables (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1161291743" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/262" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/262/hovercard" href="https://github.com/watchexec/watchexec/issues/262">#262</a>)</li>
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
						<td rowspan="18">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-gnu.deb">DEB</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-gnu.rpm">RPM</a> (6.2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-musl.deb">DEB</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-musl.rpm">RPM</a> (6.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (6.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-i686-unknown-linux-musl.deb">DEB</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-i686-unknown-linux-musl.rpm">RPM</a> (9.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-i686-unknown-linux-musl.tar.xz">XZ</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-gnu.deb">DEB</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-gnu.rpm">RPM</a> (9.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-musl.deb">DEB</a> (8.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-musl.rpm">RPM</a> (8.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-unknown-linux-musl.tar.xz">XZ</a> (8.8 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-pc-windows-msvc.zip">Zip</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-aarch64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/watchexec-1.18.8-x86_64-apple-darwin.tar.xz">XZ</a> (1.3 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.18.8).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.8/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.8/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.8/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-03-16
>	|
>	this page built on 2022-03-16 at 04:12
>	| generator v0.0.2
>	| [json metadata](meta.json)


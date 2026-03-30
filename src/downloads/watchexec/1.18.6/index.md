# Watchexec 1.18.6

## Release notes

<ul dir="auto">
<li>Paths in <code>WATCHEXEC_*_PATH</code>s are deduplicated (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1117937837" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/253" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/253/hovercard" href="https://github.com/watchexec/watchexec/issues/253">#253</a>)</li>
<li>If the filesystem watcher fails to instantiate, Watchexec will error and stop. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1117919945" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/251" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/251/hovercard" href="https://github.com/watchexec/watchexec/issues/251">#251</a>)</li>
<li>Runtime errors are pretty-printed</li>
<li>Bugfix: global ignores would not be loaded if the working directory wasn't a VCS (Git etc) project (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1125298986" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/255" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/255/hovercard" href="https://github.com/watchexec/watchexec/issues/255">#255</a>)</li>
<li>Reverted change from 1.18.0: <code>.git</code> folders (and so on for other VCS) are ignored by default regardless of whether the watched project is detected as Git (and so on).</li>
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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-gnu.deb">DEB</a> (4.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-gnu.rpm">RPM</a> (5.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (4.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-musl.deb">DEB</a> (5.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-musl.rpm">RPM</a> (5.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (5.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-i686-unknown-linux-musl.deb">DEB</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-i686-unknown-linux-musl.rpm">RPM</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-i686-unknown-linux-musl.tar.xz">XZ</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-gnu.deb">DEB</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-gnu.rpm">RPM</a> (7.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-musl.deb">DEB</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-musl.rpm">RPM</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-unknown-linux-musl.tar.xz">XZ</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-pc-windows-msvc.zip">Zip</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-aarch64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/watchexec-1.18.6-x86_64-apple-darwin.tar.xz">XZ</a> (1.2 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.18.6).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.6/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.6/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.6/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-02-07
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


# Watchexec 1.19.0

## Release notes

<ul dir="auto">
<li>Decrease default debounce timeout to 50ms (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="710961239" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/168" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/168/hovercard" href="https://github.com/watchexec/watchexec/issues/168">#168</a>).</li>
<li>Add notice that the <code>--shell</code> default will be changing on Unix (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="979180368" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/210" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/210/hovercard" href="https://github.com/watchexec/watchexec/issues/210">#210</a>).</li>
<li>New option <code>--project-origin</code> provides an override which can be useful when the project origin is misdetected, or if you want to stop watchexec from looking for ignore files in (some) parent directories (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1113916874" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/246" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/246/hovercard" href="https://github.com/watchexec/watchexec/issues/246">#246</a>).</li>
<li>New option <code>--workdir</code> sets the command's working directory independent of Watchexec's.</li>
<li>New option <code>--env</code> (short: <code>-E</code>) adds environment variables to the command without affecting Watchexec itself.</li>
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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-gnu.deb">DEB</a> (5.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (5.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (5.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-musl.deb">DEB</a> (5.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-musl.rpm">RPM</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5.3 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5.2 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-i686-unknown-linux-musl.deb">DEB</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-i686-unknown-linux-musl.rpm">RPM</a> (8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-i686-unknown-linux-musl.tar.xz">XZ</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-gnu.deb">DEB</a> (7.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (7.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-musl.deb">DEB</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-musl.rpm">RPM</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-pc-windows-msvc.zip">Zip</a> (1.7 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-aarch64-apple-darwin.tar.xz">XZ</a> (1014 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/watchexec-1.19.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.19.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.19.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.19.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.19.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-04-15
>	|
>	this page built on 2022-04-15 at 04:09
>	| generator v0.0.2
>	| [json metadata](meta.json)


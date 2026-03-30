# Cargo Watch 8.1.1

## Release notes

<p dir="auto"><strong>To be yanked from crates.io</strong> (pending release of 9.0)</p>
<ul dir="auto">
<li>Releng: Experimental: RPM packages are now available. These are built from the same binaries in the tarballs and DEB packages, so may not work properly for distros due to glibc versions or whatever. Untested as I don’t run RPM-based distros, tell me how it goes.</li>
<li>CI: Cross build targets + FreeBSD are now checked.</li>
<li>Workaround: <code>-N</code> / desktop notifications are disabled on FreeBSD (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1002122621" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/184" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/184/hovercard" href="https://github.com/watchexec/cargo-watch/issues/184">#184</a>)</li>
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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-aarch64-unknown-linux-gnu.deb">DEB</a> (1020 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1006 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-gnu.deb">DEB</a> (798 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-gnu.rpm">RPM</a> (867 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (783 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-musl.deb">DEB</a> (815 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-musl.rpm">RPM</a> (874 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-unknown-linux-musl.tar.xz">XZ</a> (801 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-aarch64-pc-windows-msvc.zip">Zip</a> (756 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-pc-windows-msvc.zip">Zip</a> (821 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-aarch64-apple-darwin.tar.xz">XZ</a> (530 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/cargo-watch-v8.1.1-x86_64-apple-darwin.tar.xz">XZ</a> (631 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.1.1).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.1.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.1.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.1.1/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-09-22
>	|
>	this page built on 2022-02-19 at 12:28
>	| generator v0.0.2
>	| [json metadata](meta.json)


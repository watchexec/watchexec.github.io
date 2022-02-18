# Watchexec 1.17.1

## Release notes

<ul>
<li>
<p>Experimental: <code>--notify</code>/<code>-N</code> flag sends a desktop notification when a change is observed (which may or may not trigger a command restart). While objectively the better behaviour would be to notify on command <em>finish</em> and vary the notification on exit status, we just can't do that at the moment with the current architecture.</p>
</li>
<li>
<p>RPM builds are temporarily disabled due some shifts in the ecosystem not being all caught up yet.</p>
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
						<td rowspan="10">Linux</td>
						
<td rowspan="2">AArch64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-aarch64-unknown-linux-gnu.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">x86</td>
            
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-i686-unknown-linux-musl.deb">DEB</a> (924 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-i686-unknown-linux-musl.tar.xz">XZ</a> (920 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-unknown-linux-gnu.deb">DEB</a> (884 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (880 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-unknown-linux-musl.deb">DEB</a> (910 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-unknown-linux-musl.tar.xz">XZ</a> (906 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-aarch64-pc-windows-msvc.zip">Zip</a> (847 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-pc-windows-msvc.zip">Zip</a> (918 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/watchexec-1.17.1-x86_64-apple-darwin.tar.xz">XZ</a> (707 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.17.1).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.17.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.17.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.1/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-07-30
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


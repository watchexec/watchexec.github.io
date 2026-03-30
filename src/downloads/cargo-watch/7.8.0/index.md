# Cargo Watch 7.8.0

## Release notes

<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="873303216" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/172" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/172/hovercard" href="https://github.com/watchexec/cargo-watch/issues/172">#172</a> Restore and document the behaviour where the directory is changed to the project/crate root by default, not the workspace root, as introduced by 7.7.1</li>
<li>New: <code>-C</code>/<code>--workdir</code> option to change the working directory to a custom location. Note that this will behave very strangely in combination with other path options (like <code>-w</code>/<code>--watch</code>) until real support is added upstream, cf <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="855282691" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/188" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/188/hovercard" href="https://github.com/watchexec/watchexec/issues/188">watchexec/watchexec#188</a>.</li>
<li>New: logo.</li>
<li>New: Windows ARM builds.</li>
<li>New: <a href="https://watchexec.github.io/downloads/" rel="nofollow">checksums and release signing</a>.</li>
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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-unknown-linux-gnu.deb">DEB</a> (903 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (908 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (955 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (962 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-gnu.deb">DEB</a> (687 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (693 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.deb">DEB</a> (705 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (711 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-pc-windows-msvc.zip">Zip</a> (763 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-pc-windows-msvc.zip">Zip</a> (830 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-apple-darwin.tar.xz">XZ</a> (643 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v7.8.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-05-01
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


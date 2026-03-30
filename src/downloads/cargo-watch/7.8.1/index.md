# Cargo Watch 7.8.1

## Release notes

<ul>
<li>New: there’s now <strong><a href="https://watchexec.github.io" rel="nofollow">a website</a></strong>. Some of it is still being drafted, notably dedicated general documentation for this tool, but there’s already some specific documentation that’s been brought over and rewritten/improved, such as the <a href="https://watchexec.github.io/docs/inotify-limits.html" rel="nofollow">inotify limitations</a> and the <a href="https://watchexec.github.io/docs/glob-patterns.html" rel="nofollow">glob patterns</a> pages. Eventually a lot of the documentation that’s in the readme will be moved there, so the readme can become a much leaner page with only the important bits, and the information moved over will have a lot more space to really shine.</li>
<li>Meta: moved to the watchexec github organisation. As well as bringing these very related projects together under one umbrella, this should make the maintaining relationship much clearer. As a bonus, that enables Github’s tooling to move issues to a different repo.</li>
<li>Deps: updated to watchexec 1.16.1, which pins the <code>globset</code> crate to version 0.4.6. While not a good long-term fix, this fixes issues installing via <code>cargo install cargo-watch</code> yielding buggy (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="943296406" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/176" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/176/hovercard" href="https://github.com/watchexec/cargo-watch/issues/176">#176</a>) builds without <code>--locked</code>.</li>
<li>Releng: in 7.8.0, binary release builds got checksums and signatures, but generated a combined CHECKSUMS file with both SHA512 and BLAKE3 checksums, which causes warnings to be emitted when checking with the <code>sha512sum</code> or <code>b3sum</code> tools, as well as being a bit ambiguous. In this release, checksums go in separate files, one for each checksum algorithm; these files are also signed separately. I also retroactively fixed that in the 7.8.0 release. The release download list on GitHub is becoming quite long! The <a href="https://watchexec.github.io/downloads/#cargo-watch" rel="nofollow">one on the website</a> is a lot easier to use.</li>
<li>Packagers: if you’re reading these notes:
<ul>
<li>The website now has a JSON metadata file for every release and the latest, and that’s probably easier to parse than fighting it out with the API and the filenames. It also has the release notes and the checksums in nicely machine readable locations if you’re into that.</li>
<li>The archive format for tarballs will change to ZStandard (<code>.tar.zst</code>) at some point in the future, probably around version 8.</li>
</ul>
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
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-aarch64-unknown-linux-gnu.deb">DEB</a> (886 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (891 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-armv7-unknown-linux-gnueabihf.deb">DEB</a> (941 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (948 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-unknown-linux-gnu.deb">DEB</a> (674 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (679 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-unknown-linux-musl.deb">DEB</a> (693 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-unknown-linux-musl.tar.xz">XZ</a> (698 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-aarch64-pc-windows-msvc.zip">Zip</a> (752 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-pc-windows-msvc.zip">Zip</a> (820 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/cargo-watch-v7.8.1-x86_64-apple-darwin.tar.xz">XZ</a> (631 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v7.8.1).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.1/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.1/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-07-15
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


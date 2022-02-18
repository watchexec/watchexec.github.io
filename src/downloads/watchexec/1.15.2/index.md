# Watchexec 1.15.2

## Release notes

<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="368405131" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/99" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/99/hovercard" href="https://github.com/watchexec/watchexec/issues/99">#99</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="855280397" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/185" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/185/hovercard" href="https://github.com/watchexec/watchexec/issues/185">#185</a> <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="738206355" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/171" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/171/hovercard" href="https://github.com/watchexec/watchexec/issues/171">#171</a> Experimental: new spinoff crate <a href="https://github.com/watchexec/clearscreen"><code>clearscreen</code></a> which is a comprehensive terminal/console screen clearing library. Eventually this will be used here, for now it needs a little time to settle and get properly tested around a bunch of terminals and setups (<a href="https://github.com/watchexec/clearscreen/blob/main/TERMINALS.md">and you can help!</a>). However, its API is stable and it is fully documented, so please consider it for new projects!</li>
<li>New: a cute logo for the project! <g-emoji class="g-emoji" alias="shell" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f41a.png">🐚</g-emoji><g-emoji class="g-emoji" alias="crab" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/1f980.png">🦀</g-emoji> This is used in the repo metadata and the API documentation. For now!</li>
<li>On Windows, the application manifest has been added to, which opts us into a few modern settings, adds some metadata, and declares compatibility to Windows 8, 8.1, and 10.</li>
</ul>
<p><a target="_blank" rel="noopener noreferrer" href="https://github.com/watchexec/watchexec/blob/main/doc/logo.png"><img src="https://github.com/watchexec/watchexec/blob/main/doc/logo.png" alt="black and white image of a hermit crab in a round shell with an upside-down diagonal-sideways greek lowercase omega" style="max-width:100%;"></a></p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-aarch64-unknown-linux-gnu.deb">DEB</a> (775 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (779 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-armv7-unknown-linux-gnueabihf.deb">DEB</a> (839 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (844 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">x86</td>
            
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-i686-unknown-linux-musl.deb">DEB</a> (604 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-i686-unknown-linux-musl.tar.xz">XZ</a> (609 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-unknown-linux-gnu.deb">DEB</a> (575 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (579 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-unknown-linux-musl.deb">DEB</a> (606 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-unknown-linux-musl.tar.xz">XZ</a> (610 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-pc-windows-msvc.zip">Zip</a> (665 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.2/watchexec-1.15.2-x86_64-apple-darwin.tar.xz">XZ</a> (551 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.15.2).

## Checksums





>	 version released on 2021-04-26
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


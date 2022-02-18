# Watchexec 1.17.0

## Release notes

<ul>
<li>
<p><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="710961239" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/168" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/168/hovercard" href="https://github.com/watchexec/watchexec/issues/168">#168</a> The default debounce time was further decreased to 100ms.</p>
</li>
<li>
<p><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="558497647" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/145" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/145/hovercard" href="https://github.com/watchexec/watchexec/issues/145">#145</a> New: <code>@path</code>-style argument parsing. <sup>(<a href="https://doc.rust-lang.org/nightly/rustc/command-line-arguments.html#path-load-command-line-flags-from-a-path" rel="nofollow">like rustc</a>)</sup> You can now specify <code>@argfile</code> as the first argument to <code>watchexec</code>: this will read the file <code>argfile</code> and insert each line as an argument (no need to quote for whitespace, blank lines make empty arguments). This is a rudimentary way to have a configuration file, or as shorthand for established workflows; a more thorough file-based configuration system may be introduced later (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="198392565" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/33" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/33/hovercard" href="https://github.com/watchexec/watchexec/issues/33">#33</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="513078994" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/136" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/136/hovercard" href="https://github.com/watchexec/watchexec/issues/136">#136</a>).</p>
</li>
<li>
<p>New: there’s now <strong><a href="https://watchexec.github.io" rel="nofollow">a website</a></strong>. Some of it is still being drafted, notably dedicated general documentation for this tool, but there’s already some specific documentation that’s been brought over and rewritten/improved, such as the <a href="https://watchexec.github.io/docs/inotify-limits.html" rel="nofollow">inotify limitations</a> and the <a href="https://watchexec.github.io/docs/glob-patterns.html" rel="nofollow">glob patterns</a> pages. Eventually a lot of the documentation that’s in readmes and issues will be moved there and improved.</p>
</li>
<li>
<p><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="644081153" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/158" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/158/hovercard" href="https://github.com/watchexec/watchexec/issues/158">#158</a> New: <code>--no-process-group</code> flag to disable process groups.</p>
</li>
<li>
<p>New: RPM packages are now available. These are built from the same binaries in the tarballs and DEB packages, so may not work properly for distros due to glibc versions or whatever. Untested as I don’t run RPM-based distros, tell me how it goes.</p>
</li>
<li>
<p>New: checksums (BLAKE3 and SHA512) and signatures (minisign) for first-party pre-built packages. See <a href="https://watchexec.github.io/downloads/" rel="nofollow">on the website</a>.</p>
</li>
<li>
<p>New: fatal errors (panics) will now be displayed via <a href="https://github.com/yaahc/color-eyre">eyre</a>; for a bit of a friendlier look and easier to debug. Hopefully these aren't seen much though!</p>
</li>
<li>
<p>Packagers: if you’re reading these notes:</p>
<ul>
<li>The website now has a JSON metadata file for every release and the latest, and that’s probably easier to parse than fighting it out with the API and the filenames. It also has the release notes and the checksums in nicely machine readable locations if you’re into that.</li>
<li>The archive format for tarballs will change to ZStandard (.tar.zst) at some point in the future (probably next year).</li>
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
						<td rowspan="15">Linux</td>
						
<td rowspan="3">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-aarch64-unknown-linux-gnu.deb">DEB</a> (956 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (951 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-i686-unknown-linux-musl.deb">DEB</a> (785 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-i686-unknown-linux-musl.rpm">RPM</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-i686-unknown-linux-musl.tar.xz">XZ</a> (781 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-gnu.deb">DEB</a> (748 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (744 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-musl.deb">DEB</a> (778 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-musl.rpm">RPM</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (773 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-aarch64-pc-windows-msvc.zip">Zip</a> (812 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-pc-windows-msvc.zip">Zip</a> (891 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/watchexec-1.17.0-x86_64-apple-darwin.tar.xz">XZ</a> (700 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.17.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.17.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.17.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.17.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2021-07-21
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


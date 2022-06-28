# Watchexec 1.20.1

## Release notes

<ul dir="auto">
<li>Fix various crashes on start, mostly to do with option parsing (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1285013416" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/345" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/345/hovercard" href="https://github.com/watchexec/watchexec/issues/345">#345</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1285732535" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/347" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/347/hovercard" href="https://github.com/watchexec/watchexec/pull/347">#347</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1286622822" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/348" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/348/hovercard" href="https://github.com/watchexec/watchexec/pull/348">#348</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1286627376" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/349" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/349/hovercard" href="https://github.com/watchexec/watchexec/pull/349">#349</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1287248423" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/350" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/350/hovercard" href="https://github.com/watchexec/watchexec/pull/350">#350</a>)</li>
<li>Fix initialisation of logging systems with error handling (being louder) and a less brute-force approach (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1286622822" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/348" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/348/hovercard" href="https://github.com/watchexec/watchexec/pull/348">#348</a>)</li>
<li>Add runtime and <code class="notranslate">--help</code> warnings when running or building with non-standard options (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1286622822" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/348" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/348/hovercard" href="https://github.com/watchexec/watchexec/pull/348">#348</a>)</li>
<li>Allow non-UTF8 argument values for paths (and <code class="notranslate">--exts</code>, as a "path fragment") (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1285013416" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/345" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/345/hovercard" href="https://github.com/watchexec/watchexec/issues/345">#345</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1286627376" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/349" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/349/hovercard" href="https://github.com/watchexec/watchexec/pull/349">#349</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1287248423" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/350" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/350/hovercard" href="https://github.com/watchexec/watchexec/pull/350">#350</a>)</li>
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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-gnu.deb">DEB</a> (5.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-gnu.rpm">RPM</a> (6.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (5.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-musl.deb">DEB</a> (5.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-musl.rpm">RPM</a> (6.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (6.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-i686-unknown-linux-musl.deb">DEB</a> (8.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-i686-unknown-linux-musl.rpm">RPM</a> (8.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-i686-unknown-linux-musl.tar.xz">XZ</a> (8.5 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-gnu.deb">DEB</a> (8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-gnu.rpm">RPM</a> (8.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-musl.deb">DEB</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-musl.rpm">RPM</a> (8.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-unknown-linux-musl.tar.xz">XZ</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-aarch64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/watchexec-1.20.1-x86_64-apple-darwin.tar.xz">XZ</a> (1.3 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.20.1).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.20.1/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.1/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.20.1/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-06-28
>	|
>	this page built on 2022-06-28 at 13:36
>	| generator v0.0.2
>	| [json metadata](meta.json)


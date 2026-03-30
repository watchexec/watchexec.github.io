# Watchexec 1.21.0

## Release notes

<p dir="auto"><em>Software development often involves running the same commands over and over. Boring! Watchexec is a simple, standalone tool that watches a path and runs a command whenever it detects modifications. Install it today with <a href="https://github.com/cargo-bins/cargo-binstall"><code class="notranslate">cargo-binstall watchexec-cli</code></a>, from the binaries below, find it <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">in your favourite package manager</a>, or build it from source with <code class="notranslate">cargo install watchexec-cli</code>.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>New: <code class="notranslate">--stdin-quit</code> makes watchexec exit when STDIN closes, which <a href="https://hexdocs.pm/elixir/1.14.2/Port.html#module-zombie-operating-system-processes" rel="nofollow">avoids zombies when running with Elixir</a>. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1446835013" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/440" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/440/hovercard" href="https://github.com/watchexec/watchexec/issues/440">#440</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1463715980" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/449" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/449/hovercard" href="https://github.com/watchexec/watchexec/pull/449">#449</a>)</li>
<li>Clearscreen improvements on systems without a functioning terminfo database. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1512893907" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/463" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/463/hovercard" href="https://github.com/watchexec/watchexec/pull/463">#463</a>)</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>MSRV bumped to 1.61.0</li>
<li>ZSH autocomplete fixes (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1373576164" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/417" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/417/hovercard" href="https://github.com/watchexec/watchexec/pull/417">#417</a>)</li>
<li>Artifacts are not signed due to a process failure. A new <a href="https://sigstore.dev/" rel="nofollow">more standard</a> signing setup is being prepared for next release.</li>
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
						<td rowspan="24">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-musl.deb">DEB</a> (1.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-musl.rpm">RPM</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (1.6 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-s390x-unknown-linux-gnu.deb">DEB</a> (2.2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-s390x-unknown-linux-gnu.rpm">RPM</a> (2.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.2 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (1.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-i686-unknown-linux-musl.deb">DEB</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-i686-unknown-linux-musl.rpm">RPM</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-i686-unknown-linux-musl.tar.xz">XZ</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-gnu.deb">DEB</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-musl.deb">DEB</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-musl.rpm">RPM</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.9 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-pc-windows-msvc.zip">Zip</a> (2 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.0/watchexec-1.21.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.4 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v1.21.0).

## Checksums





>	 version released on 2023-01-09
>	|
>	this page built on 2026-03-30 at 18:52
>	| generator v0.0.2
>	| [json metadata](meta.json)


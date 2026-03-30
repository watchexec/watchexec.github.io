# Watchexec 1.21.1

## Release notes

<p dir="auto"><em>Software development often involves running the same commands over and over. Boring! Watchexec is a simple, standalone tool that watches a path and runs a command whenever it detects modifications. Install it today with <a href="https://github.com/cargo-bins/cargo-binstall"><code class="notranslate">cargo-binstall watchexec-cli</code></a>, from the binaries below, find it <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">in your favourite package manager</a>, or build it from source with <code class="notranslate">cargo install watchexec-cli</code>.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>Running as PID 1 (for examples, in a container) is now partially supported. Watchexec will properly exit, but may not behave exactly like an init program should. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="519151302" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/140" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/140/hovercard" href="https://github.com/watchexec/watchexec/issues/140">#140</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1584904829" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/497" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/497/hovercard" href="https://github.com/watchexec/watchexec/pull/497">#497</a>)</li>
<li>Fixed a critical bug due to Rust 1.66 changing a behaviour. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1581611930" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/494" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/494/hovercard" href="https://github.com/watchexec/watchexec/pull/494">#494</a>)</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>Added checksum files that are per-file as well as the bulk SUMS files, and added sha256 sums. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1585179518" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/500" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/500/hovercard" href="https://github.com/watchexec/watchexec/pull/500">#500</a>)</li>
<li>Added <a href="https://github.com/axodotdev/cargo-dist">dist-manifest.json</a> (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1585179518" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/500" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/500/hovercard" href="https://github.com/watchexec/watchexec/pull/500">#500</a>)</li>
</ul>

## Packages

<table class="downloads">
<thead>
<tr>
<th>OS</th>
<th>Arch</th>
<th>Variant</th>
<th>Download</th>

<th>Checksums</th>
</tr>
</thead>
<tbody>
<tr>
						<td rowspan="24">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.deb">DEB</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.deb">DEB</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.rpm">RPM</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.tar.xz">XZ</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.6 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.deb">DEB</a> (2.2 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.rpm">RPM</a> (2.5 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.1 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC 64-bit LE</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.deb">DEB</a> (1.7 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (1.7 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.deb">DEB</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.rpm">RPM</a> (2.1 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.tar.xz">XZ</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.deb">DEB</a> (1.8 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.rpm">RPM</a> (2 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1.8 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.deb">DEB</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.rpm">RPM</a> (2.1 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.9 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-pc-windows-msvc.zip">Zip</a> (2 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-apple-darwin.tar.xz">XZ</a> (1.2 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-apple-darwin.tar.xz">XZ</a> (1.4 MB)</td>
						
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/watchexec-1.21.1-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v1.21.1).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.21.1/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2023-02-15
>	|
>	this page built on 2026-03-30 at 18:17
>	| generator v0.0.2
>	| [json metadata](meta.json)


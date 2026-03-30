# Watchexec 2.5.0

## Release notes

<ul dir="auto">
<li>Feature: add <code class="notranslate">--timeout</code> flag to kill long-running commands (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="3983699721" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1022" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1022/hovercard" href="https://github.com/watchexec/watchexec/pull/1022">#1022</a>)</li>
<li>Feature: specify when <code class="notranslate">--notify</code> fires (<code class="notranslate">start</code>, <code class="notranslate">end</code>, <code class="notranslate">both</code>) (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="3983521571" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1021" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1021/hovercard" href="https://github.com/watchexec/watchexec/pull/1021">#1021</a>)</li>
<li>Feature: add <code class="notranslate">--exit-on-error</code> to exit if the process errored (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="3983874269" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1023" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1023/hovercard" href="https://github.com/watchexec/watchexec/pull/1023">#1023</a>)</li>
</ul>
<p dir="auto">This release also bumps the required rustc for building the CLI to 1.93.0 — I'm aware this is pretty new, so if that causes issues, especially for packagers, let me know.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.deb">DEB</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.rpm">RPM</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC 64-bit LE</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-pc-windows-msvc.zip">Zip</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-apple-darwin.tar.xz">XZ</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/watchexec-2.5.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.5.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.5.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2026-02-25
>	|
>	this page built on 2026-03-30 at 18:36
>	| generator v0.0.2
>	| [json metadata](meta.json)


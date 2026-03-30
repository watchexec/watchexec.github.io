# Watchexec 2.3.3

## Release notes

<p dir="auto">Fixes from contributors:</p>
<ul dir="auto">
<li>bring watchexec global ignore file discovery in line with git (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="3602363817" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/973" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/973/hovercard" href="https://github.com/watchexec/watchexec/pull/973">#973</a>)</li>
<li>eliminate a panic when SIGTERM is received in some circumstances (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="3699318157" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/982" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/982/hovercard" href="https://github.com/watchexec/watchexec/pull/982">#982</a>)</li>
</ul>
<p dir="auto">All dependencies were also updated.</p>
<p dir="auto">A change in the "bosion" crate, which provides build-time information for the verbose version output, means that the dependency tree when building from source is about 300 crates lighter, which should result in ~30-60 seconds faster builds.</p>
<p dir="auto">Development pace has slowed considerably but Watchexec remains pretty solid, though it has its various longstanding bugs. Contributions are always appreciated. Let's hope 2026 is a good year.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.deb">DEB</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.rpm">RPM</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC 64-bit LE</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-pc-windows-msvc.zip">Zip</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-apple-darwin.tar.xz">XZ</a> (1.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-apple-darwin.tar.xz">XZ</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/watchexec-2.3.3-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.3.3).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.3/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2026-01-20
>	|
>	this page built on 2026-03-30 at 18:36
>	| generator v0.0.2
>	| [json metadata](meta.json)


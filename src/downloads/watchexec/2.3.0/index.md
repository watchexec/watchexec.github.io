# Watchexec 2.3.0

## Release notes

<ul dir="auto">
<li>Upgrade jaq to 2.0 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2798169309" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/902" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/902/hovercard" href="https://github.com/watchexec/watchexec/pull/902">#902</a>)</li>
<li>Flags are ordered alphabetically in <code class="notranslate">--help</code></li>
<li><code class="notranslate">--filter-prog</code> is no longer experimental (won't warn on use)</li>
<li><code class="notranslate">--only-emit-events</code> now implies <code class="notranslate">--emit-events-to=json-stdio</code></li>
<li>Zero-value unitless time spans are no longer deprecated (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2830968510" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/909" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/909/hovercard" href="https://github.com/watchexec/watchexec/issues/909">#909</a>)</li>
<li>Linux build machines for the pre-built binaries now use Ubuntu 24.04; if you require an older glibc you'll need to build from source</li>
</ul>
<h3 dir="auto">New feature: <code class="notranslate">--socket</code> (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2799946042" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/905" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/905/hovercard" href="https://github.com/watchexec/watchexec/pull/905">#905</a>)</h3>
<p dir="auto">This implements the systemd socket-passing protocol, like a lightweight <a href="https://github.com/mitsuhiko/systemfd">systemfd</a>: sockets are opened from the watchexec process, and then passed to the commands it runs. This lets you keep sockets open and avoid address reuse issues or dropping packets.</p>
<p dir="auto">The value can be either of <code class="notranslate">PORT</code> (opens a TCP listening socket at that port), <code class="notranslate">HOST:PORT</code> (specify a host IP address; IPv6 addresses can be specified <code class="notranslate">[bracketed]</code>), <code class="notranslate">TYPE::PORT</code> or <code class="notranslate">TYPE::HOST:PORT</code> (specify a socket type, <code class="notranslate">tcp</code> / <code class="notranslate">udp</code>). This syntax and the behaviour of this option is compatible with <code class="notranslate">systemfd</code>'s.</p>
<p dir="auto">This integration only provides basic support, if you want more control you should use the <code class="notranslate">systemfd</code> tool from <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/mitsuhiko/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/mitsuhiko">@mitsuhiko</a> directly. To make use of the sockets in Rust code, you can use the <a href="https://github.com/mitsuhiko/listenfd">listenfd</a> crate, also by Armin.</p>
<p dir="auto">I've also written a document that describes the minimal protocol, if you want to see how to use it in your projects or how to implement it yourself, both for Unix (systemd's invention) and for Windows (systemfd's invention): <a href="https://github.com/watchexec/watchexec/blob/main/doc/socket.md">https://github.com/watchexec/watchexec/blob/main/doc/socket.md</a></p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.deb">DEB</a> (3.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.rpm">RPM</a> (3.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (3.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC 64-bit LE</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.deb">DEB</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.tar.xz">XZ</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.deb">DEB</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.rpm">RPM</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-pc-windows-msvc.zip">Zip</a> (3.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-apple-darwin.tar.xz">XZ</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/watchexec-2.3.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.3.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.3.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2025-02-09
>	|
>	this page built on 2026-03-30 at 18:36
>	| generator v0.0.2
>	| [json metadata](meta.json)


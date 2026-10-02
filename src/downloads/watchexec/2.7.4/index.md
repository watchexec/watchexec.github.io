# Watchexec 2.7.4

## Release notes

<ul dir="auto">
<li>docs: clarify opt-in event emission in the CLI README (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/Likio3000/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/Likio3000">@Likio3000</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5545099897" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1123" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1123/hovercard" href="https://github.com/watchexec/watchexec/pull/1123">#1123</a>)</li>
<li>fix: error instead of panicking on an unreadable <code class="notranslate">@argfile</code> (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/00200200/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/00200200">@00200200</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5629187879" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1129" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1129/hovercard" href="https://github.com/watchexec/watchexec/pull/1129">#1129</a>)</li>
<li>fix: infer file type of removed paths from the event kind (by <a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/lsh4711/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/lsh4711">@lsh4711</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5636185762" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1130" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1130/hovercard" href="https://github.com/watchexec/watchexec/pull/1130">#1130</a>)</li>
<li>fix: eliminate a potential race in the keyboard watcher (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5676528218" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1133" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1133/hovercard" href="https://github.com/watchexec/watchexec/pull/1133">#1133</a>)</li>
<li>fix: exclude more shell constructs from the exec optimisation (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5676566571" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1135" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1135/hovercard" href="https://github.com/watchexec/watchexec/pull/1135">#1135</a>)</li>
<li>deps: gix-config 0.61, process-wrap 9.1.1, clearscreen 5.0.0 (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="5679077608" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/1136" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/1136/hovercard" href="https://github.com/watchexec/watchexec/pull/1136">#1136</a>)</li>
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
						<td rowspan="1">FreeBSD</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-freebsd.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-freebsd.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-freebsd.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-freebsd.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="27">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">RISC-V</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.tar.xz">XZ</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-riscv64gc-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.deb">DEB</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.rpm">RPM</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.tar.xz">XZ</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.deb">DEB</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-pc-windows-msvc.zip">Zip</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-pc-windows-msvc.zip">Zip</a> (3.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-apple-darwin.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-apple-darwin.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/watchexec-2.7.4-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.7.4).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.4/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2026-10-02
>	|
>	this page built on 2026-10-02 at 20:04
>	| generator v0.0.2
>	| [json metadata](meta.json)


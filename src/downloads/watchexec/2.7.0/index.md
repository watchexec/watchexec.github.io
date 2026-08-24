# Watchexec 2.7.0

## Release notes

<p dir="auto">Watchexec now only traverses and watches directories that are expected to produce events (according to filter/ignore files and patterns). This means that if you have a very large directory tree, and 90% of it is gitignored, Watchexec will no longer go through and add watches to that 90% at all. Thus:</p>
<ul dir="auto">
<li>startup in those cases should be much faster and less resource-intensive</li>
<li>ongoing performance should also improve (as there will be vastly fewer events to process), though that may be less perceptible</li>
</ul>
<p dir="auto">The way this is achieved is by handling all directory traversal/recursion within Watchexec, instead of letting the Notify library handle it. This is a large amount of work and code, and it's expected that there will be new bugs as a result. However, this limitation was also the underlying source of a number of long-standing bugs, and I quite look forward to close some very old issues as a result.</p>
<pre class="notranslate"><code class="notranslate">:; hyperfine 'watchexec-2.7.0 -1 echo' 'watchexec-2.6.1 -1 echo'
Benchmark 1: watchexec-2.7.0 -1 echo
  Time (mean ± σ):     158.9 ms ±   2.2 ms    [User: 199.6 ms, System: 63.3 ms]
  Range (min … max):   155.8 ms … 163.1 ms    18 runs

Benchmark 2: watchexec-2.6.1 -1 echo
  Time (mean ± σ):     387.1 ms ±   2.4 ms    [User: 157.5 ms, System: 366.3 ms]
  Range (min … max):   382.0 ms … 390.0 ms    10 runs

Summary
  watchexec-2.7.0 -1 echo ran
    2.44 ± 0.04 times faster than watchexec-2.6.1 -1 echo
</code></pre>
<p dir="auto"><sup>(The <code class="notranslate">-1</code> option is a hidden test-only option which starts watchexec normally, runs the command, and exits.)</sup></p>
<p dir="auto">On BSD, the kqueue backend has been disabled; those platforms will use polling instead. kqueue is not designed for this kind of thing, and using it thus has significant downsides, such as heavy kernel resource usage, as well as correctness bugs. FreeBSD 14 and up have native inotify support; Watchexec does not yet take advantage of this but will in the future (it comes in with Notify 9, which is currently in RC).</p>
<p dir="auto">Watching a single file no longer stops watching it when it is replaced (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="860633319" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/190" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/190/hovercard" href="https://github.com/watchexec/watchexec/issues/190">#190</a>). Similarly, though this is less common, if a folder being watched (using <code class="notranslate">-W</code> or <code class="notranslate">-w</code>) is deleted and recreated, Watchexec used to lose the watch and no longer produce events. That was because the watch was placed on the file (or folder) object, not on a pathname string, and replacing the object naturally makes the watch disappear along with the old version of the object. Watchexec now watches from one path segment above, so it can notice when a watch gets replaced; because it also does directory tree filtering, this is not significantly more expensive.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-freebsd.tar.xz">XZ</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-freebsd.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-freebsd.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-freebsd.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="27">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">RISC-V</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-riscv64gc-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.deb">DEB</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.rpm">RPM</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.tar.xz">XZ</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.deb">DEB</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-pc-windows-msvc.zip">Zip</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-pc-windows-msvc.zip">Zip</a> (3.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-apple-darwin.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-apple-darwin.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/watchexec-2.7.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.7.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.7.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2026-08-24
>	|
>	this page built on 2026-08-24 at 16:03
>	| generator v0.0.2
>	| [json metadata](meta.json)


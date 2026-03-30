# Watchexec 1.22.0

## Release notes

<p dir="auto"><em>Software development often involves running the same commands over and over. Boring! Watchexec is a simple, standalone tool that watches a path and runs a command whenever it detects modifications. Install it today with <a href="https://github.com/cargo-bins/cargo-binstall"><code class="notranslate">cargo-binstall watchexec-cli</code></a>, from the binaries below, find it <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">in your favourite package manager</a>, or build it from source with <code class="notranslate">cargo install watchexec-cli</code>.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>New: <code class="notranslate">--emit-events-to &lt;mode&gt;</code> provides alternative ways to receive event details. To the <code class="notranslate">environment</code> mode, equivalent to the previous behaviour (and still the default), the <code class="notranslate">file</code>, <code class="notranslate">stdin</code>, <code class="notranslate">json-file</code>, and <code class="notranslate">json-stdint</code> modes are added. The <a href="https://docs.rs/watchexec-events" rel="nofollow">watchexec-events</a> crate can be used to parse the JSON format from Rust programs.</li>
<li>New: <code class="notranslate">--fs-events &lt;events&gt;</code> provides a more flexible way to filter which kinds of filesystem events cause command runs. The <code class="notranslate">--no-meta</code> option is now an alias to the configuration of this method omitting metadata changes.</li>
<li>New: <code class="notranslate">--clear=reset</code> performs a stronger screen clear (roughly equivalent to a <code class="notranslate">tput reset</code>).</li>
<li>Whole new extended help, manual page, and completion scripts (now for bash, elvish, fish, nu, and powershell in addition to zsh).</li>
<li>New: <code class="notranslate">--manual</code> shows the manual page, <code class="notranslate">--help</code> shows extended help (<code class="notranslate">-h</code> shows short help), <code class="notranslate">--completion &lt;shell&gt;</code> prints a completion script, such that even installing from source can benefit from the manpage or completions.</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>New: <code class="notranslate">--filter-file</code> and <code class="notranslate">--ignore-file</code> provide ways to load arbitrary ignore files, and also "filter files", which should be formatted like ignore files but patterns are treated the same as <code class="notranslate">--ignore</code> patterns.</li>
<li>New: <code class="notranslate">--stop-signal</code> lets you override the signal used to stop the process during a restart.</li>
<li>New: <code class="notranslate">--stop-timeout</code> lets you override the time waited between sending a signal and killing the process (which defaults to 30 seconds).</li>
<li><code class="notranslate">--log-file</code>'s path argument is now optional. Providing the bare <code class="notranslate">--log-file</code> option uses the current directory. Further, the option also accepts a directory instead of a file, in which case it will create a file named <code class="notranslate">watchexec.&lt;timestamp&gt;.log</code> there.</li>
<li><code class="notranslate">--poll</code> is the new name for <code class="notranslate">--force-poll</code> (the old name exists as an alias), and the polling interval is now optional, with a default provided if no value is given.</li>
<li>All options that take durations now both take numbers in the unit described, but also expressions of the form <code class="notranslate">2mins 30s</code>. This is a very similar format as <a href="https://www.freedesktop.org/software/systemd/man/systemd.time.html#" rel="nofollow">systemd's durations</a>, implemented by the <a href="https://docs.rs/humantime" rel="nofollow">humantime</a> crate.</li>
<li>Removed support for the "tagged" filterer experiment.</li>
<li>The long <code class="notranslate">--version</code> option now prints extended version information, including the build date and commit hash (where that information is available). This is powered by a new crate, <a href="https://docs.rs/bosion" rel="nofollow">bosion</a>.</li>
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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.deb">DEB</a> (1.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.deb">DEB</a> (1.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.rpm">RPM</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (1.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.deb">DEB</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.rpm">RPM</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (1.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (1.9 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.deb">DEB</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.rpm">RPM</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.tar.xz">XZ</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.deb">DEB</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.deb">DEB</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.rpm">RPM</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-pc-windows-msvc.zip">Zip</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/watchexec-1.22.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v1.22.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.22.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2023-03-18
>	|
>	this page built on 2026-03-30 at 18:52
>	| generator v0.0.2
>	| [json metadata](meta.json)


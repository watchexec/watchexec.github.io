# Watchexec 1.24.0

## Release notes

<p dir="auto"><em>Software development often involves running the same commands over and over. Boring! Watchexec is a simple, standalone tool that watches a path and runs a command whenever it detects modifications. Install it today with <a href="https://github.com/cargo-bins/cargo-binstall"><code class="notranslate">cargo-binstall watchexec-cli</code></a>, from the binaries below, find it <a href="https://github.com/watchexec/watchexec/blob/main/doc/packages.md">in your favourite package manager</a>, or build it from source with <code class="notranslate">cargo install watchexec-cli</code>.</em></p>
<h4 dir="auto">In this release:</h4>
<ul dir="auto">
<li>New: start/stop messages are now in colour. Use <code class="notranslate">--colour=never</code> (<code class="notranslate">--color</code> also accepted) to disable, or the conventional <code class="notranslate">always</code> and <code class="notranslate">auto</code>. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="556066807" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/144" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/144/hovercard" href="https://github.com/watchexec/watchexec/issues/144">#144</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1111817781" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/237" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/237/hovercard" href="https://github.com/watchexec/watchexec/issues/237">#237</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2012093119" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/698" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/698/hovercard" href="https://github.com/watchexec/watchexec/pull/698">#698</a>)</li>
<li>New: <code class="notranslate">--timings</code> to print how long the command took. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1204344051" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/278" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/278/hovercard" href="https://github.com/watchexec/watchexec/issues/278">#278</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2012093119" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/698" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/698/hovercard" href="https://github.com/watchexec/watchexec/pull/698">#698</a>)</li>
<li>New: <code class="notranslate">--quiet</code> to disable printing any message (except warning and error logs). (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2012093119" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/698" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/698/hovercard" href="https://github.com/watchexec/watchexec/pull/698">#698</a>)</li>
<li>New: <code class="notranslate">--bell</code> to ring the terminal bell on command end. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1111817933" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/238" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/238/hovercard" href="https://github.com/watchexec/watchexec/issues/238">#238</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2012093119" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/698" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/698/hovercard" href="https://github.com/watchexec/watchexec/pull/698">#698</a>)</li>
<li>New: <code class="notranslate">--ignore-nothing</code> to switch on all the <code class="notranslate">--no-*-ignore</code> flags. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1194809568" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/275" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/275/hovercard" href="https://github.com/watchexec/watchexec/issues/275">#275</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1803498406" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/625" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/625/hovercard" href="https://github.com/watchexec/watchexec/issues/625">#625</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2011980242" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/695" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/695/hovercard" href="https://github.com/watchexec/watchexec/pull/695">#695</a>)</li>
<li>New: <code class="notranslate">--only-emit-events</code> disables launching a command, and only prints events to stdout. Requires <code class="notranslate">--emit-events-to</code> to specify the format to print. This lets you obtain a stream of change events to handle directly rather than mediating via a command. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1994301765" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/676" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/676/hovercard" href="https://github.com/watchexec/watchexec/issues/676">#676</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2010889857" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/691" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/691/hovercard" href="https://github.com/watchexec/watchexec/pull/691">#691</a>)</li>
<li>New: <code class="notranslate">--map-signal</code> to map signals received by Watchexec to other signals sent to the command. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="604405956" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/151" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/151/hovercard" href="https://github.com/watchexec/watchexec/issues/151">#151</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1361294475" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/387" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/387/hovercard" href="https://github.com/watchexec/watchexec/issues/387">#387</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2033739882" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/710" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/710/hovercard" href="https://github.com/watchexec/watchexec/pull/710">#710</a>)</li>
<li>Change: <code class="notranslate">--emit-events-to</code> <code class="notranslate">stdin</code> and <code class="notranslate">json-stdin</code> modes are renamed to <code class="notranslate">stdio</code> and <code class="notranslate">json-stdio</code> respectively; the old names are aliased to preserve compatibility.</li>
</ul>
<h4 dir="auto">Other changes:</h4>
<ul dir="auto">
<li>Uses the <a href="https://github.com/watchexec/watchexec/blob/main/crates/lib/CHANGELOG.md#v300-2023-11-26">Watchexec library 3.0</a>. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li><code class="notranslate">-w /dev/null</code> disables watching any files. This is the literal string <code class="notranslate">/dev/null</code>, it won't detect the null device via links or fifos. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li>Running as PID1 (e.g. in Docker) is fully supported. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="519151302" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/140" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/140/hovercard" href="https://github.com/watchexec/watchexec/issues/140">#140</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>, <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1795860419" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/624" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/624/hovercard" href="https://github.com/watchexec/watchexec/pull/624">#624</a>)</li>
<li>Performance improvements and bugfixes around reaping processes (via <code class="notranslate">command-group</code> 5). (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li>Performance improvements and bugfixes around watching files (via <code class="notranslate">notify</code> 6). (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li>Clear the screen <em>before</em> printing events, so <code class="notranslate">--print-events</code> and <code class="notranslate">--clear</code> can meaningfully be used together. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li>Hint that more or less help is available with long <code class="notranslate">--help</code> and short <code class="notranslate">-h</code> flags. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1740247516" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/601" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/601/hovercard" href="https://github.com/watchexec/watchexec/pull/601">#601</a>)</li>
<li>The PDF version of the manual page is gone, due to the tooling I used disappearing, and the general ugliness of its typesetting. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2033739882" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/710" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/710/hovercard" href="https://github.com/watchexec/watchexec/pull/710">#710</a>)</li>
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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.deb">DEB</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.deb">DEB</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.rpm">RPM</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.rpm">RPM</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC 64-bit LE</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.deb">DEB</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.rpm">RPM</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.deb">DEB</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (2.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.deb">DEB</a> (2.4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.rpm">RPM</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-pc-windows-msvc.zip">Zip</a> (2.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.7 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/watchexec-1.24.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v1.24.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v1.24.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2023-12-09
>	|
>	this page built on 2026-03-30 at 18:36
>	| generator v0.0.2
>	| [json metadata](meta.json)


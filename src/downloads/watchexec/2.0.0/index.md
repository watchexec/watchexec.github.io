# Watchexec 2.0.0

## Release notes

<p dir="auto">This is the first breaking release. Most of it is cleaning up a number of deprecated options, and changing some defaults. The idea, however, is to start a new era of Watchexec releases, where breaking changes are allowed more easily (to give an idea of how breaking-change-averse the project has been: this release was planned <em>in January 2022!</em> and ever-delayed since).</p>
<p dir="auto">Fear not! The cadence of breaking releases will be at most once or twice a year, and whenever possible a deprecation will precede a break by at least three months. Watchexec will remain a stable part of your workflow, while allowing ourselves some evolution.</p>
<ul dir="auto">
<li>Shell default changes to <code class="notranslate">$SHELL</code> when it is present. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="979180368" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/210" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/210/hovercard" href="https://github.com/watchexec/watchexec/issues/210">#210</a>)<br>
Use <code class="notranslate">--shell=sh</code> to switch back if your <code class="notranslate">$SHELL</code> is something else.</li>
<li>Shell default changes to Powershell on Windows when Watchexec detects it is running in Powershell. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="309814012" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/80" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/80/hovercard" href="https://github.com/watchexec/watchexec/issues/80">#80</a>)<br>
Use <code class="notranslate">--shell=cmd</code> to switch back to CMD.EXE, or set the <code class="notranslate">SHELL</code> environment variable.<br>
A reminder that Windows 7 is <em>not</em> supported, and hasn't been for years.</li>
<li><code class="notranslate">--on-busy-update</code> defaults to <code class="notranslate">do-nothing</code> now (was <code class="notranslate">queue</code>).<br>
Events received while a command is running won't trigger a run of the command immediately following this one.</li>
<li><code class="notranslate">-W</code> / <code class="notranslate">--watch-when-idle</code> is removed, as it is now the default.</li>
<li>The default for <code class="notranslate">--stop-timeout</code> is now 10 seconds.</li>
<li><code class="notranslate">--debounce</code>, <code class="notranslate">--delay-run</code>, <code class="notranslate">--poll</code>, and <code class="notranslate">--stop-timeout</code> now prefer durations with a unit, and warn if given unit-less durations. The default units for these are millisecond for <code class="notranslate">--debounce</code> and <code class="notranslate">--poll</code>, and seconds for <code class="notranslate">--delay-run</code> and <code class="notranslate">--stop-timeout</code>, which is a source of confusion. Unit-less durations will be removed in a future breaking release.</li>
<li><code class="notranslate">--no-shell</code> is removed.<br>
Use <code class="notranslate">--shell=none</code> instead. The <code class="notranslate">-n</code> short option remains as an alias to <code class="notranslate">--shell=none</code>.</li>
<li><code class="notranslate">-k</code> / <code class="notranslate">--kill</code> is removed.<br>
Use <code class="notranslate">--signal=KILL</code> instead.</li>
<li><code class="notranslate">--changes-only</code> is removed.<br>
Use <code class="notranslate">--print-events</code> instead.</li>
<li><code class="notranslate">--emit-events-to</code> defaults to <code class="notranslate">none</code>, and the <code class="notranslate">environment</code> mode is deprecated.</li>
<li><code class="notranslate">--emit-events-to</code> no longer accepts <code class="notranslate">stdin</code> (deprecated alias for <code class="notranslate">stdio</code>) and <code class="notranslate">json-stdin</code>(deprecated alias for <code class="notranslate">json-stdio</code>).</li>
<li><code class="notranslate">--no-ignore</code> is removed.<br>
Use <code class="notranslate">--no-project-ignore</code> instead.</li>
<li><code class="notranslate">--no-environment</code> is deprecated.</li>
<li><code class="notranslate">--clear=reset</code> will reset the screen on graceful shutdown. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2176475379" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/797" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/797/hovercard" href="https://github.com/watchexec/watchexec/pull/797">#797</a>)</li>
<li><code class="notranslate">--no-process-group</code> is deprecated.</li>
<li>Watchexec no longer warns (nor does anything else) when it sees the deprecated <code class="notranslate">WATCHEXEC_FILTERER</code> environment variable.</li>
</ul>
<h2 dir="auto">Improvements</h2>
<ul dir="auto">
<li>New: <code class="notranslate">--wrap-process=MODE</code> lets you choose between using process groups, process sessions, or nothing at all. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2165865222" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/794" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/794/hovercard" href="https://github.com/watchexec/watchexec/issues/794">#794</a>)</li>
<li>New: the <code class="notranslate">WATCHEXEC_TMPDIR</code> environment variable can be used to customize where Watchexec will write temporary files, if for some reason your <code class="notranslate">$TMPDIR</code> is unwritable. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2249078391" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/814" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/814/hovercard" href="https://github.com/watchexec/watchexec/issues/814">#814</a>)</li>
<li>Fix: watchexec no longer creates a temporary file at startup. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2249078391" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/814" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/814/hovercard" href="https://github.com/watchexec/watchexec/issues/814">#814</a>)</li>
<li>Fix: the screen is no longer cleared on all events, only when starting a new process. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="2230014005" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/809" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/809/hovercard" href="https://github.com/watchexec/watchexec/issues/809">#809</a>)</li>
</ul>
<h2 dir="auto">Experimental new feature</h2>
<p dir="auto">As a treat, this release also features an experimental new option: <code class="notranslate">-j</code> or <code class="notranslate">--filter-prog</code>, which lets you write <em>filter programs</em>.</p>
<h3 dir="auto"><code class="notranslate">-j</code>, <code class="notranslate">--filter-prog EXPRESSION</code></h3>
<p dir="auto">Provide your own custom filter programs in <a href="https://github.com/01mf02/jaq#examples">jaq</a> (similar to jq) syntax. Programs are given an event in the same format as described in <code class="notranslate">--emit-events-to</code> and must return a boolean. In addition to the jaq stdlib, watchexec adds some custom filter definitions:</p>
<ul dir="auto">
<li><code class="notranslate">path | file_meta</code> returns file metadata or null if the file does not exist.</li>
<li><code class="notranslate">path | file_size</code> returns the size of the file at path, or null if it does not exist.</li>
<li><code class="notranslate">path | file_read(bytes)</code> returns a string with the first n bytes of the file at path. If the file is smaller than n bytes, the whole file is returned. There is no filter to read the whole file at once to encourage limiting the amount of data read and processed.</li>
<li><code class="notranslate">string | hash</code>, and <code class="notranslate">path | file_hash</code> return the hash of the string or file at path. No guarantee is made about the algorithm used: treat it as an opaque value.</li>
<li><code class="notranslate">any | kv_store(key)</code>, <code class="notranslate">kv_fetch(key)</code>, and <code class="notranslate">kv_clear</code> provide a simple key-value store. Data is kept in memory only, there is no persistence. Consistency is not guaranteed.</li>
<li><code class="notranslate">any | printout</code>, <code class="notranslate">any | printerr</code>, and <code class="notranslate">any | log(level)</code> will print or log any given value to stdout, stderr, or the log (levels = error, warn, info, debug, trace), and pass the value through (so <code class="notranslate">[1] | log("debug") | .[]</code> will produce a <code class="notranslate">1</code> and log <code class="notranslate">[1]</code>).</li>
</ul>
<p dir="auto">All filtering done with such programs, and especially those using kv or filesystem access, is much slower than the other filtering methods. If filtering is too slow, events will back up and stall watchexec. Take care when designing your filters.</p>
<p dir="auto">If the argument to this option starts with an '@', the rest of the argument is taken to be the path to a file containing a jaq program.</p>
<p dir="auto">Jaq programs are run in order, after all other filters, and short-circuit: if a filter (jaq or not) rejects an event, execution stops there, and no other filters are run. Additionally, they stop after outputting the first value, so you'll want to use 'any' or 'all' when iterating, otherwise only the first item will be processed, which can be quite confusing!</p>
<h3 dir="auto">Examples:</h3>
<p dir="auto">Regexp ignore filter on paths:</p>
<div class="highlight highlight-source-jq" dir="auto"><pre class="notranslate"><span class="pl-c1">all</span>(<span class="pl-e">.tags</span>[] <span class="pl-k">|</span> <span class="pl-c1">select</span>(<span class="pl-e">.kind</span> <span class="pl-k">==</span> <span class="pl-s">"path"</span>); <span class="pl-e">.absolute</span> <span class="pl-k">|</span> <span class="pl-c1">test</span>(<span class="pl-s">"[.]test[.]js$"</span>)) <span class="pl-k">|</span> <span class="pl-c1">not</span></pre></div>
<p dir="auto">Pass any event that creates a file:</p>
<div class="highlight highlight-source-jq" dir="auto"><pre class="notranslate"><span class="pl-c1">any</span>(<span class="pl-e">.tags</span>[] <span class="pl-k">|</span> <span class="pl-c1">select</span>(<span class="pl-e">.kind</span> <span class="pl-k">==</span> <span class="pl-s">"fs"</span>); <span class="pl-e">.simple</span> <span class="pl-k">==</span> <span class="pl-s">"create"</span>)</pre></div>
<p dir="auto">Pass events that touch executable files:</p>
<div class="highlight highlight-source-jq" dir="auto"><pre class="notranslate"><span class="pl-c1">any</span>(<span class="pl-e">.tags</span>[] <span class="pl-k">|</span> <span class="pl-c1">select</span>(<span class="pl-e">.kind</span> <span class="pl-k">==</span> <span class="pl-s">"path"</span> &amp;&amp; <span class="pl-e">.filetype</span> <span class="pl-k">==</span> <span class="pl-s">"file"</span>); <span class="pl-e">.absolute</span> <span class="pl-k">|</span> <span class="pl-c1">metadata</span> <span class="pl-k">|</span> <span class="pl-e">.executable</span>)</pre></div>
<p dir="auto">Ignore files that start with shebangs:</p>
<div class="highlight highlight-source-jq" dir="auto"><pre class="notranslate"><span class="pl-c1">any</span>(<span class="pl-e">.tags</span>[] <span class="pl-k">|</span> <span class="pl-c1">select</span>(<span class="pl-e">.kind</span> <span class="pl-k">==</span> <span class="pl-s">"path"</span> &amp;&amp; <span class="pl-e">.filetype</span> <span class="pl-k">==</span> <span class="pl-s">"file"</span>); <span class="pl-e">.absolute</span> <span class="pl-k">|</span> <span class="pl-c1">read</span>(<span class="pl-c1">2</span>) <span class="pl-k">==</span> <span class="pl-s">"#!"</span>) <span class="pl-k">|</span> <span class="pl-c1">not</span></pre></div>
<p dir="auto">More examples can be found and contributed in the <a href="https://github.com/watchexec/watchexec/discussions/592" data-hovercard-type="discussion" data-hovercard-url="/watchexec/watchexec/discussions/592/hovercard">discussion thread</a></p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (2.6 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-armv7-unknown-linux-gnueabihf.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">IBM Z</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.deb">DEB</a> (3.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.rpm">RPM</a> (4 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.tar.xz">XZ</a> (3.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-s390x-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">PowerPC</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.deb">DEB</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.rpm">RPM</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.tar.xz">XZ</a> (2.8 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-powerpc64le-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.deb">DEB</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.rpm">RPM</a> (3.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.tar.xz">XZ</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-i686-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.deb">DEB</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (3.2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-gnu.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.deb">DEB</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.deb.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.deb.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.deb.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.rpm">RPM</a> (3.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.rpm.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.rpm.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.rpm.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (3.1 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-unknown-linux-musl.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-pc-windows-msvc.zip">Zip</a> (3.5 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-pc-windows-msvc.zip.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-pc-windows-msvc.zip.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-pc-windows-msvc.zip.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-apple-darwin.tar.xz">XZ</a> (2 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-aarch64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-apple-darwin.tar.xz">XZ</a> (2.3 MB)</td>
						<td><small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-apple-darwin.tar.xz.b3">BLAKE3</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-apple-darwin.tar.xz.sha256">SHA256</a></small> <small><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/watchexec-2.0.0-x86_64-apple-darwin.tar.xz.sha512">SHA512</a></small></td>
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/v2.0.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/B3SUMS">BLAKE3 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/SHA256SUMS">SHA256 checksums</a></th>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/v2.0.0/SHA512SUMS">SHA512 checksums</a></th>
		
</tr>
	
</table>




>	 version released on 2024-04-20
>	|
>	this page built on 2026-03-30 at 18:52
>	| generator v0.0.2
>	| [json metadata](meta.json)


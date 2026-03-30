# Watchexec 1.18.0

## Release notes

<p dir="auto">This is the first release built on top of Watchexec library 2.0. <strong>Any change in behaviour other than listed below is a bug.</strong></p>
<ul dir="auto">
<li>Verbose mode is now more configurable, with <code>-v</code>, <code>-vv</code>… up to <code>-vvvv</code> printing increasingly more logs. Note that <code>-v</code> already prints <em>a lot more</em> logs than in previous releases, and that the log format has changed. Additionally, <a href="https://docs.rs/tracing-subscriber/latest/tracing_subscriber/struct.EnvFilter.html" rel="nofollow">the <code>RUST_LOG</code> environment variable</a> can be set for even finer-grained logging.</li>
<li><code>--changes-only</code> was renamed to <code>--print-events</code>, and its output changed. The former option persists as an alias for compatibility but will be removed eventually.</li>
<li><code>--no-ignore</code> was renamed to <code>--no-project-ignore</code> for clarity. The former option persists as an alias for compatibility but will be removed eventually.</li>
<li>When the command ends with a non-0 exit code, or by a signal, or a Windows exception, a message is printed (e.g. <code>[[Command exited with 63]]</code>) (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="203661044" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/37" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/37/hovercard" href="https://github.com/watchexec/watchexec/issues/37">#37</a>)</li>
<li>The <code>--notify</code> / <code>-N</code> option now works properly, and is therefore out of experimental.</li>
<li>Finding the "project origin" now considers other VCSs and many software development tooling conventions.</li>
<li>Ignore file loading has been improved, with support for:
<ul dir="auto">
<li>global VCS ignores (e.g. <code>~/.gitignore</code>, <code>~/.bazaar/ignore</code>) (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="230898429" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/58" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/58/hovercard" href="https://github.com/watchexec/watchexec/issues/58">#58</a>)</li>
<li>global Git excludes (i.e. as configured in git config <code>core.excludesFile</code>)</li>
<li>local project git excludes (i.e. <code>.git/info/exclude</code>)</li>
<li>more VCS ignore files (for Mercurial, Darcs, Fossil, Bazaar) <sub>(nb gitignore syntax is assumed)</sub></li>
<li>a watchexec-specific global ignore file in <code>~/.config/watchexec/ignore</code> or as system-appropriate</li>
<li>any files listed in the new <code>WATCHEXEC_IGNORE_FILES</code> environment variable (format is as for the system <code>PATH</code> variable)</li>
<li>contextual ignores: git ignores won't be loaded for a Mercurial project, for example.</li>
<li>correct ignore file parsing, notably around folder patterns.</li>
</ul>
</li>
<li>The environment variables set on the command have a number of minor changes:
<ul dir="auto">
<li>paths are absolute</li>
<li><code>WATCHEXEC_COMMON_PATH</code> is always set, with the common prefix of all paths</li>
<li><code>WATCHEXEC_OTHERWISE_CHANGED_PATH</code> is new, for event kinds not covered by the other five (<code>CREATED</code>, <code>META_CHANGED</code>, <code>REMOVED</code>, <code>RENAMED</code>, <code>WRITTEN</code>)</li>
<li>within each envvar, paths are sorted (with binary ordering).</li>
</ul>
</li>
<li>Permission and other errors encountered when watching files are now reported with the concerned file path, when available. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="584848025" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/149" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/149/hovercard" href="https://github.com/watchexec/watchexec/issues/149">#149</a>)</li>
<li>Basic Pijul support was added.</li>
<li>Some errors are now not fatal: they are reported to STDERR but won't cause a crash.</li>
<li><a href="https://github.com/microsoft/mimalloc">Mimalloc</a> is used for musl builds, which may improve performance and reduce memory use. This is only done on musl because its default allocator <a href="https://www.linkedin.com/pulse/testing-alternative-c-memory-allocators-pt-2-musl-mystery-gomes/" rel="nofollow">is</a> <a href="https://andygrove.io/2020/05/why-musl-extremely-slow/" rel="nofollow">so</a> <a href="https://github.com/BurntSushi/ripgrep/issues/1268" data-hovercard-type="issue" data-hovercard-url="/BurntSushi/ripgrep/issues/1268/hovercard">terrible</a>. Non-musl builds use the system allocator.</li>
</ul>
<p dir="auto">Note that this release is not coincidental with the stable release of the library 2.0, which is still "in preview."</p>
<h3 dir="auto">New experimental filter engine</h3>
<p dir="auto">Set <code>WATCHEXEC_FILTERER=tagged</code> to opt into a new filter engine. This will disable the <code>-e</code> / <code>--exts</code> and <code>-i</code> / <code>--ignore</code> options and change the <code>-f</code> / <code>--filter</code> option to accept a "tagged filter" instead.</p>
<p dir="auto">Documentation is available at: <a href="https://docs.rs/watchexec/2.0.0-pre.6/watchexec/filter/tagged/struct.TaggedFilterer.html" rel="nofollow">https://docs.rs/watchexec/2.0.0-pre.6/watchexec/filter/tagged/struct.TaggedFilterer.html</a></p>
<p dir="auto">For example, this set of filters:</p>
<pre><code>path=data/*.json
path*!test-*
kind=Create(*)
</code></pre>
<p dir="auto">would only allow file creation events from JSON files in <code>data/</code> except those starting with <code>test-</code>.</p>
<p dir="auto">There are 8 "tags" that can be matched upon, including <code>path</code>, <code>kind</code> (of the event: creates, modifies, metadata, etc), <code>type</code> (of the path that was changed: file, folder, link, etc). There is a variety of operators: glob (<code>*=</code> and <code>*!</code>), exact (<code>==</code> and <code>!=</code>), sets (<code>:=</code> and <code>:!</code>), and regex (<code>~=</code> and <code>~!</code>). The shorthand <code>=</code> operator resolves to the "most useful" operator for the tag being matched (currently <code>*=</code> for <code>path</code>, <code>kind</code>, and <code>exit</code>, <code>:=</code> otherwise). This provides a lot more power and flexibility, but does make filtering less intuitive — ideas are welcome on improvements.</p>
<p dir="auto">The tagged filterer also solves the current filterer's main confusing issue: folder filtering. <code>path=folder/</code> and <code>path!*folder/</code> will do the expected thing, instead of requiring obscure forms like <code>**/folder/**</code>.</p>
<p dir="auto">Using the <code>--filter-file</code> / <code>-F</code> option, one or more "tagged filter files" can be loaded, with every filter on a new line. Blank lines and lines starting with <code>#</code> are ignored. Like for ignore files, the <code>WATCHEXEC_FILTER_FILES</code> envvar is also consulted, and the <code>~/.config/watchexec/filter</code> global filter file applies if present (or in system-relevant location). There is no default project-local filter file.</p>
<p dir="auto"><em>This mode is experimental: it is not to be considered stable in any way. Using it will print a warning message when watchexec starts. The aim is to have it become good enough so it can be the default in a future breaking release (but not 2.0, which will instead focus on other long-standing issues which aren't as earth-shattering as this), or to use its ideas to improve the existing implementation, whichever ends up better for everyone!</em></p>
<h3 dir="auto">Thanks</h3>
<p dir="auto">This release comes with special thanks to Radon Rosborough, supporting me <a href="https://ko-fi.com/passcod" rel="nofollow">via Ko-fi</a>.</p>

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
						<td rowspan="18">Linux</td>
						
<td rowspan="6">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-gnu.deb">DEB</a> (4.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (5.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (4.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-musl.deb">DEB</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-musl.rpm">RPM</a> (5.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (5.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-i686-unknown-linux-musl.deb">DEB</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-i686-unknown-linux-musl.rpm">RPM</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-i686-unknown-linux-musl.tar.xz">XZ</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-gnu.deb">DEB</a> (7.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (7.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-musl.deb">DEB</a> (7.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-musl.rpm">RPM</a> (7.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (7.5 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-pc-windows-msvc.zip">Zip</a> (2.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/watchexec-1.18.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.2 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.18.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/B3SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.18.0/.github/workflows/release.pub">key</a>)
</td>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.18.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
(<a href="https://passcod.name/keys/software.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-01-18
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


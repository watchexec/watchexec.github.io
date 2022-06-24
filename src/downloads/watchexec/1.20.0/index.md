# Watchexec 1.20.0

## Release notes

<ul dir="auto">
<li>Watchexec should not refuse to quit if there are too many events anymore. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1268153992" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/302" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/302/hovercard" href="https://github.com/watchexec/watchexec/pull/302">#302</a>, <a href="https://github.com/watchexec/watchexec/blob/main/crates/lib/CHANGELOG.md#v200-2022-06-17">lib 2.0.0</a>)</li>
<li>The help text is reorganised in semantic sections rather than options/flags. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1268186947" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/304" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/304/hovercard" href="https://github.com/watchexec/watchexec/pull/304">#304</a>)</li>
<li>Verbose logs now may include even more logging (from third-party libraries). (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1263061262" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/300" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/300/hovercard" href="https://github.com/watchexec/watchexec/pull/300">#300</a>)</li>
<li>All arguments must now be valid UTF-8. Previously the command and the <code class="notranslate">-E</code> flag would accept but silently discard invalid UTF-8, now watchexec errors on such input. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1273357615" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/317" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/317/hovercard" href="https://github.com/watchexec/watchexec/pull/317">#317</a>)</li>
<li>New <code class="notranslate">--log-file</code> option makes it easier to collect logs without polluting the screen. (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1273684138" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/321" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/321/hovercard" href="https://github.com/watchexec/watchexec/pull/321">#321</a>)</li>
<li>New <code class="notranslate">--delay-run</code> option adds a configurable sleep before running the command. A native, cross-platform alternative to <code class="notranslate">sleep N; command...</code>! (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="308561423" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/79" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/79/hovercard" href="https://github.com/watchexec/watchexec/issues/79">#79</a>)</li>
</ul>
<p dir="auto">Additionally, some repo changes:</p>
<ul dir="auto">
<li>PRs are now exclusively merged by <a href="https://bors.tech" rel="nofollow">Bors</a>, and pushing directly to <code class="notranslate">main</code> is not allowed.</li>
<li>There's a PR-based (and thus Bors-assisted) release workflow, rather than initiating releases from a local checkout. For example, this release was made with PR <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="1282136554" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/337" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/337/hovercard" href="https://github.com/watchexec/watchexec/pull/337">#337</a>.</li>
<li>Github Discussions are enabled, and should be used for asking questions, discussing the project or features, and comments on releases. Feature requests and bug reports should go to the Issues as before.
<ul dir="auto">
<li>One exception is discussion of "<a href="https://github.com/watchexec/watchexec/discussions/332" data-hovercard-type="discussion" data-hovercard-url="/watchexec/watchexec/discussions/332/hovercard">Known Issues</a>", which has its own topic to keep the Issues list focused on discrete problems.</li>
</ul>
</li>
<li>The scheme for releases has changed slightly: only CLI releases will show up in GitHub releases, so the <a href="https://github.com/watchexec/watchexec/releases/latest">/latest link</a> will always point to the last CLI release; the library and other crates now have their own, file-based <a href="https://github.com/watchexec/watchexec/blob/main/crates/lib/CHANGELOG.md">CHANGELOG.md</a>.</li>
</ul>
<p dir="auto">Not directly related to the CLI, but the <a href="https://docs.rs/watchexec" rel="nofollow">library 2.0.0 version</a> has now been released as stable! Full semver applies again.</p>
<p dir="auto">Known issue with some packages: DEB and RPM packages may be missing dependencies due to being auto-generated instead of lovingly handcrafted. Prefer distributions from packagers rather than these ones, which are just provided on an ad-hoc, if convenient, as-is, basis.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-gnu.deb">DEB</a> (5.6 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-gnu.rpm">RPM</a> (6.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (5.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-musl.deb">DEB</a> (5.8 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-musl.rpm">RPM</a> (6.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-unknown-linux-musl.tar.xz">XZ</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (6.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (5.7 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86</td>
            
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-i686-unknown-linux-musl.deb">DEB</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-i686-unknown-linux-musl.rpm">RPM</a> (8.5 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-i686-unknown-linux-musl.tar.xz">XZ</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-gnu.deb">DEB</a> (7.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-gnu.rpm">RPM</a> (8.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (7.9 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-musl.deb">DEB</a> (8.2 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-musl.rpm">RPM</a> (8.3 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (8.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-aarch64-apple-darwin.tar.xz">XZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/watchexec-1.20.0-x86_64-apple-darwin.tar.xz">XZ</a> (1.3 MB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/cli-v1.20.0).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.20.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/watchexec/releases/download/cli-v1.20.0/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/watchexec/cli-v1.20.0/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2022-06-23
>	|
>	this page built on 2022-06-24 at 03:17
>	| generator v0.0.2
>	| [json metadata](meta.json)


# Watchexec 1.15.0

## Release notes

<ul>
<li>CI and releases switch from Travis to Github Actions.</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="849936718" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/178" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/178/hovercard" href="https://github.com/watchexec/watchexec/pull/178">#178</a> Add <code>arm-unknown-linux-gnueabihf</code> and <code>aarch64-unknown-linux-gnu</code> builds.</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="853714558" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/180" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/180/hovercard" href="https://github.com/watchexec/watchexec/pull/180">#180</a> Add <code>--changes-only</code> option to print what changes (for debugging purposes, not for consuming).</li>
<li>Add <code>--shell=SHELL</code> and deprecate <code>--no-shell</code>.</li>
<li>The above allows to use powershell instead of cmd (<code>--shell=powershell</code>), which solves <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="309814012" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/80" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/80/hovercard" href="https://github.com/watchexec/watchexec/issues/80">#80</a>. On Windows, powershell will become the CLI default in v2.0, and is the new default right now in the library.</li>
<li>Undocument <code>--kill</code> from the man page, help, and completion. It will be removed in v2.0.</li>
<li>In the library, <code>Args</code> is renamed to <code>Config</code> and the old name deprecated.</li>
<li>In the library, <code>get_args</code> is deprecated in the view to be removed at some point (and only exist in the CLI).</li>
<li>In the builder, durations are now Durations instead of integers of milliseconds.</li>
<li>In the builder, <code>restart</code> and <code>watch_when_idle</code> are deprecated and replaced with <code>on_busy_update</code> which clarifies the behaviour when receiving updates while a command is running.</li>
<li>Add <code>--on-busy-update=</code> option to expose that builder option.</li>
<li>Deprecate <code>--watch-when-idle</code> (ref <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="453125990" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/123" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/123/hovercard" href="https://github.com/watchexec/watchexec/issues/123">#123</a>) in favour of <code>--on-busy-update=do-nothing</code>.</li>
<li>Add <a href="https://github.com/ryankurte/cargo-binstall">binstall</a> metadata.</li>
<li>Add DEB packages for every linux build.</li>
<li>Increase MSRV to 1.43.0.</li>
</ul>

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
						<td rowspan="10">Linux</td>
						
<td rowspan="2">AArch64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-aarch64-unknown-linux-gnu.deb">DEB</a> (773 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (776 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">ARMv7 HF</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-armv7-unknown-linux-gnueabihf.deb">DEB</a> (835 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (839 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="2">x86</td>
            
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-i686-unknown-linux-musl.deb">DEB</a> (600 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-i686-unknown-linux-musl.tar.xz">XZ</a> (604 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="4">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-unknown-linux-gnu.deb">DEB</a> (571 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (576 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="2">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-unknown-linux-musl.deb">DEB</a> (602 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (606 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-pc-windows-msvc.zip">Zip</a> (659 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.15.0/watchexec-1.15.0-x86_64-apple-darwin.tar.xz">XZ</a> (548 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.15.0).

## Checksums





>	 version released on 2021-04-10
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


# Watchexec 1.14.0

## Release notes

<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="643802808" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/157" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/157/hovercard" href="https://github.com/watchexec/watchexec/pull/157">#157</a> Error and exit gracefully when the subprocess's environment would be too large (as a result of too many changes being added to the <code>WATCHEXEC_*</code> environment variables instead of throwing a cryptic message ("Argument list too long")</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="643802808" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/157" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/157/hovercard" href="https://github.com/watchexec/watchexec/pull/157">#157</a> Add <code>--no-environment</code> to disable the <code>WATCHEXEC_*</code> variables being set, to avoid the above situation completely.</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="643802808" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/157" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/157/hovercard" href="https://github.com/watchexec/watchexec/pull/157">#157</a> Add <code>--no-meta</code> to disable only "metadata" events from being reported as above, which mitigates the above situation.</li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="645447440" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/160" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/160/hovercard" href="https://github.com/watchexec/watchexec/pull/160">#160</a> <g-emoji class="g-emoji" alias="warning" fallback-src="https://github.githubassets.com/images/icons/emoji/unicode/26a0.png">⚠️</g-emoji> Stop initialising the logger in the library code. Downstream users will need to initialise their own logger if they want debug/warn output. As a reminder, the library API is not considered in semver here, and downstream users are encouraged to specify <strong>exact</strong> versions: <code>watchexec = "=1.14.0"</code>.</li>
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
						<td rowspan="4">Linux</td>
						
<td rowspan="1">x86</td>
            
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-i686-unknown-linux-musl.tar.xz">XZ</a> (641 KB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-unknown-linux-gnu.deb">DEB</a> (626 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (630 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-unknown-linux-musl.tar.xz">XZ</a> (648 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="2">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-pc-windows-msvc.zip">Zip</a> (753 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">MingW</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-pc-windows-gnu.zip">Zip</a> (784 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.14.0/watchexec-1.14.0-x86_64-apple-darwin.tar.xz">XZ</a> (581 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.14.0).

## Checksums





>	 version released on 2020-07-03
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


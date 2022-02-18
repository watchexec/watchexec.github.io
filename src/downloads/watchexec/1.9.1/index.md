# Watchexec 1.9.1

## Release notes

<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="281977645" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/74" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/74/hovercard" href="https://github.com/watchexec/watchexec/pull/74">#74</a> — Watchexec no longer panics on missing folders — patch<sup><strong>1, 2</strong></sup></li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="352699054" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/95" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/95/hovercard" href="https://github.com/watchexec/watchexec/pull/95">#95</a> — Fix issues with whitespace in commands — patch<sup><strong>2</strong></sup></li>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="356592084" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/96" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/96/hovercard" href="https://github.com/watchexec/watchexec/pull/96">#96</a> — Add install instructions for Arch Linux — docs</li>
<li>Create a Debian package on release — releng</li>
<li>Add install instructions for Debian — docs</li>
</ul>
<p><sub> <strong>1:</strong> Other changes were added following this to convert more panics into normal errors: some glob parsing errors, and file watcher initialisation errors.</sub></p>
<p><sub> <strong>2:</strong> These changes also cause a breaking changes to the library interface, but this is not considered in the version numbers (i.e. if you use it, <em>pin it</em>). From cursory investigation only <a href="https://github.com/passcod/cargo-watch">cargo-watch</a> is a consumer and changes there in consequence are coordinated.</sub></p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.1/watchexec-1.9.1-i686-unknown-linux-musl.tar.gz">GZ</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.1/watchexec-1.9.1-x86_64-unknown-linux-gnu.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.1/watchexec-1.9.1-x86_64-unknown-linux-gnu.tar.gz">GZ</a> (1.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.1/watchexec-1.9.1-x86_64-unknown-linux-musl.tar.gz">GZ</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.1/watchexec-1.9.1-x86_64-apple-darwin.tar.gz">GZ</a> (904 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.9.1).

## Checksums





>	 version released on 2018-09-09
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


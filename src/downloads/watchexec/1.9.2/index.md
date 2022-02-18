# Watchexec 1.9.2

## Release notes

<p>My bad. My changes related to whitespace handling completely broke several use cases, e.g.:</p>
<ul>
<li><code>watchexec -- 'echo foo; echo bar'</code></li>
</ul>
<p>See discussion on <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="320178392" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/82" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/82/hovercard" href="https://github.com/watchexec/watchexec/issues/82">#82</a>.</p>
<p>This release reverts (comments out) those parts, but keeps everything else.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-i686-unknown-linux-musl.tar.gz">GZ</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-x86_64-unknown-linux-gnu.deb">DEB</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-x86_64-unknown-linux-gnu.tar.gz">GZ</a> (1.4 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-x86_64-unknown-linux-musl.tar.gz">GZ</a> (1.8 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MingW</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-x86_64-pc-windows-gnu.zip">Zip</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.9.2/watchexec-1.9.2-x86_64-apple-darwin.tar.gz">GZ</a> (905 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.9.2).

## Checksums





>	 version released on 2018-09-09
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


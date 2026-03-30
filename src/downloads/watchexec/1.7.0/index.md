# Watchexec 1.7.0

## Release notes

<ul>
<li>Add <code>-w</code>/<code>--watch</code> option back to manually specify directories to watch. This is useful for cutting down on file descriptors used when watching deeply nested directory structures on Linux.</li>
<li>Load <code>.gitignore</code> files from all watched directories, and then search upwards in the directory hierarchy for any additional ones that need to be loaded. When watching multiple directories with a common parent, <code>.gitignore</code> files will only be loaded once.</li>
<li>Test all <code>.gitignore</code> files found for each notification from the outside in.</li>
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
						<td rowspan="1">Linux</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.7.0/watchexec-1.7.0-x86_64-unknown-linux-gnu.tar.gz">GZ</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MingW</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.7.0/watchexec-1.7.0-x86_64-pc-windows-gnu.zip">Zip</a> (587 KB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.7.0/watchexec-1.7.0-x86_64-apple-darwin.tar.gz">GZ</a> (700 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.7.0).

## Checksums





>	 version released on 2017-02-05
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


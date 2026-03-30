# Watchexec 1.10.0

## Release notes

<h2>New API features</h2>
<p>The API now enables:</p>
<ul>
<li>parsing a command line string into watchexec <code>Args</code></li>
<li>writing one's own handler to react to changes</li>
<li>a lot more errors to be caught instead of panicking</li>
</ul>
<p>This also has creates two minor breaking changes:</p>
<ul>
<li><code>Args</code>'s <code>paths</code> field is now a <code>Vec&lt;PathBuf&gt;</code> rather than a <code>Vec&lt;String&gt;</code></li>
<li><code>cli::get_args()</code> returns a <code>Result</code> rather than nothing</li>
</ul>
<p>This was done in <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="403393276" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/105" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/105/hovercard" href="https://github.com/watchexec/watchexec/pull/105">#105</a>, prompted by <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="402361805" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/104" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/104/hovercard" href="https://github.com/watchexec/watchexec/pull/104">#104</a> and <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="400308445" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/103" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/103/hovercard" href="https://github.com/watchexec/watchexec/pull/103">#103</a>.</p>
<h2>Clap update</h2>
<p>This formalises the fix to <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="289563829" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/76" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/76/hovercard" href="https://github.com/watchexec/watchexec/issues/76">#76</a>.</p>
<h2>Support commands that allocate their own TTY</h2>
<p>Through setsid, thanks to <a class="user-mention" data-hovercard-type="user" data-hovercard-url="/users/Mange/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/Mange">@Mange</a> (<a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="400262130" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/102" data-hovercard-type="pull_request" data-hovercard-url="/watchexec/watchexec/pull/102/hovercard" href="https://github.com/watchexec/watchexec/pull/102">#102</a>).</p>
<p>Fixes <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="217889720" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/47" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/47/hovercard" href="https://github.com/watchexec/watchexec/issues/47">#47</a> and some other issues downstream.</p>
<h2>An attempt to fix some issues clearing screens under some Windows configurations</h2>
<p>These are hard to guess at / repro. Maybe it helped <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="368405131" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/99" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/99/hovercard" href="https://github.com/watchexec/watchexec/issues/99">#99</a>.</p>
<h2>Dependency upgrades</h2>
<ul>
<li>Notify 4.0.7 brings in some fixes, notably a performance/battery-saving optimisation on Linux.</li>
<li>Regex 1.1.0 brings in some performance improvements for (git)ignore patterns</li>
</ul>
<h2>Some progress towards manual restarts</h2>
<p>As part of the API changes, file-triggered runs and manual runs are now explicitly differentiated, which will help when eventually implementing <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="260532309" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/67" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/67/hovercard" href="https://github.com/watchexec/watchexec/issues/67">#67</a>.</p>

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
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-i686-unknown-linux-musl.tar.gz">GZ</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						
						
<td rowspan="3">x86-64</td>
            
						
<td rowspan="2">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-x86_64-unknown-linux-gnu.deb">DEB</a> (867 KB)</td>
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-x86_64-unknown-linux-gnu.tar.gz">GZ</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						
						
						
<td rowspan="1">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-x86_64-unknown-linux-musl.tar.gz">GZ</a> (1.2 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">Windows</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MingW</td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-x86_64-pc-windows-gnu.zip">Zip</a> (1.1 MB)</td>
						
</tr>
					
<tr>
						<td rowspan="1">macOS</td>
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/watchexec/releases/download/1.10.0/watchexec-1.10.0-x86_64-apple-darwin.tar.gz">GZ</a> (833 KB)</td>
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/watchexec/releases/1.10.0).

## Checksums





>	 version released on 2019-01-26
>	|
>	this page built on 2022-02-19 at 03:29
>	| generator v0.0.2
>	| [json metadata](meta.json)


# Cargo Watch 8.5.3

## Release notes

<p dir="auto">This is the final release of Cargo Watch.</p>
<hr>
<p dir="auto">Cargo Watch is now dormant: it will not receive further updates, but does remain available.</p>
<p dir="auto">I (<a class="user-mention notranslate" data-hovercard-type="user" data-hovercard-url="/users/passcod/hovercard" data-octo-click="hovercard-link-click" data-octo-dimensions="link_type:self" href="https://github.com/passcod">@passcod</a>) currently have very little time to dedicate to unpaid OSS. There is a significant amount of work I deem required to get Watchexec (the library) to a good-enough state to bring its improvements to Cargo Watch, and that has been the case for years without a realistic end in sight. I have had dwindling motivation in the face of having spent 10 years on or around this project and its dependencies (it was a long while ago, but once upon a time the Notify library was spun off from Cargo Watch!), when at the very start, this tool was only made to clear a quick hurdle that I'd encountered while trying to code <em>other, probably more interesting, yet now long-forgotten</em> Rust adventures.</p>
<p dir="auto">However, not all is lost, dear users. For almost the entire life of the project, I have had a thought: that someone with more resources, skill, time, and/or the benefit of hindsight would come around and make something <em>better</em>. Granted, I thought this would happen to Notify. But Notify has persisted, has been passed on to live a long life, and instead the contender is <a href="https://dystroy.org/bacon/" rel="nofollow">Bacon</a>.</p>
<p dir="auto">I have had no involvement in Bacon. Yet it is everything I have wanted to achieve in Cargo Watch. Indeed some five years ago I started development on a Cargo Watch replacement I called "Overwatch", which would have a TUI, a tasks file, a rich pager, and more long-desired features. That never eventuated, though a lot of the low-level improvements that I wrote in preparation for Overwatch "made it" into Notify version 5 and the Watchexec library version 2.<br>
Bacon today is what I wanted Overwatch to be.</p>
<p dir="auto">Let's face it: Cargo Watch has gone through too many incremental changes, with too little overarching design. It sports no less than four different syntaxes to run commands. Its lackluster filtering options can be obnoxious to use. Pager support is non-existent, sometimes requiring arcane invocations to get right. It can conflict with Rust Analyzer (which didn't exist 10 years ago!), though that has improved a lot over the years.</p>
<p dir="auto">It's time to let it go.<br>
Use <a href="https://dystroy.org/bacon/" rel="nofollow">Bacon</a>.<br>
Remember Cargo Watch.</p>
<hr>
<p dir="auto"><a href="https://github.com/watchexec/watchexec">Watchexec</a> is also available for a similar experience that will continue to be maintained, albeit slowly.</p>
<p dir="auto">Discuss at <a href="https://www.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/" rel="nofollow">https://www.reddit.com/r/rust/comments/1ftc7cj/cargo_watch_is_on_life_support/</a></p>

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
						<td rowspan="12">Linux</td>
						
<td rowspan="3">AArch64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-aarch64-unknown-linux-gnu.deb">DEB</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-aarch64-unknown-linux-gnu.rpm">RPM</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-aarch64-unknown-linux-gnu.tar.xz">XZ</a> (1.2 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="3">ARMv7 HF</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-armv7-unknown-linux-gnueabihf.deb">DEB</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-armv7-unknown-linux-gnueabihf.rpm">RPM</a> (1.5 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-armv7-unknown-linux-gnueabihf.tar.xz">XZ</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="6">x86-64</td>
            
						
<td rowspan="3">glibc</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-gnu.deb">DEB</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-gnu.rpm">RPM</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-gnu.tar.xz">XZ</a> (1.3 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td rowspan="3">musl</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-musl.deb">DEB</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-musl.rpm">RPM</a> (1.5 MB)</td>
						
						
</tr>
					
<tr>
						
						
						
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-unknown-linux-musl.tar.xz">XZ</a> (1.4 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">Windows</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-aarch64-pc-windows-msvc.zip">Zip</a> (959 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1">MSVC</td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-pc-windows-msvc.zip">Zip</a> (1 MB)</td>
						
						
</tr>
					
<tr>
						<td rowspan="2">macOS</td>
						
<td rowspan="1">AArch64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-aarch64-apple-darwin.tar.xz">XZ</a> (716 KB)</td>
						
						
</tr>
					
<tr>
						
						
<td rowspan="1">x86-64</td>
            
						
<td rowspan="1"></td>
            
<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/cargo-watch-v8.5.3-x86_64-apple-darwin.tar.xz">XZ</a> (873 KB)</td>
						
						
</tr>
					</tbody>
</table>


View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v8.5.3).

## Checksums

<table class="signatures">
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/B3SUMS">BLAKE3 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/B3SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.5.3/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
<tr>
<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/SHA512SUMS">SHA512 checksums</a></th>
		
<td>
<a href="https://github.com/watchexec/cargo-watch/releases/download/v8.5.3/SHA512SUMS.auto.minisig">Automated signature</a>
(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v8.5.3/.github/workflows/release.pub">key</a>)
</td>
		
</tr>
	
</table>




>	 version released on 2024-10-02
>	|
>	this page built on 2026-03-30 at 18:20
>	| generator v0.0.2
>	| [json metadata](meta.json)


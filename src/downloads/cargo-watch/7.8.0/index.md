# Cargo Watch 7.8.0

### Release notes

<ul>
<li><a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="873303216" data-permission-text="Title is private" data-url="https://github.com/watchexec/cargo-watch/issues/172" data-hovercard-type="issue" data-hovercard-url="/watchexec/cargo-watch/issues/172/hovercard" href="https://github.com/watchexec/cargo-watch/issues/172">#172</a> Restore and document the behaviour where the directory is changed to the project/crate root by default, not the workspace root, as introduced by 7.7.1</li>
<li>New: <code>-C</code>/<code>--workdir</code> option to change the working directory to a custom location. Note that this will behave very strangely in combination with other path options (like <code>-w</code>/<code>--watch</code>) until real support is added upstream, cf <a class="issue-link js-issue-link" data-error-text="Failed to load title" data-id="855282691" data-permission-text="Title is private" data-url="https://github.com/watchexec/watchexec/issues/188" data-hovercard-type="issue" data-hovercard-url="/watchexec/watchexec/issues/188/hovercard" href="https://github.com/watchexec/watchexec/issues/188">watchexec/watchexec#188</a>.</li>
<li>New: logo.</li>
<li>New: Windows ARM builds.</li>
<li>New: <a href="https://watchexec.github.io/downloads/" rel="nofollow">checksums and release signing</a>.</li>
</ul>

<table class="downloads">
	<thead>
		<tr>
			<th>OS</th>
			<th>Arch</th>
			<th>Variant</th>
			<th>Download</th>
			<th>BLAKE3 checksum</th>
		</tr>
	</thead>
	<tbody>
					<tr>
						<td rowspan="8">Linux</td>
						<td rowspan="2">AArch64</td>
						<td rowspan="2">glibc</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-unknown-linux-gnu.deb">903 KB (DEB)</a></td>
						<td><code class="checksum">ef75bd0fa29f3710da2d6e034855162c7c02835504f7f21f16789806e09d18e9</code></td>
					</tr>
					
					<tr>
						
						
						
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-unknown-linux-gnu.tar.xz">908 KB (XZ)</a></td>
						<td><code class="checksum">e704de2b30ba647247aa97a456ffb515b93a3dfd44f09753203dc80dd0b3f4c5</code></td>
					</tr>
					
					<tr>
						
						<td rowspan="2">ARMv7 HF</td>
						<td rowspan="2">glibc</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-armv7-unknown-linux-gnueabihf.deb">955 KB (DEB)</a></td>
						<td><code class="checksum">a520f6a08c2f4bf374bfd8323d95077d3201647b07518b2880f4a0be4221d027</code></td>
					</tr>
					
					<tr>
						
						
						
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-armv7-unknown-linux-gnueabihf.tar.xz">962 KB (XZ)</a></td>
						<td><code class="checksum">419d61706c5bbfa0cf114e3eb1c2ae1f475146e568d6d26742d3967463822e63</code></td>
					</tr>
					
					<tr>
						
						<td rowspan="4">x86-64</td>
						<td rowspan="2">glibc</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-gnu.deb">687 KB (DEB)</a></td>
						<td><code class="checksum">90fb7b7073c64aaea23077d8be68f4820be993112007db98969a29787acdc820</code></td>
					</tr>
					
					<tr>
						
						
						
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-gnu.tar.xz">693 KB (XZ)</a></td>
						<td><code class="checksum">2f7776ec81fa19906ef8cf201f97cbc5e5d8f00c20f0ba4b07dad37d35b4d29d</code></td>
					</tr>
					
					<tr>
						
						
						<td rowspan="2">musl</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.deb">705 KB (DEB)</a></td>
						<td><code class="checksum">bcfc831196e08f557918331ccf24159d190822aa6955ba7341bd2cc90f6914ff</code></td>
					</tr>
					
					<tr>
						
						
						
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-unknown-linux-musl.tar.xz">711 KB (XZ)</a></td>
						<td><code class="checksum">70d6185dbb3703f64f04814112827d19041b3826eb0b9be69e4669c621335fc7</code></td>
					</tr>
					
					<tr>
						<td rowspan="2">Windows</td>
						<td rowspan="1">AArch64</td>
						<td rowspan="1">MSVC</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-aarch64-pc-windows-msvc.zip">763 KB (Zip)</a></td>
						<td><code class="checksum">f01b5b99e53c915a2a0cf7336cee5bcde3b3ed280347a92db07c83b63f56c24d</code></td>
					</tr>
					
					<tr>
						
						<td rowspan="1">x86-64</td>
						<td rowspan="1">MSVC</td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-pc-windows-msvc.zip">830 KB (Zip)</a></td>
						<td><code class="checksum">f62f0da7445450e1a5024134dd63a4cc4c08d021a1a86a8e3ec22df2d3b39539</code></td>
					</tr>
					
					<tr>
						<td rowspan="1">macOS</td>
						<td rowspan="1">x86-64</td>
						<td rowspan="1"></td>
						<td><a class="download" href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/cargo-watch-v7.8.0-x86_64-apple-darwin.tar.xz">643 KB (XZ)</a></td>
						<td><em class="missing-checksum">missing checksum</em></td>
					</tr>
					</tbody>
</table>


<table class="signatures">
	
	<tr>
		<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS">BLAKE3 checksums</a></th>
		
		<td>
			<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS.auto.minisig">Automated signature</a>
			(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.0/.github/workflows/release.pub">key</a>)
		</td>
		
		<td>
			<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/B3SUMS.passcod.minisig">Félix’s signature</a>
			(<a href="https://passcod.name/keys/software.pub">key</a>)
		</td>
		
	</tr>
	
	<tr>
		<th><a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS">SHA512 checksums</a></th>
		
		<td>
			<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS.auto.minisig">Automated signature</a>
			(<a href="https://raw.githubusercontent.com/watchexec/cargo-watch/v7.8.0/.github/workflows/release.pub">key</a>)
		</td>
		
		<td>
			<a href="https://github.com/watchexec/cargo-watch/releases/download/v7.8.0/SHA512SUMS.passcod.minisig">Félix’s signature</a>
			(<a href="https://passcod.name/keys/software.pub">key</a>)
		</td>
		
	</tr>
	
</table>



View release [on GitHub](https://github.com/watchexec/cargo-watch/releases/v7.8.0)


>	 version released on 2021-05-01
>	|
>	this page built on 2022-02-19 at 02:21 UTC
>	| generator v0.0.1
>	| [json metadata](meta.json)


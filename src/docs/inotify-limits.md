# Linux inotify limits

The underlying technology used to watch for filesystem changes on Linux is
[inotify](https://www.kernel.org/doc/html/latest/filesystems/inotify.html), a
component of the Linux kernel.

inotify has several configurable limits:

- `max_user_instances` limits (roughly) how many applications can watch files
  (per user);
- `max_user_watches` limits how many filesystem items can be watched, in total
  across all applications (per user);
- `max_queued_events` limits how many filesystem events will be held in the
  kernel queue if the application does not read them;

There are also physical limits, dependent on the amount of RAM/memory a system
has available, and open file/handle limits, as configured in
[limits.conf (5)](https://man.archlinux.org/man/limits.conf.5) and with Bash's
`ulimit`.

Note that the first two limits are _per user_, rather than per application.
This means that all applications a user runs which make use of filesystem
events _compete_ for the same “pool” of resources. Also note that it's not
possible to set different limits for different users.

The default `max_user_watches` limit on most Linux systems is **8192**. That is
regularly reached if you watch large folder structures, or if you use more than
one application that watches the filesystem in some way, something you might
not be aware of!

## Errors you may have seen

You may have seen one of the following errors, or something similar, which are
characteristic of exceeding an inotify limit:

```plain
Unable to monitor filesystem. Please run:
echo 100000 | sudo tee /proc/sys/fs/inotify/max_user_watches
```

```plain
No space left on device - Failed to watch “…”:
The user limit on the total number of inotify watches was
reached or the kernel failed to allocate a needed resource.
(Errno::ENOSPC)
```

```plain
Io(Error { repr: Os {
	code: 28,
	message: “No space left on device”
} })
```

```plain
tail: cannot watch ‘/path/to/dir’: No space left on device
```

```plain
Failed to watch /path/to/dir;
upper limit on inotify watches reached!
```

Note that these errors are from a variety of applications, as this page is
intended for guidance for more than just the Watchexec family of tools.

## Fixing it

For most users, the default limit is much too low. **65536** is a good start on
a more reasonable allowance.

To fix it permanently, you need to use
[sysctl](https://man.archlinux.org/man/core/systemd/sysctl.d.5.en) to configure
your kernel on boot. Write the following line to a appropriately-named file
under `/etc/sysctl.d/`, for example `/etc/sysctl.d/inotify.conf`:

```ini
fs.inotify.max_user_watches=65536
```

To fix it temporarily (it will persist until a reboot), use the `sysctl`
command instead:

```bash
sysctl fs.inotify.max_user_watches=65536
```

You may increase the limits further, if you require it.

The hard upper bound is controlled by how much kernel memory you are willing or
able to dedicate for this use. One _inotify watch_ costs 540 bytes of kernel
memory on 32-bit architectures, and 1080 bytes on 64-bit ones. The consuming
application may have additional overhead on this.

For example, applications that use the
[Notify](https://github.com/notify-rs/notify) library use about 10 additional
bytes, plus the full filesystem path, for each watched item. (A watched item is
generally a single directory. When watching directories recursively, one
_inotify watch_ is established _per_ subdirectory.)

There are recorded instances of people using _millions_ of watches, for example
to monitor their entire home directory. They may potentially be using
_gigabytes_ of kernel (which isn’t swappable) and userspace (which is) memory
for that purpose.

### Other limits

`max_user_instances` may also be too low on some systems. For example, on
servers it is frequently set as low as **127**. A good workstation default is
**1024**.

`max_queued_events` is rarely an issue. Applications will most often read
events from the queue as fast as they can, and use their own userspace queues
and structures to deal with them, so events will not remain kernel-side for
very long.

This is compounded by the behaviour of the inotify subsystem when the queue
maximum is reached: instead of ignoring further events, the whole inotify
instance is dropped. This is usually a fatal error for an application, and may
be hard to recover from if not: applications will therefore want to zealously
avoid that happening.

## Sources

- [Sorah Fukumori: limit of inotify](http://web.archive.org/web/20161106193425/http://blog.sorah.jp/2012/01/24/inotify-limitation) (archived in 2016, original published 24 january 2012)
- [Unix.SE: Kernel inotify watch limit reached](https://unix.stackexchange.com/questions/13751/kernel-inotify-watch-limit-reached) (25 may 2011)
- [Linux Journal: Kernel Korner - Intro to inotify](http://www.linuxjournal.com/article/8478) (28 september 2005)

> **Note on dnotify, inotify, fanotify, fsnotify**
>
> Dnotify was the original Linux kernel implementation of filesystem watching.
> Prior to dnotify was “FAM”, which came from IRIX. Dnotify was replaced around
> 2008 by inotify. Then fanotify was initially implemented, but ultimately not
> merged and abandoned. Meanwhile, fsnotify replaced the entire _backend_
> kernel implementation, with dnotify and inotify (and then fanotify)
> re-written as _frontends_ on fsnotify, eliminating duplication.
>
> Thus, nowadays (i.e. on kernels 4.3 and higher, from about 2015 onwards) when
> speaking of all or any of these systems, one is talking about the _same_
> mechanism, but accessed through different kernel calls, potentially exposing
> different capabilities and trade-offs (e.g. fanotify is more powerful and may
> be vastly more efficient, but currently — as of 2021 — requires root access,
> as it is geared towards access control rather than monitoring).

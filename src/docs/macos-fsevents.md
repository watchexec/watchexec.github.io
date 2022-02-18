# macOS FSEvents limitations

On Watchexec prior to 1.18, and on Cargo Watch, the underlying technology used
to watch for filesystem changes on Mac is
[FSEvents](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/FSEvents_ProgGuide/TechnologyOverview/TechnologyOverview.html#//apple_ref/doc/uid/TP40005289-CH3-SW1),
a component of the Darwin kernel.

FSEvents is used because [Notify](https://github.com/notify-rs/notify), the
filesystem monitor library Watchexec uses, chose it as its backend on Mac prior
to version 5. However, that was a mistake.

> The author of this page is [Félix Saparelli](https://passcod.name), the
> original author of Notify. They feel confident in stating their own mistakes,
> and note that Notify versions 5 and up have since changed this. For unrelated
> reasons, Félix is no longer involved in maintainership of the Notify project.

FSEvents is **not designed for the usecases Notify (and Watchexec) is used for.**

Instead, it was designed for backup software like Apple TimeMachine:
internally, it is powered by a log of filesystem-level events on every mounted
volume. When an application requests a watch, it delivers events regarding that
subset of the volume. Due to how the logging is done, it may batch events,
provide them out of order, miss some entirely, or even deliver events to an
application that it _didn't ask for_.

The idea behind FSEvents is to receive a notification that _something_ changed
on a volume, and so the application should go and rescan either all or part of
the volume it is looking after.

The _correct_ interface to use for Watchexec and most Notify usecases is
[Kqueue](https://en.wikipedia.org/wiki/Kqueue), which is a component of BSD
kernels, of which family Darwin is part. Notify versions 5 and up use Kqueue on
Mac and BSDs, and so Watchexec 1.18+ (and soon, Cargo Watch 9+) use Notify 5.

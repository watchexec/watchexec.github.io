# The road to Watchexec 2: Tokio

> In July last year, I started working on what I didn't yet know would be a
> complete rewrite of the Watchexec internals.
> 
> Motivations abounded: the initial drive was to finally move to an async core.
> This move has long been a mental, if not strictly real, blocker for a variety
> of issues and features missing from Watchexec. Keyboard input in the style of
> nodemon and entr. Mouse input, even less attainable. Some kind of pager or
> TUI, a pipe dream.
> 
> Also plaguing Watchexec were the numerous issues with filters and ignores. As
> I went through the filesystem event code, I rediscovered a particular
> abberation: the handling for `.ignore` and `.gitignore` files was completely
> separate, even if they mostly did the same thing.
> 
> In August, it became evident a gradual approach, a refactor, was not enough.
> Feeling like I was possibly making a mistake, I trashed the work I had, moved
> the library code out of the `src` tree, and started from scratch. I upgraded
> to Notify 5, adopted its "callbacks, not channels" model, put down the
> general foundation lines, and struck rock.
> 
> I expected to be finished sometime before November, to make a Watchexec CLI
> release about then, iron out the compatibility bugs during December, and
> start on new exciting features in the glorious new year.
> 
> That didn't quite happen.
> 
> I've had enough, however, of woeful tales of miscalculations and burnout.
> Been there. Got the scars. Still hurts. Instead I want to talk about a few
> choices I made, and how they panned out. This is the first episode in a
> series.

## Tokio

I started with Rust async with Tokio in the 0.1 days. I... didn't like it much.
I rode halfway through the 0.2 chaos before throwing in the towel. At some
point, I picked up async-std and had a much better time. I remained an
async-std person until I really started considering Watchexec 2.

At that point, the choice was between async-std/smol, Tokio, and possibly Actix
but I never strongly considered it. You can see my hesitation play out in the
[Command Group](https://github.com/watchexec/command-group) implementation,
where after implementing the std-based, thus synchronous version, I cast out
PRs to both Tokio and async-std for the underlying APIs which would allow me to
complete the work.

My intention at the time was to provide both implementations. Both APIs are now
available, yet I only implemented the Tokio version in Command Group, mostly
because Tokio got back to me first. I could have easily implemented the
async-std version, but [never did](https://github.com/watchexec/command-group/issues/4).

Command group support was one of two critical _technical_ requirements I had
for the async implementation. The other was signals.

Signal handling is critical in a long-running process. Signal handling is also
fiendishly complicated, and signalhandling in an async context is where nasal
demons like to lurk when they're not eating your laundry. I never want to deal
with it directly.

Tokio has a built-in signal module.
Async-std [doesn't](https://github.com/async-rs/async-std/issues/302).

I liked async-std's channels better than the Tokio ones. Async-std
[channels](https://docs.rs/async-std/latest/async_std/channel/index.html) are a
single type, they general work for all kinds of situations, _both `Sender` and
`Receiver` sides are `Clone`._ Oooh, what a pain it is in Tokio that this is
not true.

But nicer channels against nasal demons? Sorry, async-std.

After spending winter, spring, and summer with Tokio, I have to say I like it.
It works. I've never complained about speed, but that's not really a concern in
Watchexec anyway. Tokio documentation is awesome. Tokio 1.x has been remarkably
stable, such that I've never feared to upgrade. Some edges are rough and
doubled-up, especially when dealing with `Result`-returning futures, Tokio
still having accommodations which are now better handled in the futures crate.
I've talked about channels. There's no Tokio `Condvar`.

[Tokio Console](https://github.com/tokio-rs/console) is _amazing_.

I have built a permanent Cargo feature into the Watchexec CLI so that I can pop
open a Console anytime during development. It has helped me figure out several
thorny issues in mere moments.

A lot of things work with Tokio. It certainly has more energy behind it, though
that is no fault of others. At the end of the road, the question is: will I
choose Tokio for future projects? Yes.

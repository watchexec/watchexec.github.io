# Glob patterns

Glob patterns in Watchexec (for `--ignore` and `--filter`) and in Cargo Watch
(for `--ignore`) are matched against the path of a file affected by an event
and include or exclude that event from triggering an action based on that
match.

Thus, the purpose of glob patterns is to provide flexible and powerful
filtering in a syntax familiar to users in respect to other tools in similar
shells.

The current implementation of glob patterns relies on the globset library. Here
is a basic summary of its syntax:

- `?` matches one character.
- `*` matches zero or more characters but not beyond a path component boundary
  (i.e. it stays within two slashes).
- `**` matches zero or more characters and may span several path components
  (but there are many limitations, see docs).
- `{a,b}` matches a or b where a and b are glob patterns (nesting is not
  supported).
- `[ab]` matches a or b where a and b are characters (not patterns).

## Cargo Watch: Subtleties that may catch you out

The current implementation is less than friendly in some situations, due to how
matching is performed. The most important thing to note is that matching is
done over the full, absolute path. Here’s how that might catch you out:

- You generally want to prepend `*/` to all patterns that don't otherwise start
  with a glob star.

- Folders that are parents of the watched or current directory can get matched.
  For example, if the current directory is `/home/data/project` and your
  project contains a directory at `test/data` that you want to ignore, the
  pattern `*/data/**/*` will match every event path, resulting in zero events
  triggering actions.

- Ignoring a directory doesn’t ignore files inside it, because the events will
  be delivered with the path for the files within, so you need to use a pattern
  like `*/foldername/**/*` instead.

This is being upgraded and most of those issues are gone in Watchexec 1.18+.
The underlying engine will eventually make it to Cargo Watch, once it has
stabilised.

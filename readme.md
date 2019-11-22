# git-remote-fetcher(1) -- recursively fetches remote origins

## :ok_hand: Why

As a high paced engineer working on many git repo's, I faced an annoying
problem. And that was, most companies I work for enforce up to date heads when
pushing branches. Which is'nt a problem if you're the only author to a branch,
but sometimes more than 1 more collaborate, and I found myself git committing,
then pushing only for the remote to reject it.

So; I ran up a quick and dirty crontab to git fetch some of my repo's every 5
mins, and with fishshell's help, I got instant feedback when my head was out of
date.

This is just an app that wraps that idea, in tool I hope to grow, as well as be
a product I can use to learn rust along for the ride as well.

## :dragon_face: Usage

Supply a starting point where it will traverse down and collect every directory
that is a git root.

```sh
git-remote-fetcher <LOCATION>
```

## :bow: Ambition

- [ ] Support glob style for directory, ie `git-remote-fetcher '~/{dev,sites}'`
- [ ] Perhaps be a daemon itself, rather than run it in crontab?
- [ ] async or thread each fetcher, rather than waterfall
- [ ] Add in a .ignore type file, to specify either repos or directories to not
      fetch
- [ ] Specify which origins, rather than _all_ like it is now

## :poop: Known issues

- Currently your repos must use ssh authentication, because I somehow need to
  store username and passwords for http style repos. Or unless im missing
  something :man_shrugging:
- Windows users
  [have to use Pageant](https://github.com/libssh2/libssh2/blob/81b2548fef64f1d278ac02ff27aa0055b84c3776/src/agent.c#L277-L279)

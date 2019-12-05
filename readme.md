# git-remote-fetcher(1) -- recursively fetches remote origins

> A utility that fetches all remotes for all git roots south of a given
> location.

## :ok_hand: Why

As a high paced engineer working on many git repos I faced an annoying problem;
most companies I work for enforce up to date heads when pushing branches. This,
and I am one of those infamous rebase'ers, which is not a problem if you are the
only author of a branch. But, sometimes there is more than one collaborator. I
also found myself git committing, then pushing only for the remote to reject it.

So, I ran up a quick and dirty crontab to git fetch some of my repos every five
minutes, and with fishshell's help, I got instant feedback when my head was out
of date.

This software wraps that idea in tool-form. I hope to grow this tool while
learning rust along the way.

## :dragon_face: Usage

Supply a starting point where it will traverse down and collect every directory
that is a git root.

```sh
git-remote-fetcher <LOCATION>
```

If you're repo are using the username/password clone, you have to be using a
credentials manager. Please see
[Caching your GitHub password in Git](https://help.github.com/en/github/using-git/caching-your-github-password-in-git)
on how to enable that. Even if you're not using GitHub the process still
applies.

## :bow: Ambition

- [ ] Support glob style for directory, ie `git-remote-fetcher '~/{dev,sites}'`
- [ ] Perhaps be a daemon itself, rather than run it in crontab?
- [ ] async or thread each fetcher, rather than waterfall
- [ ] Add in a .ignore type file, to specify either repos or directories to not
      fetch
- [ ] Specify which origins, rather than _all_ like it is now
- [ ] When an error happens, depending on where, we shouldn't fail there, but
      exit 1. ie try everything, and panic at the end maybe

## :poop: Known issues

- Windows users
  [have to use Pageant](https://github.com/libssh2/libssh2/blob/81b2548fef64f1d278ac02ff27aa0055b84c3776/src/agent.c#L277-L279)
  for SSH

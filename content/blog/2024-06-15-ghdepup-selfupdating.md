+++
title="ghdepup is now self-updating"
date=2024-06-15T14:30:00+02:00
[taxonomies]
originally-published-on=["github.io"]
categories=["opensource", "rust", "github", "dependencies"]
+++
ghdepup is now self-updating
============================

<p style="text-align:right;"><em>Choo-choo, choo-choo, choo-choo, choo</em></p>
<p style="text-align:right;"><em>We're riding on the Tekkno train</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=CFlhlZbeKgE">Tekkno Train, Electric Callboy</a></em></p>

So, for a selfhosted private project, I wanted to update my dependencies regulary and without too much hassle. Usually, tools like [dependabot](https://github.com/dependabot) or [renovate](https://github.com/renovatebot/renovate) can be used for that. However, being a nagging old neckbeard, those two both seemed to be big dependencies themselves -- I wanted something simpler. I also wanted to try to use the [github API](https://docs.github.com/en/rest?apiVersion=2022-11-28) directly and write some [rust](https://www.rust-lang.org/) for fun.

And thus I created [ghdepup](https://www.rust-lang.org/), which does what I wanted -- and just that. It now <em>works for me(tm)</em> and if you need to get semantic versioning information from github in a way that you can easily integrate into scripts, it might also be useful for you.

However, one thing was still missing from it: A real world use case as integration test. Now in rust, your dependencies are usually managed by cargo and can be updated with [cargo update](https://doc.rust-lang.org/cargo/commands/cargo-update.html). That is sensible for almost every other rust project, but as I wanted to test ghdepup itself, I shunned it and wrote a [github workflow](https://github.com/bjoernmichaelsen/ghdepup/actions/runs/9524791148/workflow) and [github action](https://github.com/bjoernmichaelsen/ghdepup/tree/main/.github/actions/selfupdate) that runs ghdepup on itself and suggests updates.

Should you use ghdepup now, when renovate and dependabot exist? In most current environments probably not, as the latter are more flexible and powerful. However, if you want an dependency updater that is so simple you wouldnt be scared to fork it anytime, you might want to give ghdepup a look. 


**Comments? Feedback? Additions? Most welcome [here on the fediverse](https://chaos.social/@Sweetshark/)** <img style="width:1.5em" src="/img/gh/mastodon.svg"/> **!**

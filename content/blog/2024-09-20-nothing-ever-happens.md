+++
title="Nothing ever happens"
date=2024-09-20T01:30:00+02:00
[taxonomies]
originally-published-on=["github.io"]
categories=["libreoffice"]
+++
Nothing ever happens.
=====================

<div style="text-align: right;"><em>And nothing ever happens, nothing happens at all</br>
The needle returns to the start of the song</br>
And we all sing along like before</br>
-- <a href="https://www.youtube.com/watch?v=TxbIU0X-lCI">Del Amitri, Nothing Ever Happens</a></em></div>

In my [last post on Libreoffice](../writer-again/) I promised to talk about Writer changes once in a while, but then ... nothing ever happened. However, given that I had an annoying motorcycle accident in the meantime that turned out much more persistently annoying than originally thought, I think I have a bit of an excuse.

So ... what did happen? For one, I fixed [quite a few regressions with my name on them](https://bugs.documentfoundation.org/buglist.cgi?action=wrap&f1=cf_regressionby&o1=anywords&resolution=---&resolution=FIXED&resolution=INVALID&resolution=WONTFIX&resolution=DUPLICATE&resolution=WORKSFORME&resolution=MOVED&resolution=NOTABUG&resolution=NOTOURBUG&resolution=INSUFFICIENTDATA&v1=michaelsen),
but ... is there much to talk about here? Mostly not: If you look at the fixes,
they are often oneliners fixing something that seems rather obvious in
retrospect. The more tricky question is: how did these get in in the first
place? Its hard for me to say that, as the introducing commits are from even
longer ago.

One thing is certain though: Often a unittest would have caught them, so
whenever possible, I tried to create a reproducer adding such a test with the
fix. To anyone writing bug reports: Creating minimal reproduction test is
hugely valuable in this -- not just for finding the issue, but also as a
starting point for a regression test. So if a bug bugs you and it is missing a
minimal reproduction scenario, adding one is a great way to move this forward.
Oh, and maybe [verifying a bugfix, if someone provided a
fix](https://bugs.documentfoundation.org/show_bug.cgi?id=153866) and the
friendly bot say affected users are "encouraged to test the fix and report
feedback".

While doing these fixes, I stumbled over [Noel suggesting to speed up bookmarks
in writer](https://gerrit.libreoffice.org/c/core/+/171406) which is of course
great, but I noticed that the code could be optimized a bit more as the
bookmarks of a document are now sorted by their starting position (which was
one of the first changes I made back on OpenOffice.org about more than a decade
ago). Thus we can use [bisectional search on the bookmarks
here](https://gerrit.libreoffice.org/c/core/+/171442), which should be even
faster. Now, it would be great if the discussion on this between Noel and me
would available for others to learn from, wouldnt it? The cool thing is: it is.

All discussion happened on [gerrit in the comments](https://gerrit.libreoffice.org/c/core/+/171442)
so if you want to learn about bookmark in Writer and how to maybe speed them up
for documents that have a lot of them, that is a great starting point! Is there
anything to add? Well maybe the following: Currently the bookmarks starting at
the same position are currently not sorted. If one would sort them by their end
position, the bisectional search could maybe cover even more? This would also
remove one extra loop of logic and make the code simpler and easier to read.

The performance improvement is likely irrelevant -- esp. since there will be
not that many documents with lots of bookmarks starting at the same position.
The simpler code might be worth it though. So why wasnt it done?

It still can be tried in a follow-up, but speaking about regressions earlier:
This has some obscure regression risk, because if we change the order of
bookmarks starting at the same position from undefined to something ordered by the
end position it might impact a lot of code using bookmarks. The function in
question might actually be faster, but other functions (e.g. the inserting of
new bookmarks) might actually be slower. So ... this is left as an exercise to
the reader.

**Comments? Feedback? Additions? Most welcome [here on the fediverse](https://chaos.social/@Sweetshark/113166795661703346)** <img style="width:1.5em" src="/img/gh/mastodon.svg"/> **!**

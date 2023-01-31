+++
title="Ubuntu 12.04 LTS (Precise Pangolin) released with LibreOffice 3.5.2, forward to 3.6 with bibisect!"
date=2012-04-28
[taxonomies]
originally-published-on=["livejournal"]
+++
Ubuntu 12.04 LTS (Precise Pangolin) released with LibreOffice 3.5.2, forward to 3.6 with bibisect!
==================================================================================================

<div style="text-align: right;"><em>
I was so much older then,</br>
Im younger than that now.</br>
-- My Back Pages, Bob Dylan</br>
</em></div>

Ubuntu release day is over and it ships with LibreOffice 3.5.2 -- the best free
office suite ever.

![a precise pangolin wallpaper](/img/lj/2012-04-28-precise-pangolin-wallpaper.png)

A big Thank You to everyone who contributed to make that possible: LibreOffice
developers, bugwranglers, testers, translators, documentation writers,
infrastructure admins, packagers, event organizers, administrative gremlins and
everyone I forgot.

In the best "the king is dead, long live the king"-fashion, lets look forward
to a great LibreOffice 3.6.X series which will be on the next Ubuntu release
named [Quantal Quetzal](http://www.markshuttleworth.com/archives/1121). And here is a piece to keep us moving quickly and
confidently:
[a first bibisect archive of LibreOffice 3.6/master](http://people.canonical.com/~bjoern/bibisect-3.6-20120328.tar.xz).
See ["How to Bibisect"](https://wiki.documentfoundation.org/index.php?title=QA/HowToBibisect) or
[Florians excellent video](https://www.youtube.com/watch?v=SA88flop4MM) for what to do with it -- the only
change is that this bibisect needs Ubuntu 12.04 (precise) instead of Ubuntu
11.10 (oneiric) now. I will provide updates for this bibisect along as we
approach the LibreOffice 3.6 release.

Bibisect already proved very helpful in triaging and preventing regressions in
LibreOffice 3.5 -- in LibreOffice 3.6, we will be able to even better control
the quality and pinpoint problems, since we have bibisect-3.6 continuing right
were bibisect-3.5 ended: At the start of the LibreOffice 3.5 release branch.
This will give us an enormous practical advantage in quickly identifing and
fixing issues in our huge codebase -- without the highly intrusive encumbrance
and overhead that the dogmatic classic approaches to QA enforce and burden
projects with<sup>(1)</sup>.

<sup>(1)</sup> We implement those too, but are only tightening them on the release
branches.

This was originally published at 2012-04-28 16:57:00/2012-04-28 16:57:00 on [livejournal](https://sweetshark.livejournal.com/11119.html).

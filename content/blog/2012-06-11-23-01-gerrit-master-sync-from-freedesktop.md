+++
title="gerrit master sync from freedesktop"
date=2012-06-11
[taxonomies]
originally-published-on=["livejournal"]
+++
gerrit master sync from freedesktop
===================================

The
[master](https://gerrit.libreoffice.org/gitweb?p=core.git;a=shortlog;h=refs%2Fheads%2Fmaster)
and the [3.5.x](https://gerrit.libreoffice.org/gitweb?p=core.git;a=shortlog;h=refs%2Fheads%2Flibreoffice-3-5)
and [3.6.x](https://gerrit.libreoffice.org/gitweb?p=core.git;a=shortlog;h=refs%2Fheads%2Flibreoffice-3-6)
release branches (links require OpenID login) on
[gerrit.libreoffice.org](https://gerrit.libreoffice.org/) are now
synced every 15 minutes by the friendly
[LibreOffice gerrit bot](https://launchpad.net/~r-gerrit-0)
from freedesktop. If you based your patch on these branches more than 15 minutes
ago, you can be sure to be able to send it there for review without any hassle.
Once we make the gerrit repository our reference, this syncing isnt needed
anymore of course (we would only need to push those changes to freedesktop then
-- the other way around). But for all practical proposes, everyone should be
able to submit his patches to gerrit with this.

This was originally published at 2012-06-11 23:01:00/2012-06-11 23:01:00 on [livejournal](https://sweetshark.livejournal.com/12605.html).

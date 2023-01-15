+++
title="pbuilder, bindmounts and ccache"
date=2011-12-18
[taxonomies]
originally-published-on=["livejournal"]
+++
pbuilder, bindmounts and ccache
===============================

Note to self: When building LibreOffice packages in a <a href="http://en.wikipedia.org/wiki/Debian_build_toolchain">pbuilder </a>(a chroot with bells and whistles) which bindmounts your ccache, it is not a Good Idea to kill the pbuilder process mercilessly. This is because debugging a <a href="http://catb.org/jargon/html/H/heisenbug.html">heisenbug</a> caused by a corrupted ccache is never fun.

This was originally published at 2011-12-18 01:52:00/2011-12-18 00:52:14 on [livejournal](https://sweetshark.livejournal.com/8006.html).

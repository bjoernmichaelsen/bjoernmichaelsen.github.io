+++
title="State and Overview of LibreOffice Ubuntu Packages"
date=2012-04-18
[taxonomies]
originally-published-on=["livejournal"]
+++
State and Overview of LibreOffice Ubuntu Packages
=================================================

<div style="text-align: right;"><em>
Come together, yeah</br>
Come together, yeah</br>
Come together, yeah</br>
Come together, yeah</br>
...</br>
-- the Beatles, Come Together</br>
</em></div>

Here is a short update on the state of LibreOffice Packaging on Ubuntu (and Debian):

* [Ubuntu 12.04 LTS (Precise)](https://launchpad.net/ubuntu/precise) will ship with at least [LibreOffice 3.5.2-2ubuntu1](https://launchpad.net/ubuntu/precise/+source/libreoffice)
* Thanks to the great work of [ricotz](https://launchpad.net/~ricotz), LibreOffice 3.5.2 is also available for [Ubuntu 11.10 (Oneiric)](https://launchpad.net/~libreoffice/+archive/ppa/+sourcepub/2375289/+listing-archive-extra) and [Ubuntu 10.04 LTS (Lucid)](https://launchpad.net/~libreoffice/+archive/ppa/+sourcepub/2375288/+listing-archive-extra) from the [LibreOffice PPA](https://launchpad.net/~libreoffice/+archive/ppa).
* There is a new [PPA for the LibreOffice 3.4 series](https://launchpad.net/~libreoffice/+archive/libreoffice-3-4). It contains the [latest minor release (3.4.6)](https://launchpad.net/~libreoffice/+archive/libreoffice-3-4/+sourcepub/2378562/+listing-archive-extra) of LibreOffice 3.4. From now on there will be a PPA for each LibreOffice major series (3.4, 3.5, 3.6) allowing users to update to the latest minor release of that series, without having to wait for the thorough process of [Stable Release Update](https://wiki.ubuntu.com/StableReleaseUpdates) verification. There will never be a surprise update to the next major release in these PPAs.

During the 3.4/3.5 upstream releases, Debian (and thus Ubuntu) switched from
building with the old [libreoffice-build wrapper](http://cgit.freedesktop.org/libreoffice/build) to a clean direct build. All
patches had to be either upstreamed or migrated from libreoffice-build to our
own vendor-patches. This is a lot of thankless work (as it risks regressions,
while nothing changes if everyting goes well). A quick "git log" against the
LibreOffice packaging repository from the 3.4.1-1 release to now (3.5.2) shows
more than 800 commits -- over 600 of those by Rene Engelhard, but also
including contributions by Lionel Elie Mamane, Nelson A. de Oliveira, Rico
Tzschichholz, Lubok Lunak.

These changes have made Ubuntu and Debian come a lot closer to upstream -- a
lot of patches have been either removed as obsolete, upstreamed to LibreOffice
or included as vendor-patches. With [only 41 Patches in debian/patches](http://anonscm.debian.org/gitweb/?p=pkg-openoffice/libreoffice.git;a=tree;f=patches;h=549a0141f723ef3c3b285ab97561b3e3e7961e64;hb=83cdc913d0d3c5d93ede9b0b41995afc908d729c) we are a
lot closer to upstream now, which is very good as it removes the need to
doublecheck bugs to be Debian/Ubuntu-specific in the most cases. Of the five
patches that are only in Ubuntu, but not in Debian:

* three are minor fixes that have been upstreamed to LibreOffice 3.6 (but are not backported to 3.5 there)
* two are backported upstream patches from LibreOffice 3.5.3 so they are in the LTS release from the start

This shows that a lot of friction in upstreaming patches has been removed --
and whatever we are patching in can and is upstreamed quickly to Debian or
LibreOffice, keeping our vendor patch queue small. Thanks to everyone who made
this possible -- most of all: Rene Engelhard.

This was originally published at 2012-04-18 12:37:00/2012-04-18 12:37:00 on [livejournal](https://sweetshark.livejournal.com/10977.html).

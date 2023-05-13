+++
title="Survived UDS, LibreOffice on Ubuntu Nexus 7 and my obsession with clicky keyboards"
date=2012-11-08
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
Survived UDS, LibreOffice on Ubuntu Nexus 7 and my obsession with clicky keyboards
==================================================================================

<p style="text-align:right;"><em><em>Dr. Soran: What if I told you I found a new truth?</em></em></p>
<p style="text-align:right;"><em>Picard: The nexus?</em></p>
<p style="text-align:right;"><em>-- Star Trek: Generations</em></p>
<p style="text-align:left;">So <a href="http://uds.ubuntu.com/">UDS</a> is over and you can find the results of what was discussed there showing up bit by bit on <a href="https://blueprints.launchpad.net/~bjoern-michaelsen">my blueprints</a> this week. Some highlights:</p>
<p style="text-align:left;">Ubuntu will extend its daily testbuilds (plus testsuite runs) to three scenarios:</p>

<ul>
	<li>Running a master build with mostly LibreOffice internal library versions</li>
	<li>Running a release build with mostly system libraries</li>
	<li>Running a QEMU ARM release build</li>
</ul>
The first is intended to find upstream bugs and regressions and I intend to have it integrated it with the set of other <a href="http://tinderbox.libreoffice.org/MASTER/status.html">upstream tinderboxes</a> churning away at constantly building and testing LibreOffice. The second is more important to find regressions in the system libraries that LibreOffice depends on -- as the release branch should be free of build or test failures in general given the review requirements for pushing to it. So it will test for stuff outside of LibreOffice breaking LibreOffice (like <a href="https://bugs.launchpad.net/ubuntu/+bug/1017125">lp#1017125</a> or <a href="https://bugs.launchpad.net/ubuntu/+bug/745836">lp#745836</a>) on Ubuntu. The third will hopefully improve the visibility of bugs in the ARM build.

Speaking of ARM: Ubuntu is <a href="https://wiki.ubuntu.com/Nexus7/Installation">easily installable on an Nexus 7</a> and I had to try it out obviously. Not only Ubuntu also <a href="https://www.libreoffice.org/">LibreOffice</a> is running quite nicely on it as you can see in this video (note that I even opened the document template twice in my inability to use a mouse -- I prefer keyboards):

[youtube http://www.youtube.com/watch?v=Rcby0jlsQQI&amp;;w=560&amp;h=315]

<a title="Link: LibreOffice on Ubuntu Nexus 7 and a Model M" href="http://www.youtube.com/watch?feature=player_embedded&amp;v=Rcby0jlsQQI">Link: LibreOffice on Ubuntu Nexus 7 and a Model M</a>
Other stuff discussed at the UDS: Some boring packaging detail, fixing up the upstream packaging with the hope of getting more logic directly into upstream instead of fixing around it in our ./debian/rules. We also had a discussion on bibisect and if it can be applied to Ubuntu as a whole -- discovering some overlap with <a href="http://snapshot.debian.org/">http://snapshot.debian.org/</a> -- still bibisect might be something to consider for example for GNOME.
I also learned that <a href="https://errors.ubuntu.com/">https://errors.ubuntu.com/</a> is shaping up nicely, and expect that to become even more awesome combined with <a href="https://wiki.ubuntu.com/PhasedUpdates">phased updates</a> -- esp. for LibreOffice.

Finally: <strong>Valve/Steam on Ubuntu -- awesome!</strong>

[caption id="" align="aligncenter" width="864"]<img title="Steam/Valve on Ubuntu" alt="" src="https://lh3.googleusercontent.com/-svMSo1fjdBY/UI6G0skuHnI/AAAAAAAAAg4/JhJE_69PqXA/s864/20121029_141544.jpg" height="648" width="864" /> Steam/Valve at UDS[/caption]
Originally published on 2012-11-08 10:42:51 on [wordpress](https://skyfromme.wordpress.com/2012/11/08/survived-uds-libreoffice-on-ubuntu-nexus-7-and-my-obsession-with-clicky-keyboards/).

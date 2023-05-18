+++
title="LibreOffice rules"
date=2012-08-23
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice rules
=================

<p style="text-align:right;"><em>I love deadlines. I like the whooshing sound they make as they fly by.</em></p>
<p style="text-align:right;">-- Douglas Adams</p>
So yesterday was <a href="https://wiki.ubuntu.com/QuantalQuetzal/ReleaseSchedule">Ubuntu Quantals feature freeze</a> and two important features made it in with the <a href="https://launchpad.net/ubuntu/+source/libreoffice/1:3.6.0~rc4-0ubuntu3">libreoffice-3.6.0~rc4-0ubuntu3 package</a>:
<ul>
	<li>the awesome work by Antonio Fernandez from Aentos (sponsored by Canonical) on the <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=shortlog;h=refs/heads/feature/unitymenus">feature/unitymenus branch</a></li>
	<li>some PackageKit integration for LibreOffice on Ubuntu (will be upstreamed for 3.7)</li>
</ul>
I'll go into the details of both and what they mean for endusers in later blogposts as they each deserving one of their own. Also, for the current stable Ubuntu 12.04 LTS release we have in:
<ul>
	<li>the <a href="https://launchpad.net/~libreoffice/+archive/ppa?field.series_filter=precise">LibreOffice PPA</a> a current backport of LibreOffice 3.6.0</li>
	<li>the <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-3-5?field.series_filter=precise">LibreOffice 3.5 PPA</a> a current update to LibreOffice 3.5.6</li>
</ul>
This allows users to be up-to-date with LibreOffice on the current stable release of Ubuntu too. Ok, so we are running a tight ship for LibreOffice on Ubuntu, but that is kind of expected, right? Well, yes -- but I want to reach out to another point, which is: how we got there. To illustrate that, I want to select some random datapoints on the debian/rules file, which is the core file that makes a package out of a plain upstream build. The situation has improved since the beginning of LibreOffice (LibreOffice-3.3.0-1):
<ul>
	<li>The debian/rules file, while probably still one of the most "impressive" of all of Ubuntu, shrank by 12% since that first LibreOffice release on Debian and is now at 3173 lines. Removing complexity here is a Good Thing<sup>(tm)</sup> and hopefully will continue.</li>
	<li>Less than 3% of the lines are different between Ubuntu and Debian by now in the rules file. That is a Good Thing<sup>(tm)</sup>. While there certainly are some differences between the distributions, if such vendor changes need modifications in the rules file it is usually a sign of bad design.</li>
	<li>Finally, since that first release, there have been 640 commits made touching that file -- 90% of those by Rene Engelhard, 10% by me. In total 1092 lines have been added and 1541 lines have been removed -- meaning at least 1/3 of that file has been rewritten. Now, only 1.5 KLOC delta does not sound much, but with a turnaround time of <a href="https://launchpad.net/ubuntu/+source/libreoffice/1:3.6.0~rc4-0ubuntu3/+build/3733805">1.3 days on some architectures</a> even a dedicated machine would not keep up with that for every commit. Also: I bet the rules file of a lot of other Debian/Ubuntu packages will fit in that 1.5 KLOC (or even in the 449 lines we lost since the first release).</li>
</ul>
Finally, this tasty pie chart shows that the red to yellow upstream parts of the rules file are not that big. Most of the rules file is concerned with mapping the build we want to do to both ./configure switches and dependencies<sup>(*)</sup> and splitting up the build result:

<a href="/static/img/wp/2012/08/pkgstats.png"><img class="aligncenter size-full wp-image-130" title="" src="/static/img/wp/2012/08/pkgstats.png" alt="" width="519" height="389" /></a>

This "last mile" of getting LibreOffice on Ubuntu and Debian is often overlooked. I still think it is quite an important (although mostly invisible) job. An explicit "Thank You!" to Rene for all his hard and continuous work on this.

<sup>(*)</sup> Yes, LibreOffice has a rule to generate its own control file. Manually maintaining that would be truely painful.

Originally published on 2012-08-23 19:37:49 on [wordpress](https://skyfromme.wordpress.com/2012/08/23/libreoffice-rules/).

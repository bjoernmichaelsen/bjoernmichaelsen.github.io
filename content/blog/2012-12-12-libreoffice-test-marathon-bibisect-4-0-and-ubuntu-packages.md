+++
title="LibreOffice Test Marathon, Bibisect 4.0 and Ubuntu packages"
date=2012-12-12
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice Test Marathon, Bibisect 4.0 and Ubuntu packages
===========================================================

<p style="text-align:right;"><em>“Linux does endless loops in six seconds”</em></p>
<p style="text-align:right;"><a href="http://www.linuxjournal.com/article/1026"><em>-- allegedly Linus Torvalds, 1995 at the First Dutch International Symposium on Linux</em></a></p>

<address> </address><address>The  <a href="http://wiki.documentfoundation.org/QA/Test_Marathon_LibreOffice_4.0">LibreOffice 4.0 Test Marathon</a> will start on Friday, and this is a call to arms to join in on the fun! ;)</address>
<p style="text-align:center;"><a href="http://wiki.documentfoundation.org/QA/Test_Marathon_LibreOffice_4.0" rel="attachment wp-att-278"><img class="aligncenter size-full wp-image-278" alt="GIn_BHS400-800" src="/img/wp/2012/12/gin_bhs400-800.png" width="519" height="75" /></a></p>
Now, while a lot of people will join the test marathon just for the good cause, for others it might need some candy to persuade them. So here is some sweet sugar:
<h1>Bibisect 4.0</h1>
<a href="http://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/">Dicke Bertha</a> had her burn-in in the last days. She compiled 146 full builds of the LibreOffice master branch from May 2012 up till now.

It took her some 25 hours churning away with a load average &gt;32 -- interrupted only twice: Once because I left two other LibreOffice compiles lying around on tmpfs before starting this and then ran out of tmpfs buildspace -- whoopsie!

A second time the build run was briefly interrupted because a cppunittest loved Bertha so much it went into an endless loop. And despite Linus' claim above, Linux even 17 years later does <strong>not</strong> do endless loops in six seconds (also: where is my flying car?).

Just for fun, here are some ccache stats from the full run:
<pre>cache hit (direct)               1404831
cache hit (preprocessed)          144192
cache miss                        677524</pre>
I combined that with the builds from the <a href="http://wiki.documentfoundation.org/QA/HowToBibisect">older bibisect runs</a> into one big respository. Thus in this <a href="http://people.canonical.com/~bjoern/bibisect-4.0.tar.xz"><strong>4.1 GB download</strong> </a>you have:
<ul>
	<li><strong>262 full builds of LibreOffice (~16MB per install)
</strong></li>
	<li><strong>covering a range of 26365 commits since August 2011</strong></li>
	<li><strong>thats one build every ~100 commits</strong></li>
</ul>
Thus if you notice a regression that has been introduced at some point in the last 16 months, you can run a <a href="https://www.youtube.com/watch?v=SA88flop4MM">binary bisect</a> and after <strong>testing 9 times in different versions</strong> of LibreOffice if the bug is there, you will have <strong>the regression pinned down to a range of ~100 commits</strong>, at which point it will be much easier <a href="https://www.youtube.com/watch?v=QZJC5ORod7I"><del>to sack those responsible</del></a> fix the bug quickly. So how can you help with this? In two ways:
<ul>
	<li>read <a href="http://wiki.documentfoundation.org/QA/HowToBibisect">HowToBibisect</a> and start bibisecting ;)</li>
	<li>if you dont want to do that, you can still help to find bugs that are regressions and can be tested well (on Ubuntu) and mark them with "bibisectrequest" in whiteboard status</li>
</ul>
<h1>Testcase management</h1>
Thanks to the work of Yifan, Sophie, Petr and many others the Document Foundation now has a <a href="http://vm12.documentfoundation.org/manage/cases/">MozTrap instance</a> -- and in this marathon we will find out how its workflow integrates with the rest of LibreOffices QA and development processes. So what is MozTrap? Their own <a href="https://moztrap.readthedocs.org/en/1.0.X/">documentation webpage</a> explains lengthly and eloquently: " MozTrap is a test case manager.". So we could again run to wikipedia and read that <a href="http://en.wikipedia.org/wiki/Test_management_tools">short article</a> about that, but that would not be much fun.

If you cut past all the buzzwords, you find that 'test case management' means, we have a web tool that:
<ul>
	<li>manages a set of things to test with some kind of software (in the simplest case this might be: "Does LibreOffice start?")</li>
	<li>shows testers simple instructions to perform and report back if everything works as expected</li>
	<li>then allows QA people do all kinds of statistics and voodoo on this data ;)</li>
</ul>
So, it is really simple and you are invited to join in! As a bonus you can make sure that developers know now, if something broke since the last release and since the release in February is still a bit off have more time to fix it until then!

Not every bug is as beautiful as this one ... (Photo: Jessica Lucia, Creative Commons license):

![Firefly](http://nextdoornature.files.wordpress.com/2011/07/firefly-by-jessica-lucia-cc.jpg?w=800&amp;h=533")

<h1>Ubuntu packages for alpha/beta releases</h1>
On the <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-prereleases">LibreOffice prereleases PPA</a> you find a packaged version of <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-prereleases/+sourcepub/2834822/+listing-archive-extra">LibreOffice 4.0 beta1 for Ubuntu Raring</a> that you can use in the test marathon. There is also a version of <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-prereleases/+sourcepub/2803189/+listing-archive-extra">LibreOffice 4.0 alpha1 for Ubuntu 12.10 (Quantal)</a> there, which should you also be able to use for discovering bugs.

I have a beta1 package for Ubuntu 12.10 ready too, but it is cheating quite a bit, because I disabled Python as the beta1 requires Python3.3 which is not directly available on Ubuntu 12.10.
<h1>Ubuntu stable release updates</h1>
Just for completeness: For Ubuntu 12.04 LTS a stable release update to the final (for the series) <a href="http://launchpadlibrarian.net/124327382/libreoffice_3.5.7-0ubuntu2_source.changes">3.5.7 has been uploaded</a> (actually it has even been updated while waiting in the queue). For Ubuntu 12.10, version 3.6.4 is currently in probation in the <a href="https://launchpad.net/~libreoffice/+archive/ppa/+packages?field.name_filter=&amp;field.status_filter=published&amp;field.series_filter=quantal">LibreOffice PPA</a>. Should no problems turn up, it will be proposed become a update soon.

Originally published on 2012-12-12 14:29:26 on [wordpress](https://skyfromme.wordpress.com/2012/12/12/libreoffice-test-marathon-bibisect-4-0-and-ubuntu-packages/).

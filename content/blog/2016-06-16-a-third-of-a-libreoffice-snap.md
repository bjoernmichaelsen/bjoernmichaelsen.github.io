+++
title="A third of a LibreOffice snap"
date=2016-06-16
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
A third of a LibreOffice snap
=============================

<p style="text-align:right;"><em>Take your time, hurry up</em>
<em> The choice is yours, don't be late</em>
<em> Take a rest as a friend</em>
<a href="https://www.youtube.com/watch?v=vabnZ9-ex7o">-- Nirvana, Come As You Are</a></p>
<p style="text-align:left;">I have just updated the LibreOffice snap package. The size of the package available for download created some confusion. As LibreOffice 5.2 is still in beta, I built and packed it with full debug symbols to allow analysis of possible problems. Comparing this to the size of e.g. the default install from Ubuntu *.deb packages is misleading:</p>

<ul>
	<li style="text-align:left;">The Ubuntu default install misses LibreOffice Base and Java unless you explicitly install them</li>
	<li style="text-align:left;">The Ubuntu default install misses debug symbols unless you install the package <code>libreoffice-dbg</code> too</li>
</ul>
As many people are just curious about running LibreOffice 5.2 without wanting to debug it right now, I replaced the snap package. The download and install instructions are still the same as <a href="https://skyfromme.wordpress.com/2016/06/14/libreoffice-5-2-0-beta2-as-a-snap-package/">noted here</a> -- but it is <strong>now 287MB</strong> instead of 1015MB (and it still contains Base, but no debug symbols).

The package file including full debug symbols -- in case you are interested in that -- has been renamed to <a href="http://people.canonical.com/~bjoern/snappy/libreoffice-debug_5.2.0.0.beta2_amd64.snap">libreoffice-debug</a>.

<small>(Note that if you downloaded the file while I moved files around, you might need to redo your download.)</small>

Originally published on 2016-06-16 10:42:07 on [wordpress](https://skyfromme.wordpress.com/2016/06/16/a-third-of-a-libreoffice-snap/).

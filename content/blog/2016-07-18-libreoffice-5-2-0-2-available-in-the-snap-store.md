+++
title="LibreOffice 5.2.0.2 available in the snap store"
date=2016-07-18
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice 5.2.0.2 available in the snap store
===============================================

<p style="text-align:right;"><em>'Cause the players gonna play, play, play, play, play</em>
<em> And the haters gonna hate, hate, hate, hate, hate</em>
<em><a href="https://www.youtube.com/watch?v=nfWlot6h_JM">— Taylor Swift, 1989, Shake It Off</a></em></p>
The latest release candidate of the upcoming LibreOffice 5.2.0 feature release is available for installation from the snap store. This makes it very easy to install this prerelease of LibreOffice for testing out new features (an incomplete glimpse on what to look forward for can be found on the <a href="https://wiki.documentfoundation.org/ReleaseNotes/5.2">LibreOffice 5.2 release notes page, which is still under construction</a>, go on <a href="https://wiki.documentfoundation.org/QA/IRC">#libreoffice-qa</a> if you want to help with testing).

To install this build of LibreOffice on any snap supported platform just open a terminal and run:
<pre>sudo snap install --channel=beta libreoffice</pre>
To start this version of LibreOffice, you run:
<pre>/snap/bin/libreoffice</pre>
The full path should only be needed, if you have another version of LibreOffice installed. If that is not the case a plain "libreoffice" should do.

Note that this version is still a prerelease and not for production use yet. That said, it is mostly a full-featured package including everything that would be packaged for end users of LibreOffice. While this package also includes a set of localizations to show that they work, their number has been restricted to English, French, German, Italian, Portuguese (Portugal/Brazil), Spanish for size considerations for now. This set is mostly the one Ubuntu provides on its installer images (removing those that might have issues as they need special fonts).

Another difference to <a href="https://skyfromme.wordpress.com/2016/06/14/libreoffice-5-2-0-beta2-as-a-snap-package/">prior downloads</a> is that while LibreOffice still uses X11, now runs in <a href="https://insights.ubuntu.com/2016/05/04/security-confinement-in-ubuntu-core/">confinement provided by snaps</a>. Unlike previous releases on Ubuntu, this package defaults now to do so via the newer GTK3 backend: This has a lot of advantages, see details on <a href="http://caolanm.blogspot.de/">Caolans Blog</a>, but it is also a younger backend, that hasnt has that much time to be polished yet.
Originally published on 2016-07-18 13:23:23 at https://skyfromme.wordpress.com/2016/07/18/libreoffice-5-2-0-2-available-in-the-snap-store/.

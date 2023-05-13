+++
title="LibreOffice Debian/Ubuntu Team growing"
date=2013-03-07
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice Debian/Ubuntu Team growing
======================================

<p style="text-align:right;"><em>"Ich danke der Academy für das Erkennen von Talent."</em></p>
<p style="text-align:right;"><em>-<a href="http://www.youtube.com/watch?v=jwvBd7MX-Ng">- Ich danke der Akademie, Kettcar</a></em></p>
So, I just updated the <a href="https://launchpad.net/~libreoffice/+archive/ppa?field.series_filter=raring">LibreOffice version for Ubuntu Raring in the LibreOffice PPA to the 4.0.1~rc2 version</a>, that was announced as <a href="http://blog.documentfoundation.org/2013/03/06/the-document-foundation-announces-libreoffice-4-0-1/">4.0.1 final yesterday</a>. As we dropped the <a href="https://launchpad.net/ubuntu/quantal/+package/libreoffice-filter-binfilter">old binfilter file import</a> upstream in LibreOffice 4, LibreOffice fits again in the ppa with all localizations -- previously the PPA versions were <a href="http://anonscm.debian.org/gitweb/?p=pkg-openoffice/libreoffice.git;a=commitdiff;h=461cc1bb329e6231dd3199fdeae7e08c61d72d2d;hp=d8e4793a1e7e6cf7eca25fb0598c0d0d2c6db985">restricted to a subset of languages</a>, as the buildds for the PPAs might run out of disc space otherwise. Oh, and a boring sidenote just for completeness: LibreOffice on Ubuntu LTS has been getting a <a href="https://launchpad.net/ubuntu/+source/libreoffice/1:3.5.7-0ubuntu4">stable release update to 3.5.7</a> a while ago. And now that 4.0.1 rc2 is declared final, I expect it to be sponsored soon to Ubuntu Raring proper.

I would like to take this opportunity to thank everyone who made this possible:
<ul>
	<li>Rene Engelhard for his work on the Debian LibreOffice packaging, making all of this possible</li>
	<li><a href="https://launchpad.net/~bdcomp">Boaz Dodin</a> and <a href="https://launchpad.net/~ricotz">Rico Tzschichholz</a> for their tireless work and initiative on backporting current releases</li>
	<li><a href="https://launchpad.net/~penalvch">Christopher M. Penalver</a> for churning through LibreOffice bugs in launchpad and on freedesktop, syncing and triaging them</li>
	<li><a href="https://launchpad.net/~bdrung">Benjamin Drung</a> for fixes, tweaks and contributions to the LibreOffice packaging both at Debian and Ubuntu</li>
</ul>
Will there be backports of 4.0.1 to older series? Well, an indiscreet look in <a href="https://launchpad.net/~ricotz/+archive/ppa/+builds?build_state=building">Ricos PPA</a>, suggests that it wont take long for them to end up in the <a href="https://launchpad.net/~libreoffice/+archive/ppa">LibreOffice ppa</a>. Steadily, the LibreOffice team is growing -- its exciting to see so many volunteers contribute to this!
Originally published on 2013-03-07 12:42:16 at https://skyfromme.wordpress.com/2013/03/07/libreoffice-debianubuntu-team-growing/.

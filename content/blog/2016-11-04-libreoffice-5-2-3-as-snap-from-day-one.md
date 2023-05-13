+++
title="LibreOffice 5.2.3 as snap from Day One"
date=2016-11-04
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "snap", "snapcraft", "snappy", "ubuntu"]
+++
LibreOffice 5.2.3 as snap from Day One
======================================

<p style="text-align:right;"><em>Meist scheint manches auf den ersten Blick unmöglich.</em>
<em> Manches ist es auch, doch es wäre tödlich, das selbst zu glauben solange noch nichts feststeht</em>
<em> und die Party zu verlassen, bevor sie losgeht.</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=Di0T3rVaV6Y">— Die Sterne, Stell die Verbindung her</a></em></p>
<p style="text-align:left;">Yesterday, two nice things happened: For one, <a href="https://blog.documentfoundation.org/blog/2016/11/03/announcement-of-libreoffice-5-2-3/">LibreOffice 5.2.3 has been released</a> and secondly <a href="https://insights.ubuntu.com/2016/11/03/ubuntu-core-16-delivers-foundation-for-secure-iot/">Ubuntu Core 16 has been released</a>. But beyond that, something in the middle between these two has happened: LibreOffice 5.2.3 has been released to the stable channel of the snap store at the same day. Now LibreOffice has been in the snap store for some time — and has also been on the stable channel since the Ubuntu 16.10 release. But this is the first time the LibreOffice snap is released in sync with <a href="http://www.documentfoundation.org/">The Document Foundation</a> announcing the general availability of the final downloads. This was possible even though I was on vacation yesterday: LibreOffice snap packages are now being build on launchpad, which simplifies a lot, and launchpad can be asked to populate the <em>edge channel</em> of the store. This is making life very easy. Having smoketested the amd64 build from that channel before, to release LibreOffice 5.2.3 to the <em>beta/candidate/stable channels </em>too all I had to do was push three buttons on a web interface and it was available to all.</p>
<p style="text-align:left;">Building on launchpad, I also had the opportunity to create builds for armhf and i386 along with the usual amd64 builds with little extra effort. If you are adventurous you are encouraged to test these builds too: Be aware though that these so far aren’t even smoketested, I havent looked at them at all yet, so use them at your own risk.</p>
<p style="text-align:left;">All in all, this is great progress: LibreOffice 5.2.3 is available to users of Ubuntu 16.10 and Ubuntu 16.04 LTS as a snap on the day of the upstream release. And beyond that on all other distributions where snap is available — <a href="http://snapcraft.io/">quite a few these days</a>.</p>
<p style="text-align:left;"><strong>Update:</strong> ICYMI here is how to get the LibreOffice snap: <a href="http://www.libreoffice.org/download/snap/">http://www.libreoffice.org/download/snap/</a> — although strictly speaking you dont need the <code>--channel=beta</code> option anymore now. I will fix that soon.</p>
<p style="text-align:left;"></p>
Originally published on 2016-11-04 16:52:42 at https://skyfromme.wordpress.com/2016/11/04/libreoffice-5-2-3-as-snap-from-day-one/.

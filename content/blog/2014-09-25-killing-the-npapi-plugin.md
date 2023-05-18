+++
title="Killing the NPAPI plugin"
date=2014-09-25
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice"]
+++
Killing the NPAPI plugin
========================

<p style="text-align:right;"><em>"Alles hat ein Ende nur die Wurst hat zwei."</em>
<a href="http://www.youtube.com/watch?v=r8PPYNxhLCU"><em> -- horrible 1980ies german folk song lyrics</em></a></p>
<p style="text-align:left;">So, <a href="http://lists.freedesktop.org/archives/libreoffice/2014-June/061914.html">three month ago</a>, the <a href="https://wiki.documentfoundation.org/ESC">ESC</a> discussed if we would still support the NPAPI plugin to show documents in the browser. The discussion was ignited over an mostly innocent bug: <a href="https://bugs.freedesktop.org/show_bug.cgi?id=45071">fdo#45071</a>, but the discussion soon dived into long-term viability of this technology and with Chrome <a href="http://blog.chromium.org/2013/09/saying-goodbye-to-our-old-friend-npapi.html">"phasing out NPAPI support over the coming year"</a> and as Mozilla Firefox <a href="https://blog.mozilla.org/futurereleases/2013/09/24/plugin-activation-in-firefox/">"will no longer activate most plugins by default"</a> it became quickly clear that trying to keep the plugin alive would be throwing good developer time after bad. So in LibreOffice 4.4.x there will not be a NPAPI plugin anymore, as the patch that was <a href="https://gerrit.libreoffice.org/#/c/10757/">waiting on gerrit for a month</a> is now <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=commit;h=496bc3cd6c037360632c33471c4622a095cdcadd">pushed to the master branch</a>. As by the time of release plugins will not be enabled anymore on the most common browsers using it, this likely will not make much of a difference to most users anyway.</p>
<p style="text-align:left;">Luckily, LibreOffice is not only deprecating old technologies when they become more and more irrelevant -- it is also moving towards <a href="http://blog.documentfoundation.org/2014/09/04/tender-for-base-framework-for-an-android-version-of-libreoffice-with-basic-editing-capabilities-201409-01/">new technologies when they gain relevance</a>.</p>

Originally published on 2014-09-25 15:02:49 on [wordpress](https://skyfromme.wordpress.com/2014/09/25/killing-the-npapi-plugin/).

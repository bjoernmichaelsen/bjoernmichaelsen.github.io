+++
title="Going mobile"
date=2015-07-10
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++ mobile libreofficekit", "libreoffice", "ubuntu"]
+++
Going mobile
============

<p style="text-align:right;"><em>When I'm drivin' free, the world's my home</em>
<em> When I'm mobile</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=kxoO5yrabfc">-- The Who, Who's Next, Going Mobile</a></em></p>
<p style="text-align:left;"><a href="http://news.softpedia.com/news/ubuntu-touch-to-receive-a-libreoffice-viewer-core-app-calendar-sync-improvements-485435.shtml">As you might have noticed</a>, work has started to integrate LibreOffice with the document viewer of Ubuntu core apps. Here is a screenshot of how the current code renders documents on a mobile device:</p>
<p style="text-align:left;"><a href="https://skyfromme.files.wordpress.com/2015/07/uepgpzv.png"><img class="aligncenter size-full wp-image-1102" src="https://skyfromme.files.wordpress.com/2015/07/uepgpzv.png" alt="Ubuntu core apps: LibreOffice and document viewer" width="400" height="628" /></a></p>
<p style="text-align:left;">Kudos for integrating this go entirely to <a class="sprite person" href="https://launchpad.net/%7Everzegnassi-stefano">Stefano Verzegnassi</a>, all I did was providing a <a href="http://bazaar.launchpad.net/~verzegnassi-stefano/ubuntu-docviewer-app/lo-plugin-prototype/revision/152">tiny piece of example code</a>. It loads a document and saves a rendered version of the document to a PNG file. The relevant part of that piece of C++ code is small enough to fit in one picture shown here, including build instructions et al., showing how easy it is to use LibreOfficeKit from outside LibreOffice now:</p>
<p style="text-align:left;"> <a href="https://skyfromme.files.wordpress.com/2015/07/libreoffice2png.png"><img class="aligncenter size-large wp-image-1104" src="https://skyfromme.files.wordpress.com/2015/07/libreoffice2png.png?w=680" alt="libreoffice2png source code" width="680" height="383" /></a></p>
<p style="text-align:left;">Thus the doc viewer was quickly integrated with LibreOffice in a basic way. This proof of concept isnt finished however: It just renders the all the document in one buffer. For small documents, this is reasonable, for bigger documents, tiled rendering -- which LibreOfficeKit nicely supports from the API by allowing you to render any part of a document in a buffer -- needs to be implemented on the clientside. The code for this can be <a href="https://code.launchpad.net/~verzegnassi-stefano/ubuntu-docviewer-app/lo-plugin-prototype">found on launchpad</a>, so if you are just curious how this works you are invited to have a look. If you are interested in helping out with moving this forward towards a nice all-around document viewer reading and rendering everything LibreOffice can, you are most welcome!</p>
<p style="text-align:left;"><strong>Update:</strong> A picture says more than a thousand words, but a video tells a whole story. Stefano created this awesome video, which you <a href="https://www.youtube.com/watch?v=Itjxlzr8Bmk">shouldnt miss</a>:</p>
<p style="text-align:left;">[embed]https://www.youtube.com/watch?v=Itjxlzr8Bmk[/embed]</p>
Originally published on 2015-07-10 14:58:50 at https://skyfromme.wordpress.com/2015/07/10/going-mobile/.

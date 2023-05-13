+++
title="LibreOffice IDE integration"
date=2013-12-04
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice IDE integration
===========================

<p style="text-align:right;"><em>Sometimes I wonder if the world's so small,</em>
<em> Can we ever get away from the sprawl?</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=rH_7_XRfTMs"><em>Sprawl II -- Arcade Fire</em></a></p>
<p style="text-align:left;">So these days, most people prefer to use an IDE to navigate their source code. This has often been greeted with some defensive elitism of the <a href="http://xkcd.com/378/">"real programmers"</a> kind since the early days of the open sourcing of StarOffice. <a href="http://i1.kym-cdn.com/photos/images/original/000/038/834/drive_into_mordor.png">One does not simply load</a> a code base the size of LibreOffice in your wimpy IDE: while it is possible somehow in the end, its a lot more trouble than its worth to manually set up e.g. all the include path manually to get the fancy stuff like autocompletion. Add to that, that e.g. UNO headers are generated during the build and header were at distributed over multiple IDE unfriendly locations, with many headers even available as copies from multiple locations, <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=commit;h=b9337e22ce1dbf2eba0e8c8db294ae99f4111f91">before we fixed that</a>.</p>
<p style="text-align:left;">All these things are fixed now. And while LibreOffice still is a huge beast with our new build system we can get a holistic view of what needs to get build where, how and when. This makes it easy, almost trivial to generate an IDE project file from the build system. And to prove this point, I did just that for the kdevelop IDE. This isnt limited in principle to this one IDE -- in fact the kdevelop specific part of this is some <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=blob;f=bin/gbuild-to-ide;h=00a22cfc53b6391341169db1953a23d8d1f15def;hb=HEAD">150 lines of Python</a>. So no matter what IDE you use: Eclipse, Netbeans, Anjuta, Visual Studio, Code::Blocks or XCode -- you should be able to adapt this. In fact, while writing this, I find there is already work going on for XCode. Feel invited to join the party and make LibreOffice trivially buildable in your favourite IDE!</p>
<p style="text-align:left;"><a href="http://nabble.documentfoundation.org/Building-LibreOffice-from-an-IDE-td4083960.html">So as announced to the developer list</a>, this allows you to make navigating, editing, building, testing and running LibreOffice much easier, giving you features like:</p>

<ul>
	<li>autocompletion</li>
	<li>building a module from the <span class="search-highlight">IDE</span></li>
	<li>building all of LibreOffice from the <span class="search-highlight">IDE</span></li>
	<li>nondebug and debug build configs for the above</li>
	<li>starting LibreOffice from the <span class="search-highlight">IDE</span></li>
	<li>running unitchecks, slowchecks and subsequentchecks from the <span class="search-highlight">IDE</span></li>
</ul>
Dont believe it? Here is a <a href="http://www.youtube.com/watch?v=-5hVXeHNt2M&amp;hd=1">video</a> featuring a stuttering german guy (me) on the audio track showing this:

http://www.youtube.com/watch?v=-5hVXeHNt2M&amp;hd=1

If you want to show this around on social media, there is also a <a href="https://www.youtube.com/watch?v=Shdfi_RKb8s&amp;hd=1">shorter version</a> featuring the essentials (make sure to link to the HD versions).

A closing note: A long time, common IDEs embrace and extended into the buildsystems so once you used an IDE, you could only use this one IDE and no other. In retrospect, this is obviously <a href="http://i2.kym-cdn.com/photos/images/original/000/000/144/wrong05.jpg">doing it wrong</a>. With the current approach, we can make LibreOffice easily buildable in any IDE on any platform. A very important fact for a product available on so many platforms.

<strong><em>addendum:</em></strong> As Karl Fogel wrote <strong>"<a title="Permanent Link: Credit where credit is due: LibreOffice is now ridiculously easy to build." href="http://www.rants.org/2013/07/28/libreoffice_insanely_easy_build_process/" rel="bookmark">LibreOffice is now ridiculously easy to build.</a>"</strong> <em>before</em> we even had this, it just shows that one can always do better. ;)

Originally published on 2013-12-04 10:02:18 on [wordpress](https://skyfromme.wordpress.com/2013/12/04/libreoffice-ide-integration/).

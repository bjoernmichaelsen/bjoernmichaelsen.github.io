+++
title="Liberated Build System: Mission accomplished!"
date=2014-02-09
[taxonomies]
originally-published-on=["wordpress"]
categories=["build system", "fosdem", "libreoffice", "slides", "talk", "ubuntu"]
+++
Liberated Build System: Mission accomplished!
=============================================

<p style="text-align:right;"><em>It's so hard when it doesn't come easy</em></p>
<p style="text-align:right;"><em>It's so hard when it doesn't come fast</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=Avpq7wjFRDA"><em>-- So Hard, Taking The Long Way, Dixie Chicks</em></a></p>

<p style="text-align:left;">So, <a href="http://blog.documentfoundation.org/2014/01/30/libreoffice-4-2-focusing-on-performance-and-interoperability-and-improving-the-integration-with-microsoft-windows/">LibreOffice 4.2 is released</a>, <a href="https://fosdem.org/2014/">FOSDEM</a> is over, was very nice and I am back home in Hamburg after a week in London. I missed the <a href="https://wiki.documentfoundation.org/Hackfest/FOSDEM2014">LibreOffice UX Hackfest</a> for that, which I heard was also awesome. So without further ado, here are the slides from my quick talk at FOSDEM:</p>

<!-- https://speakerdeck.com/sweetshark1/liberated-build-system-mission-accomplished -->

<a href="https://speakerdeck.com/sweetshark1/liberated-build-system-mission-accomplished">(direct link if you are watching this on a planet that does not support embedded speakerdecks: https://speakerdeck.com/sweetshark1/liberated-build-system-mission-accomplished)</a>

and some errata for it: On slide 13 it says "the same file is also hardlinked from workdir/" -- thats not true for quite a while already. <strong>LibreOffice keeps around exactly one copy of a library</strong>, unlike the confusing three copies that we had in LibreOffice 3.3. This should be a lot less confusing to the curious first time contributor.

Reviewing all these changes in toto, it became how much we simplified getting involved with LibreOffice through this. As the lyrics quoted above say: <a href="https://www.youtube.com/watch?v=Avpq7wjFRDA"><em> "Back when we started, we didn't know how hard it was"</em></a>.

If there is just one number to take away from all these slides, its that a noop rebuild for LibreOffice on a three year old developer notebook with the distro provided GNU make 3.81 takes just <strong>17 seconds</strong><sup>(*)</sup>. And slide 7 shows still some possibilities to still speed things up beyond that -- and while at current speeds it might not be worth it on Linux, it might be worthwhile for e.g. Windows, which is traditionally rather slow when it comes to file I/O.

On a related note, over time we improved the way new contributors can submit their changes on our <a href="https://gerrit.libreoffice.org">instance of gerrit</a> in many ways. Thanks a lot to David, Norbert and Robert for the work on this. One only has to look at<a href="http://nabble.documentfoundation.org/LibreOffice-Gerrit-News-for-core-on-2014-02-08-td4096179.html"> one of daily digests</a> generated from activity on gerrit and imagine we would still get one mail for each change, update and merge to the mailing list for manual patch tracking as we did in the early days. Thanks a lot also to Mathias Michel for <a href="https://gerrit.libreoffice.org/gitweb?p=dev-tools.git;a=history;f=gerritbot/send-daily-digest;h=79d86b8926ec7be8a5b9624e20e06d5c6275d85e;hb=362e11ede8be6fd4b85eb6ed6bfdaf6b946f43b4">his work on the script</a>!

So if you haven't done that yet, consider graping an <a href="https://wiki.documentfoundation.org/Development/Easy_Hacks">EasyHack</a> and get started!

A copy of the original .odp is also available at <a href="https://fosdem.org/2014/schedule/event/liberated_build_system_mission_accomplished/">FOSDEM</a> or on the<a href="https://wiki.documentfoundation.org/User:Bjoern-michaelsen"> LibreOffice wiki</a>.

<sup>(*)</sup> This includes checking 1.3GB of generated c++ dependency files for some &gt;8000 object files, <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=blob;f=solenv/bin/concat-deps.c;h=a64723f476d77f88c147545dc8844ac47c44dfb2;hb=53c84ea21e4e709f3b40d5f56ce764f0c4ce1fc7">which we simplify to &lt;350MB</a>.

Originally published on 2014-02-09 18:40:50 on [wordpress](https://skyfromme.wordpress.com/2014/02/09/liberated-build-system-mission-accomplished/).

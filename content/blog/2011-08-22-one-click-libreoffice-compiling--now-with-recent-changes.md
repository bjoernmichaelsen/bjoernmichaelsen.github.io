+++
title="One-click LibreOffice compiling, now with recent changes"
date=2011-08-22
[taxonomies]
originally-published-on=["livejournal"]
+++
One-click LibreOffice compiling, now with recent changes
========================================================

<a href="http://wiki.documentfoundation.org/Development/One_Git_Conversion"><br />OneGit</a> is there, which makes a lot of things really nice and nifty, so while we are at it: a big thumbs up to Norbert for that! Hendrik Jensen <a href="http://nabble.documentfoundation.org/PATCH-Series-of-fixes-and-migration-to-One-Git-for-contrib-dev-tools-ubuntu-jenkins-tp3244052p3244052.html">send patches</a> adjusting the Jenkins setup (Thanks!), which should make it even smoother to use. After that I also updated the <a href="http://sweetshark.livejournal.com/1858.html">Jenkins-Ubuntu setup</a> script because we can now use the git-plugin in Jenkins in a sensible way. Among other things (like automatic triggering builds on new commits), your one click compile now has a list of changes since the last build attached to it. So now you can start your day right:<br />
<ul>
 <li>trigger the good-morning build (might be triggered automatically half an hour before you get up)</li>
 <li>fetch a coffee or tea</li>
 <li>have a look at what happened on master since your last build:</li>
</ul>

![jenkins screenshot](/img/lj/2011-08-22-jenkins.png)

And yes, the &quot;detail&quot; link show the commit locally and the &quot;gitweb&quot; links go to <a href="http://cgit.freedesktop.org/libreoffice/core/commit">http://cgit.freedesktop.org/libreoffice/core/commit</a> .

This was originally published at 2011-08-22 13:56:00/2011-08-22 11:56:54 on [livejournal](https://sweetshark.livejournal.com/4298.html).

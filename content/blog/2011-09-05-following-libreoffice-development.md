+++
title="Following Libreoffice development"
date=2011-09-05
[taxonomies]
originally-published-on=["livejournal"]
+++
Following Libreoffice development
=================================

<div style="text-align: right;"> <i>Talk to me, so you can see<br />Oh, what&#39;s going on<br />What&#39;s going on<br /> <span style="font-size:smaller;"><i>-- Marvin Gaye, What&#39;s Going On</i></span></i></div><i>On the <a href="http://wiki.documentfoundation.org/Hackfest2011">exciting and successful Hackfest 2011</a> one question that came up was how even non-developers can easily follow what is cooking in development. Luckily, with the migration to OneGit that got a lot easier:</i><br /><a href="http://cgit.freedesktop.org/libreoffice/core/log/">http://cgit.freedesktop.org/libreoffice/core/log/</a><br />shows what is currently being worked on and changed in master. To follow the development locally (and even offline), one can:<br />do (after installing git) on the commandline:<br />&nbsp;&nbsp;&nbsp; <b>git clone git://anongit.freedesktop.org/libreoffice/core &amp;&amp; cd core</b><br />to get the repository and then:<br /><b>&nbsp;&nbsp;&nbsp; git log</b><br />to see what changes recently happened in the project. If one commit is of particular interest a:<br /><b>&nbsp;&nbsp;&nbsp; git show <i>&lt;commit-id&gt;</i></b><br />shows details about that commit. Later, one can update the repository with<br /><b>&nbsp;&nbsp;&nbsp; git pull</b><br />to download the latest changes. For <b>pull</b> and <b>clone</b> one has to be online, but not for <b>log</b> and <b>show</b>.<br />In addition, there are the tools <a href="https://launchpad.net/ubuntu/+source/gitg">gitg</a> and <a href="http://www.kernel.org/pub/software/scm/git/docs/gitk.html">gitk</a> with make all this available with a nice browsable graphical user interface.

This was originally published at 2011-09-05 20:52:00/2011-09-05 18:52:41 on [livejournal](https://sweetshark.livejournal.com/4478.html).

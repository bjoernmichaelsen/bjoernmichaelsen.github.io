+++
title="debugging (Junit) subsequenttests made easy!"
date=2011-06-08
[taxonomies]
originally-published-on=["livejournal"]
+++
debugging (Junit) subsequenttests made easy!
============================================

The Junit-based subsequenttests for LibreOffice have up to now been quite annoying to debug. <br />Here is a quick reinteration on how to use the subsequenttests. To run the subsequenttests one would:<br /><div style="margin-left: 40px;">./configure ....<br />make dev-install<br />make check</div><br />or, to run just the checks in one module:<br /><div style="margin-left: 40px;">source LinuxX86-64Env.Set.sh<br />make -srf sw/Makefile subsequentcheck # or alternatively<br />cd sw &amp;&amp; make -sr subsequentcheck</div><br />To really see the test execute one would set gb_JunitTest_HEADLESS explicitly to empty:<br /><div style="margin-left: 40px;">export gb_JunitTest_HEADLESS=</div><div>&nbsp;</div><br />Now, and this is new: To make debugging easier, there is another variable to control the behavior of the build system when running unittests:<br /><div style="margin-left: 40px;">export gb_JunitTest_DEBUGCOMMAND=&quot;/bin/sh `readlink -f gnomedebug.sh`&quot;</div><br />Now, whenever a office instance is started, the debugcommand is executed afterwards (and before the tests start). <a href="http://cgit.freedesktop.org/libreoffice/contrib/dev-tools/plain/junitdebug/gnomedebug.sh">gnomedebug.sh</a> is a small shell script that starts gdb in a gnome-terminal and attaches to the soffice process. So with this, when running the tests, you get a gdb watching over the office instance that gets to execute the commands it gets from the test via UNO, making it much easier to debug crashers and other weird behavior that might be exposed by the tests. If you want to use another script for your (maybe non-gnome) environment, feel free to do so! And feel free to contribute your script to <a href="http://cgit.freedesktop.org/libreoffice/contrib/dev-tools/tree/junitdebug">cgit.freedesktop.org/libreoffice/contrib/dev-tools/tree/junitdebug</a>.

This was originally published at 2011-06-08 13:24:00/2011-06-08 11:29:42 on [livejournal](https://sweetshark.livejournal.com/2271.html).

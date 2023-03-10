+++
title="gbuild: Eyecandy for developers"
date=2011-07-26
[taxonomies]
originally-published-on=["livejournal"]
+++
gbuild: Eyecandy for developers
===============================

> <p><i>This is the fourth post in a short series of blog posts about the new GNU  make based build system that will soon be integrated into the DEV300  codeline. It is covering getting nice output from gbuild.</i></p>

<p>Welcome back to the&nbsp;little blog series about the new GNU make build system. After talking about <a href="http://planets.sun.com/GullFOSS/entry/gbuild_how_to_setup_a">the dry topic of repositories</a>,  this post is a just a short post about the output of the new build  system. It tries to keep the output calm and clean by default. When you  start a build with:</p>

> <p><font face="courier new,courier,monospace">make -srj9</font></p>

<p>you will get an kbuild-like output:</p>

> <font face="courier new,courier,monospace">[ build CXX ] tools/source/misc/pathutils</font>
>
> <font face="courier new,courier,monospace">[ build LOG ] tools</font>
>
> <font face="courier new,courier,monospace">[ build LNK ] Library/libtllx.so</font>
>
> <font face="courier new,courier,monospace">...</font>
>
> <font face="courier new,courier,monospace">[ build MOD ] tools</font>
>
> <font face="courier new,courier,monospace">[ build ALL ] top level modules: tools</font>
>
> <font face="courier new,courier,monospace">[ build ALL ] loaded modules: tools</font>

<p>A <font face="courier new,courier,monospace">make clean</font> command result in the same clean output but with &quot;clean&quot; instead of &quot;build&quot;. When setting some variables:</p>

> <p><font face="courier new,courier,monospace">export gb_TITLES=T gb_COLOR=T</font></p>

The output gets a little more attractive:
![screenshot of terminals](/img/lj/2011-07-26-eyecandy.png)

The left column shows a <font face="courier new,courier,monospace">make clean</font>, the middle column a <font face="courier new,courier,monospace">make/make all</font> (top: with color, bottom: without color). The <font face="courier new,courier,monospace">__.oO</font> and <font face="courier new,courier,monospace">Xx.__</font>  ASCII art represent my best attempt at symbolizing a building/cleaning  target. If you have a better idea, give me a note. Please note that the  colored output will also help when using a verbose build as it will  stick out between all the other output and allows easier orientation in  the output. The <font face="courier new,courier,monospace">gb_TITLES=T</font> enables that the progress is also shown in the terminal title. Screenshots are not very good at conveying that, unfortunately.    <p>The two terminal windows on the right show some of the verbose error messages that the gbuild  system issues when it deems something wrong. Please note that these  errors are reported early (before starting to really build anything) and  not late (when trying to actually compile/link something that does not  exist).</p>    <p>Here are a few conditions, that gbuild will try to detect and complain about:</p>    <ul><li>initial makefile outside of the source repositories</li><li>no call to <font face="courier new,courier,monospace">gb_Helper_register_repository</font> in the <font face="courier new,courier,monospace">Repository.mk</font></li><li>adding an executable/library to an invalid group in <font face="courier new,courier,monospace">gb_Helper_register_*</font> (The error message will report the valid groups.)</li><li>corrupted module stacks</li><li>adding a object to a library which has no C/C++ source file in any of the repositories</li><li>generating a component file which has no source file in any of the repositories</li><li>generating resource for which there is no source file in any of the repositories</li><li>linking against a library that was not registered in <font face="courier new,courier,monospace">Repository.mk</font></li><li>defining a library that was not registered in <font face="courier new,courier,monospace">Repository.mk</font></li><li>unknown platform</li></ul>That is it for this post. The next one will be about issuing  build commands and how the commands in the new build system compare to  those in the old build.pl/dmake combination.<br /><br />(This is a very raw mirror of the original blog post made to blogs.sun.com  on 21 Dec 2010. As per <a href="http://web.archive.org/web/20090627144253/http://www.sun.com/termsofuse.jsp" rel="nofollow">http://web.archive.org/web/2009062714425<wbr></wbr>3/http://www.sun.com/termsofuse.jsp </a>   &quot;... You grant Sun and all other users of the Website an irrevocable,   worldwide, royalty-free, nonexclusive license to use, reproduce,  modify,  distribute, transmit, display, perform, adapt, resell and  publish such  Content (including in digital form) ...&quot; )

This was originally published at 2011-07-26 10:24:00/2011-07-26 08:24:55 on [livejournal](https://sweetshark.livejournal.com/2865.html).

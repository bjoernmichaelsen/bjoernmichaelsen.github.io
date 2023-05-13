+++
title="Getting Started With LibreOffice Development: Object-oriented Programming and C++ Introductions"
date=2018-10-11
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "Uncategorized"]
+++
Getting Started With LibreOffice Development: Object-oriented Programming and C++ Introductions
===============================================================================================

<p style="text-align:right;"><em>Turned around and found the right line</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=Sh5S3OxiE-s"><em>— No Leaf Clover, Metallica</em></a></p>
So, for getting started with LibreOffice development e.g. with an <a href="https://wiki.documentfoundation.org/Development/EasyHacks">EasyHack</a> four things are needed:
<ul>
 	<li>Understanding of Object Oriented Programming</li>
 	<li>C++ Language Fundamentals</li>
 	<li>Completing a Build from Scratch of LibreOffice master</li>
 	<li>Understanding git and gerrit to submit your changes</li>
</ul>
We traditionally have covered the latter two quite well in our Wiki:
<ul>
 	<li>e.g <a href="https://wiki.documentfoundation.org/Development/BuildingOnLinux">Building on Linux (video included)</a>, <a href="https://wiki.documentfoundation.org/Development/BuildingOnWindows">Building on Windows</a> and <a href="https://wiki.documentfoundation.org/Development">more ...</a></li>
 	<li>there exists <a href="https://wiki.documentfoundation.org/Development/Git_For_LibreOffice_Developers">extensive documentation and tutorials on git</a> and also for <a href="https://wiki.documentfoundation.org/Development/gerrit">gerrit we collected some good documentation</a>, because -- while git is a common skill these days -- gerrit still isnt that commonly used in open source projects.</li>
</ul>
However, we did not have hints on where to find good documentation on object oriented programming in general and modern C++ programming. At most universities these days programming is taught with Java, JavaScript and Python, which leaves a missing piece on getting started with C++ programming. So <a href="https://erack.de/">Eike</a> and me started looking for good resources on these topics too. Here is what we added to the <a href="https://wiki.documentfoundation.org/Development/GeneralProgrammingGuidelines">General Programming Guidelines</a>:
<ul>
 	<li style="list-style-type:none;">
<ul>
 	<li>For Object-Oriented Programming
<ul>
 	<li><a href="https://www.bookzilla.de/shop/article/5270394/brett_mclaughlin_gary_pollice_david_west_head_first_object_oriented_analysis_and_design.html">Head First: Object Oriented Analysis and Design</a></li>
 	<li><a href="https://www.bookzilla.de/shop/article/3267951/eric_freeman_elisabeth_freeman_bert_bates_kathy_sierra_head_first_design_patterns.html">Head First: Design Patterns</a></li>
</ul>
</li>
 	<li>For C++ Introductions:
<ul>
 	<li>for online reading: <a href="http://www.icce.rug.nl/documents/cplusplus/">C++ Annotations</a></li>
 	<li>as a course: <a href="https://web.stanford.edu/class/cs106b/index.shtml">Stanford University CS106b: Programming Abstractions</a>: <a href="https://www.youtube.com/playlist?list=PLnfg8b9vdpLn9exZweTJx44CII1bYczuk">winter 2018 lectures playlist</a>, <a href="https://www.youtube.com/playlist?list=PLnfg8b9vdpLn9exZweTJx44CII1bYczuk">2016 lectures playlist</a></li>
</ul>
</li>
</ul>
</li>
</ul>
The OOP books are using Java as reference language, but they do not get lost in language details and intentionally allow using the concepts on other languages like C++. The university lecture starts off expecting basic knowledge of programming (e.g. in Java/JavaScript) so both together should yield a reasonable coverage of what is needed for your first <a href="https://wiki.documentfoundation.org/Development/EasyHacks">EasyHacks</a>.

Finding a good and modern starting point for C++ development was by far the hardest topic to cover of those named above. However, looking for them prompted Eike to dig out his personal <a href="https://erack.de/bookmarks/D.html">"developer bookmarks"</a> -- a treasure trove that I will keep exploring further for other good content (this is where the "C++ Annotations" link came from).

&nbsp;

P.S. As a sidenote and additional motivation: All those books -- but especially "Head First: Design Patterns" should provide the context to understand many of the in-jokes found on the oldest wiki ever, the <a href="http://wiki.c2.com/?WelcomeVisitors">C2 WikiWikiWeb</a> which can be an quite entertaining read once in a while.
Originally published on 2018-10-11 14:55:55 at https://skyfromme.wordpress.com/2018/10/11/getting-started-with-libreoffice-development-object-oriented-programming-and-c-introductions/.

+++
title="I would kill 500 lines and I would kill 500 more ..."
date=2015-06-26
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "c++11", "libreoffice", "ubuntu"]
+++
I would kill 500 lines and I would kill 500 more ...
====================================================

<p style="text-align:right;"><em>I would walk 500 miles and I would walk 500 more</em>
<em>-- <a href="https://www.youtube.com/watch?v=tM0sTNtWDiI">The proclaimers, 500 miles</a></em></p>
<p style="text-align:left;">So <a href="https://twitter.com/Sweet5hark/status/613846026118668288">I recently noted</a> that github reported I have 1337 commits on LibreOffice since I joined Canonical in February 2011. Looking at those stats, it seems I also deleted some net 155,634 lines over that time in the codebase.</p>
<p style="text-align:left;"><a href="https://skyfromme.files.wordpress.com/2015/06/leetcommits.png"><img class="aligncenter size-large wp-image-1078" src="https://skyfromme.files.wordpress.com/2015/06/leetcommits.png?w=519" alt="LibreOffice commits" width="519" height="213" /></a></p>
<p style="text-align:left;">Even though I cant find that mail, I seem to remember that Michael Stahl, when joining the LibreOffice project proclaimed his goal to be to contribute 'a net negative lines of code.'<sup>1)</sup> Now I have not looked into the details of the above stats -- they might very likely reveal to be caused by some bulk change. Which would be lame, unless its the <a href="https://skyfromme.wordpress.com/2013/02/28/one/">killing of the old build system</a>, for which I think I can claim some credit. But in general I really love the idea of 'contributing a net negative number of lines of code'.</p>
<p style="text-align:left;">So, at the last <a href="https://wiki.documentfoundation.org/Hackfest/Cambridge2015">LibreOffice Hackfest in Cambridge <sup>2)</sup></a>, I pushed a set of commits refactoring the UNO bindings of writer tables. It all started so innocent. I was actually aiming to do something completely different: namely give the UNO cursors in Writer (<code>SwUnoCrsr</code>) somewhat saner resource management and drag them screaming and kicking out of the 1980ies. However, once in <a href="https://github.com/LibreOffice/core/blob/master/sw/source/core/unocore/unotbl.cxx">unotbl.cxx</a>, I found more of <a href="http://www.ee.ryerson.ca/~elf/hack/realmen.html">"determined Real Programmer can write FORTRAN programs in any language"</a> and copypasta there than I could bear. I thought: "This UNO stuff has <em>decent test coverage</em>, you could refactor it <em>a bit</em> <em>quickly</em>.".</p>
Of course I was wrong with both sides of that statement: On the one hand, when I started the coverage was <a href="http://dev-builds.libreoffice.org/lcov_reports/master~2014-11-02_22.37.32/sw/source/core/unocore/unotbl.cxx.gcov.html">70.1% LOC</a> on that file which is not really as high as I expected. On the other hand, I did not end with "a bit quickly", rather I went on to refactor away:<code>
dc -e "`git log --author Michaelsen -p dc8697e554417d31501a0d90d731403ede223370^..HEAD sw/source/core/unocore/unotbl.cxx|grep ^+|wc -l` `git log --author Michaelsen -p dc8697e554417d31501a0d90d731403ede223370^..HEAD sw/source/core/unocore/unotbl.cxx|grep ^-|wc -l` - p"
-1015
</code>
<p style="text-align:left;">... a thousand lines. On discovering the lacking test-coverage, I quickly added some more tests -- bringing coverage to 77.52% LOC at least now.<sup>3)</sup> And yes, I also silently fixed the <a href="https://github.com/LibreOffice/core/commit/684d2ad37aed1240eea03dac381acd1c73383b20">one regression</a> I thereby discovered I had introduced, which nobody seemed to have noticed so far. One thing I noticed in this little refactoring spree is that while C++11s features might look tame compared to more modern programming languages in metrics like avoiding boilerplate, it still outclasses what we had before. Beyond the simplifying refactoring, features like lambdas are really nice for<a href="http://nabble.documentfoundation.org/OSX-uiwriter-stacktrace-needed-tp4151007p4151083.html"> non-interactive (test-driven) debugging,</a> including quickly asserting on the state of variables some over some 10 stackframes up or down without going into major contortions in testcode.</p>
<sup>1)</sup> By the way, a quick:
<code>dc -e "`git log --author Stahl -p |grep ^+|wc -l` `git log --author Stahl -p |grep ^-|wc -l` - p"
-108686</code>

confirms Michael is more than living up to his personal goals.

<sup>2) </sup>Speaking of the Hackfest: The other thing I did there was helping/observing Sam Tuke getting setup for his first code contribution. While we made great progress in making this easier than it used to be, we could be a lot better there still. Sadly though, I didnt see a shortcut or simplification we could implement right away.

<sup>3) </sup>And along with that did bring coverage of <a href="http://lcov.libreoffice.org/sw/source/core/unocore/unochart.cxx.gcov.html">unochart.cxx</a> from abismal 4.4% LOC to at least 35.31% LOC  as a collateral damage.

<strong><em>addendum: </em></strong>Note that the writer tables core also increased coverage quite a bit from <a href="http://dev-builds.libreoffice.org/lcov_reports/master~2014-11-02_22.37.32/sw/source/core/table/index.html">54.6% LOC</a> to <a href="http://lcov.libreoffice.org/sw/source/core/table/index.html">65% LOC</a>.

Originally published on 2015-06-26 12:29:54 on [wordpress](https://skyfromme.wordpress.com/2015/06/26/i-would-kill-500-lines-and-i-would-kill-500-more/).

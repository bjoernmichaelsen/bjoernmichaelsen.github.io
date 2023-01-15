+++
title="What is bibisect? And what is it doing in my office?"
date=2011-12-09
[taxonomies]
originally-published-on=["livejournal"]
+++
What is bibisect? And what is it doing in my office?
====================================================

<div style="text-align: right;"><i>In many ways you can just see git as a filesystem -
it&#39;s content- addressable, and it has a notion of versioning [...]</i>
-- <a href="http://kerneltrap.org/node/4982">Linus Torvalds</a></div>
bibisect stands for &quot;binary bisect&quot; and is intended to help QA for LibreOffice 3.5. Regressions are a most annoying artifact that unfortunately comes with software development and QA. However, regressions are a misfeature we want to deal with quick and early as they might get harder and harder to triage and fix as time passes.

Because the way git stores its stuff, this download:

<ul><li>&nbsp;<a href="http://people.canonical.com/~bjoern/bibisect-3.5.tar.lzma">http://people.canonical.com/~bjoern/bibisect-3.5.tar.lzma</a></li></ul>contains:

<ul><li>53 complete Linux 64-bit office installs (compiled on Ubuntu 11.10, but should work elsewhere too) between the creation of the core repo and the -3-5 branchoff (that is ~5000 commits)</li><li>at 450MB each, that would be ~22GB total</li><li>however, it is only 749MB total download size, that is <b> less than 15MB per installation</b></li></ul>And one does not need to install them in parallel as one can switch through all of them with a quick &quot;git checkout source-hash-XXXXXX&quot; -- one switch costs &lt;1 second).

Now how does one use this for cornering a regression? Well, this is where the power of bisecting comes in. First you download the tarball and unpack it:

<blockquote>&gt; wget http://people.canonical.com/~bjoern/bibisect-3.5.tar.lzma
&gt; tar --lzma -xf bibisect-3.5.tar.lzma
&gt; cd bibisect-3.5
&gt; # to verify your download was not corrupted do a: git tag -v latest
&gt; # you need my gpg-key from <a href="https://launchpad.net/~bjoern-michaelsen">https://launchpad.net/~bjoern-michaelsen</a> for that
</blockquote>
Then you get yourself the newest build included in the download and check if your bug it there:

<blockquote>&gt; git checkout latest
&gt; ./opt/program/soffice.bin
</blockquote>
Then get the oldest build included in the download and check that the regression is not there:

<blockquote>&gt; git checkout oldest
&gt; ./opt/program/soffice.bin
</blockquote>
If the bug is already present at this point, it is not a regression in the range that is covered by the download, but an even older bug.
If the bug is not there it is a regression in the range covered by the download and we can corner it down very well now. Do:

<blockquote>&gt; git bisect start latest oldest
</blockquote>
to start bisecting. Then repeat these commands:

<blockquote>&gt; ./opt/program/soffice.bin
&gt; git bisect good # if the bug is not there
&gt; git bisect bad # if the bug is there
</blockquote>
after some ~5 repetitions, git will tell you something like this:

<blockquote>9625329ea5a7e3e8475cd21c07726beec20573bd is the first bad commit
commit 9625329ea5a7e3e8475cd21c07726beec20573bd
Author: Bjoern Michaelsen &lt;bjoern.michaelsen@canonical.com&gt;
Date:&nbsp;&nbsp; Thu Dec 8 12:29:59 2011 +0100

&nbsp;&nbsp;&nbsp; source-hash-2d19e9bb07ccff3134f855812dddfda5c07b1fe4
&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp; commit 2d19e9bb07ccff3134f855812dddfda5c07b1fe4
&nbsp;&nbsp;&nbsp; Author:&nbsp;&nbsp;&nbsp;&nbsp; Jan Holesovsky &lt;kendy@suse.cz&gt;
&nbsp;&nbsp;&nbsp; AuthorDate: Wed Nov 16 14:17:03 2011 +0100
&nbsp;&nbsp;&nbsp; Commit:&nbsp;&nbsp;&nbsp;&nbsp; Jan Holesovsky &lt;kendy@suse.cz&gt;
&nbsp;&nbsp;&nbsp; CommitDate: Wed Nov 16 14:21:33 2011 +0100
&nbsp;&nbsp;&nbsp;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; Kill one usage of chrel.sed to fix build.
</blockquote>
Append that (the source-hash-2d19e9bb07ccff3134f855812dddfda5c07b1fe4 line is the important one) and the output of:

<blockquote>&gt; git bisect log
</blockquote>
Which should look something like this:

<blockquote>git bisect start &#39;latest&#39; &#39;oldest&#39;
# good: [2faf4bc12ab490370d2196dedbc8091f9b09d0a5] source-hash-418a35f4861e863feb39eec73f4a39a87fbcb1f3
git bisect good 2faf4bc12ab490370d2196dedbc8091f9b09d0a5
# bad: [b6fca7e58854bc617c5fc9a75d1c1720b0d7e1a4] source-hash-ce60138d339a5eb2a174a5d27063249acf2cac42
git bisect bad b6fca7e58854bc617c5fc9a75d1c1720b0d7e1a4
# good: [0a28a62d53e996cf66d86e9bfb63ddc6ade75b7e] source-hash-71cbcb62028295a98ceee60cb4c4ee425bafcd2e
git bisect good 0a28a62d53e996cf66d86e9bfb63ddc6ade75b7e
# bad: [9625329ea5a7e3e8475cd21c07726beec20573bd] source-hash-2d19e9bb07ccff3134f855812dddfda5c07b1fe4
git bisect bad 9625329ea5a7e3e8475cd21c07726beec20573bd
# good: [89d91bb6074026dc0894bcdc6aaf8f6124102da7] source-hash-fb754a0df859e30255c25af8fa19bfaa75f257e7
git bisect good 89d91bb6074026dc0894bcdc6aaf8f6124102da7
</blockquote>
to the bug and the developers will have a very good idea where your bug is -- in this case between the commits fb754a0df859e30255c25af8fa19bfaa75f257e7 (good) and 2d19e9bb07ccff3134f855812dddfda5c07b1fe4 (bad) on master. A:

<blockquote>&gt; git log fb754a0df859e30255c25af8fa19bfaa75f257e7..2d19e9bb07ccff3134f855812dddfda5c07b1fe4
</blockquote>
on the source repository will then show the 128 commits including the one that introduced the bug, making it a lot easier for the developer to close in on the culprit.
For more details, see:<ul><li><a href="http://book.git-scm.com/5_finding_issues_-_git_bisect.html">bisect in the git community book </a></li><li><a href="http://nabble.documentfoundation.org/What-is-bibisect-And-what-is-it-doing-in-my-office-td3572953.html">discussion on the developer list</a></li></ul>

This was originally published at 2011-12-09 20:09:00/2011-12-09 19:09:52 on [livejournal](https://sweetshark.livejournal.com/7683.html).

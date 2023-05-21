+++
title="Death or Glory vs. Continuous Integration"
date=2015-05-26
[taxonomies]
originally-published-on=["wordpress"]
categories=["build system", "ci", "continuous intregration", "gerrit", "git", "jenkins", "libreoffice", "ubuntu"]
+++
Death or Glory vs. Continuous Integration
=========================================

<p style="text-align:right;"><em>But I believe in this and it's been tested by research</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=MwzMuuGOsVI"><em> -- The Clash, Death and Glory</em></a></p>

<p style="text-align:left;">Thanks to Norbert's efforts, the LibreOffice project now has a Jenkins setup that not only gives us visibility on how healthy our master branch is, with the results being reported to the ESC regularly: In addition it allows everyone easily testing commits and branches on all major LibreOffice platforms (Linux, OS X, Windows) just by uploading a change to <a href="https://gerrit.libreoffice.org/">gerrit</a>. Doing so is really easy once you are <a href="https://wiki.documentfoundation.org/Gerrit#Setting_Yourself_Up_For_Gerrit">set up</a>:</p>

<pre>./logerrit submit                      # a little helper script in our repo
git push logerrit HEAD:refs/for/master # alternative: plain old git
git review                             # alternative: needs to install the git-review addon</pre>
Each of the above commands alone send your work for review and testbuilding to gerrit. The last one needs an <a href="https://wiki.documentfoundation.org/Development/GitReview">additional setup</a>, that is however really helpful and worth it for people working with gerrit from the command-line regulary.

So, what if you have a branch that you want to testbuild? Well, just pushing the branch to gerrit as suggested above still works: gerrit then will create a change for every commit, mark them as depending on each other and testbuild every commit. This is great for a small branch of a handful of commits, but will be annoying and somewhat wasteful for a branch with more than 10-15 commits. In the latter case you might not want a manual review for each commit and also not occupy our builders for each of them. So what's the alternative, if you have a branch <code>${mybranch}</code> and want to at least test the final commit to build fine everywhere?
<pre>git checkout -b ${mybranch}-ci ${mybranch} # switch to branch ${mybranch}-ci
git rebase -i remotes/logerrit/master      # rebase the branch on master interactively</pre>
Now your favourite editor comes up showing the commits of the branch. As your favourite editor will be vim, you can then type:
<pre>:2,$s/^pick/s/ | x</pre>
To squash all the commits of the branch into one commit. Then do:
<pre>git checkout -                                   # go back to whatever branch we where on before
git push logerrit ${mybranch}-ci:refs/for/master # push squashed branch to gerrit for testbuilding
git branch -D ${mybranch}-ci                     # optional: delete squashed branch locally</pre>
Now only wait for the builder on Jenkins to report back. This allowed me to find out that our compiler on OS X didnt think of this <a href="https://gerrit.libreoffice.org/#/c/15892/1/sw/inc/unocrsr.hxx">new struct</a> as a POD-type, while our compilers on Linux and Windows where fine with it (see: <a class="question-hyperlink" href="http://stackoverflow.com/questions/7411515/why-does-c-require-a-user-provided-default-constructor-to-default-construct-a">"Why does C++ require a user-provided default constructor to default-construct a const object?"</a> for the gory details). Testbuilding on gerrit allowed me to fix this before pushing something broken on a platform to master, which would have spoiled the nifty ability to test your commit before pushing for everyone else: Duly testing your commit on gerrit only to find that the master you build upon was broken by someone else on some platform is not fun.

The above allows you to ensure the end of your branch builds fine on all platforms. But what about the intermediate commits and our test-suites? Well, you can test that each and every commit passes tests quite easily locally:
<pre>git rebase -i remotes/logerrit/master --exec 'make check'</pre>
This rebases your branch on master (even if its already up to date) and builds and runs all the tests on each commit along the way. In case there is a test breakage, git stops and lets you fix things (just like with traditional troubles on rebases like changes not applying cleanly).

Note: gerrit will close the squashed branch change if you push the branch to master: The squashed commit message ends with the Change-Id of the final commit of the branch. So once that commit is pushed, the gerrit closes the review for the squashed change.

Another note: If the above git commands are too verbose for you (they are for me), consider using <a href="https://github.com/thoughtbot/gitsh">gitsh</a> and <a href="http://githowto.com/aliases">aliases</a>. Combined they help quite a lot in reducing redundant typing when working with git.

Originally published on 2015-05-26 08:39:20 on [wordpress](https://skyfromme.wordpress.com/2015/05/26/death-or-glory-vs-continuous-integration/).

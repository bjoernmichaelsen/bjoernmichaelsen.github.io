+++
title="tb3 -- more efficient tinderboxing"
date=2013-08-03
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
tb3 -- more efficient tinderboxing
==================================

<p style="text-align:right;"><em>Stop right there I gotta know right now before we go any further</em></p>
<p style="text-align:right;"><em>...</em></p>
<p style="text-align:right;"><em>Let me sleep on it and I'll give you an answer in the morning</em></p>
<p style="text-align:right;"><a href="http://www.youtube.com/watch?v=ex4s4jXLZkg"><em>-- Paradise by the Dashboard Light, Bat out of Hell, Meat Loaf</em></a></p>
<p style="text-align:left;">So, I did some work recently to possibly make our tinderboxes more efficient and scalable -- which is a bit ironic as I recently hinted others at Paul Grahams advise to <a href="http://paulgraham.com/ds.html">"do things that do not scale"</a>. At LibreOffice we currently have <a href="http://tinderbox.libreoffice.org/MASTER/status.html">tinderbox setup</a> that served us as good as it could in the first years: It gave a quick overview of the basic health of current development branch of LibreOffice. But LibreOffice takes some time to build and test and with 50-100 commits to master each day it is playing catch-up with a moving target.</p>
<p style="text-align:left;">And whle they did a good job at this, they also have a few distinct weaknesses: For one, these tinderboxes would also mail everyone who commited on a branch since the last known good build if they were unhappy. Since they do not know anything about each other, with a generic breaker each tinderbox would do that on its own. In a tragic imitation of a <a href="http://www.fun2gag.com/tent/uploads/2013/01/Who-are-BROWSERS.jpg">certain comic</a> this would result in the incremental Linux tinderbox reporting after 5 minutes something went wrong, with all the other tinderboxes dribbling in with the same message over time, finalized by the full Windows build tinderbox excitedly reporting to 200 people (as a slow builder would have more commits between builds) that something was amiss -- possibly hours after it was fixed again. This resulted in these messages being filtered away by most users and even worse: the Windows tinderbox reports, which should be the most useful of them, as most developers use Linux as development platform, being easily ignored as "someone else broke it".</p>
<p style="text-align:left;">So I set out improve the situation with the initial goal:</p>

<ul>
	<li>to start make tinderboxes being able to coordinate</li>
	<li>to make it possible to easily collate the information from multiple builders</li>
	<li>while leaving the control over what is build with the owner of the tinderbox (as most of these boxes are sponsored, we dont want to make them into drones)</li>
	<li>for slow platforms like Windows or ARM enable bisecting a breaker as the frequency of builds is too low for those in the commit range to feel personally responsible</li>
	<li>while bisecting a breaker, also keep an eye one the branch moving forward (as in: dont try to bisect a breaker further when it was fixed in the meantime)</li>
</ul>
And I am happy to report to have reached this initial goal with <a href="https://gerrit.libreoffice.org/gitweb?p=buildbot.git;a=tree;f=tb3;h=91d26ee487eb716f4e6afbcf14b16ac4f69da411;hb=e17b17c3a859edbf917f62049abb260991cba5dc">tb3</a> which is a tinderbox coordinator written in Python3 and having as many lines of codes for unittests as for the product itself. So how is tb3 intended to work?
<h3>Leaving control over what is build with the owner of the tinderbox</h3>
tb3 is build around the idea, that the information about the state of the source is collected and managed by a central "tinderbox coordinator" and one or more tinderboxes go to it to:
<ul>
	<li>ask for something to build, giving the coordinator a branch and a platform that they are interested to work for</li>
	<li>report that they have started to build a certain state and give an estimate on when they will be finished</li>
	<li>report that they have finished to build a certain state and give a result</li>
</ul>
Note that the first two steps are separate: The tinderbox is essentially just asking for a suggestion on what to build -- its not promising to actually follow these proposals. It can come back and report to be building something completely different(*). Now the proposals the coordinator hands out come with a score. Just looking at a classical tinderbox mode, which will always build the current HEAD of a branch on a specific platform, the score of the highest ranking proposal will be equal to the number of commits since the last finished build. With tb3, a tinderbox can watch multiple branches (e.g. a development branch and a release branch) and commit itself to building the one which saw the most commits since the last finished. It can also use multipliers and use something like "if there are 10 times as many new commits on the development branch as on the release branch, then build that, otherwise stick to the release branch" or use limits: "I only want run a build if there are at least 5 new commits".
<h3>Coordinating multiple tinderboxes</h3>
<p style="text-align:left;">So how do we coordinate multiple tinderboxes and ensure that e.g. if someone pushes 9 commits to master, we do not get five Linux tinderboxes to build that last commit and then sprinkle everyones mailbox over the next hour? Here is where the "coordinator" part truly kicks in. The first tinderbox that asks for something to build will get proposals with scores as shown by the green line in the chart below: The highest score is the "9" of the newest commit -- the commit that has the biggest distance from the last build. If the first tinderbox reported to have taken on that proposed build, what would a second tinderbox that also asks to build something see? It makes little sense to give it the same build as the first tinderbox. Optimistically assuming that tinderbox will report something back, the best thing this second box can do is build something with the biggest distance to to the finished build and to the build running on the first tinderbox. As such, the coordinator will send it scored as denoted by the blue line and if the tinderbox accepts it will build commit 5 -- which is why a third tinderbox asking for something to build, while the other two are running, will get proposals as per the pink line and thus be suggested to build commit 3.</p>


[caption id="attachment_609" align="aligncenter" width="519"]<a href="/static/img/wp/2013/08/tb3started.png"><img class="size-large wp-image-609" alt="proposal scores with tinderboxes just started" src="/static/img/wp/2013/08/tb3started.png?w=519" width="519" height="344" /></a> proposal scores with tinderboxes just started[/caption]
<h3 style="text-align:left;">Trusting tinderboxes ... a bit</h3>
Now these tinderboxes "promised" to build some commit. But can we give the tinderbox unconstrained trust? E.g. should we never ever tell any other tinderbox to build this one commit, because some other tinderbox promised to build it? The answer is obviously no: As a tinderbox is a gift, the owner should be allowed to reboot or reassign a tinderbox for other tasks at any time with imprudence. This is why the tinderbox gives the coordinator an estimated duration for its build and the tinderbox coordinator "reserves" this commit for that time. As you did see in the last chart the commit that just had a tinderbox running got scores of zero. As time goes by the coodinator looses trust in the tinderbox to still report back: the chart below shows the scores given after twice the time the tinderbox gave as an estimate has passed. You see the blue line now scores highest at commit 6, not commit 5 and the pink line scores highest at commit 5, not commit 3 -- so as the coordinator looses trust in the running tinderboxes to come back, it again proposes to do builds closer to the already scheduled ones.

[caption id="attachment_610" align="aligncenter" width="519"]<a href="/static/img/wp/2013/08/tb3overdue.png"><img class="size-large wp-image-610" alt="proposed scores with tinderbox results overdue" src="/static/img/wp/2013/08/tb3overdue.png?w=519" width="519" height="349" /></a> proposed scores with tinderbox results overdue[/caption]

Another thing to note is that the highest score is rising: While in the first chart, each running tinderbox lowered the highest score by one (green line: highest at 9, blue line: highest at 8, pink line: highest at 7) after twice the time has passed, the highscores are all around 9 again.
<h3>Bisecting a breaker</h3>
Should a branch be broken, it usually would be very helpful if the tinderboxes would help bisecting. This is especially true for slow platforms and builds like Windows, ARM or the <a href="http://mmohrhard.wordpress.com/2013/04/19/automated-import-crash-testing-in-libreoffice/">document load torturer by Markus</a>. However, we do not want the tinderbox to over fixate on that, as our branch is a moving target. If there is a build breaker somewhere in a range of 256 commits, we do not want a slow tinderbox to bust away for 8 builds to find the offending one, and while doing that leave the head of the branch unwatched for a long time. So by default, the bisecting proposals have a highscore that is equal to the number of commits to bisect still. As such, by default, a tinderbox will be told to bisect -- as long as:
<ul>
	<li>the head of the branch is still broken</li>
	<li>there are more commits in the bisect range, than there are new commit on the branch.</li>
</ul>
[caption id="attachment_611" align="aligncenter" width="519"]<a href="/static/img/wp/2013/08/tb3bisect.png"><img class="size-large wp-image-611" alt="scores of commits in a range to bisect" src="/static/img/wp/2013/08/tb3bisect.png?w=519" width="519" height="361" /></a> proposal scores of commits in a range to bisect[/caption]

Otherwise, the tinderbox will be told to build the latest commit, to check if the branch is still broken or fixed in the meantime. As such the coordinator will guard against commiting tinderboxes to bisect a breaker that was already fixed. Therefore the coordinator knows a few more states than plain 'good' or 'bad' for a commit:
<ul>
	<li>UNKNOWN -- nothing known yet</li>
	<li>RUNNING -- a tinderbox is currently claiming to run this commit</li>
	<li>GOOD -- a tinderbox was happy with it</li>
	<li>BAD -- a tinderbox was unhappy with it</li>
	<li>ASSUMED_GOOD -- not tested, but the previous and the next finished build were good</li>
	<li>ASSUMED_BAD -- not tested, but the previous and the next finished build were bad</li>
	<li>POSSIBLY_BREAKING -- not tested, but the previous finished build was good and the next finished build was bad</li>
	<li>POSSIBLY_FIXING -- not tested, but the previous finished build was bad and the next finished build was good</li>
	<li>BREAKING -- this one was bad, while the previous commit was good</li>
</ul>
Here is some example output
<pre>$ ./tb3-show-history --repo ~/checkouts/core.git --platform linux --branch 65134fb75c3e94b7869fb6d490f88bf4b252760e --history-count 10
65134fb75c3e94b7869fb6d490f88bf4b252760e started on 2013-07-25 17:27:30.383767 with builder ubuntu-tinderbox and finished on 2013-07-25 17:40:41.226494 -- artifacts at 65134fb75c3e94b7869fb6d490f88bf4b252760e-137476605045.out, state: BAD (took 0:13:10.842727)
6100d94078d37cb1413a0e45460cee480ba3e211 started on None with builder None and finished on None -- artifacts at None, state: ASSUMED_BAD
24d46ea66485ff8b5bca49ec587b41547787bf42 started on None with builder None and finished on None -- artifacts at None, state: ASSUMED_BAD
d041980a7aad0e6d111752ca98db42f9853a3c6b started on 2013-07-25 17:40:52.587150 with builder ubuntu-tinderbox and finished on 2013-07-25 17:53:04.204549 -- artifacts at d041980a7aad0e6d111752ca98db42f9853a3c6b-137476685269.out, state: BAD (took 0:12:11.617399)
3b28ec6855e5df0629427752d7dafae1f0a277d4 started on None with builder None and finished on None -- artifacts at None, state: ASSUMED_BAD
cca0b9ae02603ab88ec7d8810aab2a8a1b4efda2 started on 2013-07-25 18:08:01.201013 with builder ubuntu-tinderbox and finished on 2013-07-25 18:20:39.536451 -- artifacts at cca0b9ae02603ab88ec7d8810aab2a8a1b4efda2-137476848124.out, state: BREAKING (took 0:12:38.335438)
767b02bd7614059dd80d0cd1be306d9b63291f31 started on 2013-07-25 17:53:14.745394 with builder ubuntu-tinderbox and finished on 2013-07-25 18:07:42.527839 -- artifacts at 767b02bd7614059dd80d0cd1be306d9b63291f31-137476759480.out, state: GOOD (took 0:14:27.782445)
c852f83bc4d91de51c61ad4be0edf1b848247eaa started on None with builder None and finished on None -- artifacts at None, state: ASSUMED_GOOD
0d874ee2e452ea67c03a27bf1a7f26d0ffc617dc started on None with builder None and finished on None -- artifacts at None, state: ASSUMED_GOOD
ff14c3b595ebe71153f97ebb8871cf024ea76959 started on 2013-07-25 17:12:58.024727 with builder ubuntu-tinderbox and finished on 2013-07-25 17:27:17.439374 -- artifacts at ff14c3b595ebe71153f97ebb8871cf024ea76959-137476517809.out, state: GOOD (took 0:14:19.414647)</pre>
<h3>Some details and missing bits</h3>
The coordinator stores the results in git notes as<a href="http://www.json.org/"> JSON objects</a>. This has multiple advantages: There is no need for a external database and the state of the notes are under revision control. It also has one disadvantage: Its not exactly quick. However the revision control can help to mitigate that mostly  -- as e.g. a webfrontend can easily ask: "what changed on the state since I last polled you?" and do incremental updates from there.

Which brings me to the missing bits: The stuff that tells the world the state of the repo on a webfrontend, RSS feed, IRC Bots or via email digests. The second missing bit is some kind of privilege separating between the tinderboxes and the coordinator. tb3 is currently churning away on the <a href="http://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/">Sun Ultra 24</a> that I donated to the Document Foundation doing duty as an Ubuntu tinderbox, but coordinator and tinderbox are still running on the same account -- even though as separate processes. As setuid for scripts is messy business, I plan to give tb3 a trivial REST-like interface on a non-public HTTP server. In addition to being able to offload the authentication and authorization problems outside of tb3 to something considering it a solved problem, it also makes integration in webfrontends etc. simple (esp. given that all the data is in JSON already anyway.)

In the long run, the scoring of tb3 also should make it easier for the buildbots that do duty on gerrit to make a call on if they should test build something there or if their help is more needed for tinderbox duty.
<h3>tl;dr</h3>
tb3 can:
<ul>
	<li>coordinate multiple tinderboxes working on the same build scenario or branch</li>
	<li>coordinate one tinderbox working on multiple build scenarios or multiple branches</li>
	<li>make tinderboxes bisect without loosing sight of the head of a branch</li>
	<li>especially help tests and builds that are painfully slow</li>
</ul>
They can also create builds for <a href="https://wiki.documentfoundation.org/QA/HowToBibisect">bibisect</a> along the way, but that is a story for another day.

(*) This is helpful for some test suites like e.g. subsequentcheck. If you do a build as proposed by the coordinator, you can cheaply report back the result of the build <em>only</em>. And since you then can just the subsequentcheck test suite on top of the build of that commit (and <em>only</em> on that commit), you can then report to be running these tests and report the results without ever caring if the coordinator thinks this commit has as high priority for this.

postscriptum: Yeah, I know, I promised to be on vacation now and not harass you with any posts, but this is a scheduled blogpost and as such does not count.

Originally published on 2013-08-03 15:00:18 on [wordpress](https://skyfromme.wordpress.com/2013/08/03/tb3-more-efficient-tinderboxing/).

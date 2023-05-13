+++
title="On the importance of being a bug confirmer"
date=2012-08-16
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
On the importance of being a bug confirmer
==========================================

<p style="text-align:right;"><em>The truth is rarely pure and never simple.</em></p>
<p style="text-align:right;">-- Oscar Wilde, The Importance of Being Earnest</p>
<p style="text-align:left;">Two important things happened in LibreOffice QA this week. The first one has not gone unnoticed by a lot of users: Florian Reisinger closed a lot of old and idle bugs (no activity for more than six month) in state NEEDINFO on our friendly bugzilla. As you can see from <a href="https://bugs.freedesktop.org/report.cgi?x_axis_field=resolution&amp;y_axis_field=bug_status&amp;z_axis_field=&amp;query_format=report-table&amp;short_desc_type=allwordssubstr&amp;short_desc=&amp;product=LibreOffice&amp;bug_status=UNCONFIRMED&amp;bug_status=NEW&amp;bug_status=ASSIGNED&amp;bug_status=REOPENED&amp;bug_status=RESOLVED&amp;bug_status=VERIFIED&amp;bug_status=CLOSED&amp;bug_status=NEEDINFO&amp;bug_status=PLEASETEST&amp;longdesc_type=allwordssubstr&amp;longdesc=&amp;bug_file_loc_type=allwordssubstr&amp;bug_file_loc=&amp;status_whiteboard_type=allwordssubstr&amp;status_whiteboard=&amp;keywords_type=allwords&amp;keywords=&amp;bug_id=&amp;bug_id_type=anyexact&amp;emailtype1=substring&amp;email1=&amp;emailtype2=substring&amp;email2=&amp;emailtype3=substring&amp;email3=&amp;chfieldvalue=&amp;chfieldfrom=&amp;chfieldto=Now&amp;field0-0-0=noop&amp;type0-0-0=noop&amp;value0-0-0=&amp;format=table&amp;action=wrap">this table</a> we have bugs in all kinds of states:</p>

<ul>
	<li>the "user/reporter states": NEEDINFO, UNCONFIRMED</li>
	<li>the "bugwrangler states": NEW, REOPENED</li>
	<li>the "developer states": ASSIGNED, RESOLVED/WORKSFORME</li>
	<li>the "verifier state": RESOLVED/FIXED</li>
	<li>the "janitor states": VERIFIED, CLOSED, RESOLVED with resolutions INVALID, DUPLICATE, WONTFIX, NOTABUG, NOTOURBUG</li>
</ul>
The lifecycle of a bug ideally makes the bug go through the states UNCONFIRMED -&gt; NEW -&gt; ASSIGNED -&gt; RESOLVED/FIXED -&gt; VERIFIED -&gt; CLOSED quickly. If it gets stuck somewhere, its up to the group named in the quotes to help this move on: For a bug in state NEEDINFO, it is expected from the user/reporter to come up with missing information. For a bug in state UNCONFIRMED, it is needed for somebody (best not the reporter, but a different user) to make sure that the bug is valid. This is not too hard, the details are described on the <a href="http://wiki.documentfoundation.org/BugTriage">Bug Triage page on the wiki</a>. LibreOffice is a community project and the best way to get your issues forward is to trade favours: Confirm the bugs of somebody else and let him confirm yours (of course, you should still check if the bugs of the other guy/gal are really valid!).

The bugwranglers then take care to make developers aware of the most important bugs in states NEW, REOPENED according to development capacities. If a bug sticks longer in here, it is usually not the bugwranglers to blame, but the fact that there are just more critical bugs to pipe to the developers at this point in time. However bugwranglers also need to make sure that a bug has all the important information on the bug before it gets to development. For example, when it is a regression: what was the first version that misbehaved, stacktraces, test case, test documents. If there are multiple bugs which are about the same severity to choose from to hint development at and one has all that information while others do not -- QA will tend to go with the ones with most information on them.

[caption id="" align="aligncenter" width="2940"]<img src="http://nextdoornature.files.wordpress.com/2011/07/firefly-by-jessica-lucia-cc.jpg" alt="Firefly (Creative Commons by Jessica Lucia via nextdoornature.org" width="800" height="533" /> The job of the bugwranglers is to find those bugs that stick out (Photo: Jessica Lucia, Creative Commons license)[/caption]

The developers then crunch their heads on trying to solve these issues. And if they need a break and some cheering up, they will look at a WORKSFORME bug <a href="http://nabble.documentfoundation.org/Libreoffice-qa-RESOLVED-status-question-tp3999080p4001470.html">that seems to have been fixed to QA/users, but nobody claimed to have done that intentionally</a>. If they can see that the bug was indeed solved, the bug can move to CLOSED/FIXED.  The latter is not a high priority task at all though.

If a bug is set to RESOLVED/FIXED by a developer, the fix can be verified by pretty much anyone on a <a href="http://dev-builds.libreoffice.org/daily/">daily build</a>: while the developers are usually trustworthy folks, it does not hurt to check. A very good side effect of this is, once the daily build is installed, one can easily torture it a bit and see if the trustworthy folks of the last sentence recently broke something in a horrible way: If so they should get notice ASAP via a new bug report.

The janitor states are mostly dead: We then consider that bug finished. Now some bugs were reported more than a year ago, when the<a href="https://bugs.freedesktop.org/"> freedesktop bugzilla</a> did not yet have a UNCONFIRMED default state -- those were moved from NEW to NEEDINFO as they still needed confirmation. If no other user confirmed your bug since then and there was no activity on the bug for more than half a year, the bug would now have been moved to RESOLVED/INVALID with Florians cleanup. In such cases, find somebody to confirm that what you describe is indeed a bug and ask him to reopen that bug for you. Note you dont need to wear a QA uniform, QA hat and badges to do so, although you might earn them by doing Good Work(tm). The QA team is looking for more hands and you can best find us on the <a href="http://nabble.documentfoundation.org/QA-f3613148.html">QA mailing list</a> or <a href="https://www.libreoffice.org/get-involved/qa-testers/">on IRC and other means</a>. Another good way to get started are the <a href="https://wiki.documentfoundation.org/QA/Easy_Hacks">QA EasyHacks</a>.

Oh, the second important thing happening in QA this week? Its still work in progress and this post was long enough, so I will do a post on its own about it. The buzzword will be: <strong>LibreOffice HardHacks</strong>.
Originally published on 2012-08-16 10:29:37 on [wordpress](https://skyfromme.wordpress.com/2012/08/16/on-the-importance-of-being-a-bug-confirmer/).

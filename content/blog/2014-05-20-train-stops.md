+++
title="Train Stops"
date=2014-05-20
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
Train Stops
===========

<p style="text-align:right;"><em>And the sons of pullman porters and the sons of engineers</em>
<em> Ride their father's magic carpets made of steel</em>
<em> Mothers with their babes asleep are rockin' to the gentle beat</em>
<em> And the rhythm of the rails is all they feel</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=AJMVj04lfyo">-- The City of New Orleans, Willie Nelson interpreting Steve Goodman</a></em></p>

<div style="overflow:hidden;color:#000000;background-color:#ffffff;text-decoration:none;text-align:left;">So, LibreOffice does its releases on a <a href="https://en.wikipedia.org/wiki/Software_release_train">train release schedule</a> and since we recently modified the schedule a bit (<a href="http://nabble.documentfoundation.org/Minutes-of-ESC-call-td4104685.html">by putting out the alpha1 release earlier</a>), I took the opportunity to take a closer look and explain a bit on what we are doing. With this every 6 months of LibreOffice development currently roughly look like this:</div>
<table border="0" cellspacing="0">
<tbody>
<tr>
<td rowspan="2" align="center" valign="middle" height="34">week after x.y.0</td>
<td rowspan="2" align="center" valign="middle">development</td>
<td colspan="2" align="center" valign="middle">release candidates</td>
<td colspan="2" align="center">finalized releases</td>
</tr>
<tr>
<td align="center">fresh</td>
<td align="center">stable</td>
<td align="center">fresh</td>
<td align="center">stable</td>
</tr>
<tr>
<td align="right" height="17">0</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.0</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">1</td>
<td align="center"></td>
<td align="center">x.y.1~rc1</td>
<td align="center">x.(y-1).4~rc1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">2</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">3</td>
<td align="center"></td>
<td align="center">x.y.1~rc2</td>
<td align="center">x.(y-1).4~rc2</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">4</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.1</td>
<td align="center">x.(y-1).4</td>
</tr>
<tr>
<td align="right" height="17">5</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">6</td>
<td align="center"></td>
<td align="center">x.y.2~rc1</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">7</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">8</td>
<td align="center"></td>
<td align="center">x.y.2~rc2</td>
<td align="center">x.(y-1).5~rc1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">9</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.2</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">10</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).5~rc2</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">11</td>
<td align="center"></td>
<td align="center">x.y.3~rc1</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).5</td>
</tr>
<tr>
<td align="right" height="17">12</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">13</td>
<td align="center">x.(y+1)~alpha1</td>
<td align="center">x.y.3~rc2</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">14</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.3</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">15</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">16</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">17</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">18</td>
<td align="center">x.(y+1)~beta1</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">19</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">20</td>
<td align="center">x.(y+1)~beta2</td>
<td align="center"></td>
<td align="center">x.(y-1).6~rc1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">21</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">22</td>
<td align="center">x.(y+1)~rc1</td>
<td align="center"></td>
<td align="center">x.(y-1).6~rc2</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">23</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).6</td>
</tr>
<tr>
<td align="right" height="17">24</td>
<td align="center">x.(y+1)~rc2</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">25</td>
<td align="center">x.(y+1)~rc3</td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
<td align="center"></td>
</tr>
</tbody>
</table>
The last two columns are most visible to most visitors of the LibreOffice website. Those are the versions found on the <a href="http://www.libreoffice.org/download/libreoffice-fresh/">LibreOffice Fresh</a> and <a href="http://www.libreoffice.org/download/libreoffice-stable/">LibreOffice Stable</a> download pages. We are in roughly at week 18 after 4.2.0 release now, and the versions available are 4.2.4 fresh and 4.1.6 stable. A careful reader will note that according to that schedule we should be at 4.2.3 and 4.1.5 -- that is true, but the 4.2 series still had an <a href="https://wiki.documentfoundation.org/ReleasePlan#4.2_release">extra 4.2.1</a> intermediate release to adjust the schedule of 4.2 in direction of the current plan. This is not expected for future releases (also note that there is always some flexibility in the plan to allow for holidays etc.)

If you count all the prereleases, release candidates and releases, you will find that we do 25 of those in 26 weeks. Beside the fact that this is a lot of work for release engineers, one might wonder if anyone can keep up with that, and if so -- how? The answer to that depends on how you are using LibreOffice.
<h3>self deployment on LibreOffice fresh</h3>
If you are an user or a small business installing LibreOffice yourself, you will probably run LibreOffice fresh and the table above simplifies for you as follows:
<table border="0" cellspacing="0">
<tbody>
<tr>
<td align="center" valign="middle" height="34">week after x.y.0</td>
<td align="center" valign="middle">development</td>
<td align="center" valign="middle">release candidates</td>
<td align="center" valign="middle">finalized releases</td>
</tr>
<tr>
<td align="right" height="17">0</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.0</td>
</tr>
<tr>
<td align="right" height="17">1</td>
<td align="center"></td>
<td align="center">x.y.1~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">4</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.1</td>
</tr>
<tr>
<td align="right" height="17">6</td>
<td align="center"></td>
<td align="center">x.y.2~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">9</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.2</td>
</tr>
<tr>
<td align="right" height="17">11</td>
<td align="center"></td>
<td align="center">x.y.3~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">14</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.y.3</td>
</tr>
<tr>
<td align="right" height="17">18</td>
<td align="center">x.(y+1)~beta1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">20</td>
<td align="center">x.(y+1)~beta2</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">22</td>
<td align="center">x.(y+1)~rc1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">24</td>
<td align="center">x.(y+1)~rc2</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">25</td>
<td align="center">x.(y+1)~rc3</td>
<td align="center"></td>
<td align="center"></td>
</tr>
</tbody>
</table>
The last column shows the releases you are running. If you are a member of the LibreOffice community it would be very helpful if you also spend some time of this 6 months period for <strong>three actions</strong>:
<ul>
	<li>running at least one of the release candidates in the table (available for download <a href="http://www.libreoffice.org/download/pre-releases/">here</a>) before the final is released.</li>
	<li>running at least one beta releases in the table. Note that there will be a <a href="https://wiki.documentfoundation.org/BugHunting_Session_4.3.0">bug hunting session</a> on the 4.3.0 beta release this week, that will help you get started.</li>
	<li>running a <a href="http://www.libreoffice.org/download/pre-releases/">nightly build</a> once anywhere in the weeks 1-18. Note that if you are getting excited about seeing the latest and greatest builds while they are still steaming, there are tools that can help you with this on <a href="https://wiki.documentfoundation.org/Bibisect">Linux</a> and <a href="https://wiki.documentfoundation.org/Server_Install_GUI">Windows</a>.</li>
</ul>
If you do these each of these three things <em>once</em> in the timeframe of six months and <a href="https://www.libreoffice.org/get-help/bug/">report any issues</a> you find, you are helping LibreOffice already a lot -- and you are making sure that the finalized releases of the fresh series are not only containing all the latest features, but also free of severe regressions.
<h3>bigger deployments on LibreOffice stable</h3>
If you are not installing LibreOffice yourself, but instead have a major deployment administrated centrally, things are a bit different. You might be more conservative and interested in the releases from LibreOffice stable. And you probably have <a href="http://www.libreoffice.org/get-help/professional-support/">professional support</a> from a certified developer or a <a href="http://www.libreoffice.org/get-help/professional-support/">company employing certified developers</a>.
<table border="0" cellspacing="0">
<tbody>
<tr>
<td align="center" valign="middle" height="34">week after x.y.0</td>
<td align="center" valign="middle">development</td>
<td align="center" valign="middle">release candidates</td>
<td align="center" valign="middle">finalized releases</td>
</tr>
<tr>
<td align="right" height="17">1</td>
<td align="center"></td>
<td align="center">x.(y-1).4~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">4</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).4</td>
</tr>
<tr>
<td align="right" height="17">8</td>
<td align="center"></td>
<td align="center">x.(y-1).5~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">11</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).5</td>
</tr>
<tr>
<td align="right" height="17">13</td>
<td align="center">x.(y+1)~alpha1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">18</td>
<td align="center">x.(y+1)~beta1</td>
<td align="center"></td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">20</td>
<td align="center"></td>
<td align="center">x.(y-1).6~rc1</td>
<td align="center"></td>
</tr>
<tr>
<td align="right" height="17">23</td>
<td align="center"></td>
<td align="center"></td>
<td align="center">x.(y-1).6</td>
</tr>
</tbody>
</table>
If you intend to deploy one series of LibreOffice (e.g. 4.3), there are <strong>two things</strong> that are highly recommended to be done:
<ul>
	<li>make the alpha or beta releases available quickly to interested volunteers in your deployment early. They might find bugs or regressions that are specific to your use of the software.</li>
	<li>make the release candidates of versions that you intent to deploy available early to your users.</li>
</ul>
Of these two actions, the first is by far the most important: It identifies issues early on in the life cycle and gives both your support provider and the LibreOffice developer community at large time to resolve the issue. In fact, I would argue that if you have a major deployment, the only excuse for not making available prereleases, is that you made available nightly builds.
<h3>Ubuntu</h3>
So, Ubuntu qualifies as a "bigger deployment" and I have to take care of LibreOffice on it. Also people want to be able to run the latest and greatest LibreOffice releases from the LibreOffice fresh series. Do I follow my recommendations here? Yes, mostly I do:
<ul>
	<li>both <a href="https://launchpad.net/~libreoffice/+archive/ppa">LibreOffice fresh</a> and <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-4-1">LibreOffice stable</a> series are available from PPAs for Ubuntu and are updated regularly and quickly when an rc2 is available.</li>
	<li>prereleases are made available as <a href="https://wiki.documentfoundation.org/Bibisect#Versions">bibisect repositories</a> rather quick (build on Ubuntu 12.04 LTS). In addition, fully packaged versions of LibreOffice are build in the <a href="https://launchpad.net/~libreoffice/+archive/libreoffice-prereleases">prereleases PPA</a> as early as starting with beta1.</li>
</ul>
So, you are invited to run or test builds from these PPAs -- or download the bibisect repositories -- to keep LibreOffice releases coming in the steady and stable fashion they do. Finally, there is a bug hunting session for LibreOffice this week and as said above, no matter if you are running a huge deployment or installing on your own, you are helping LibreOffice -- and yourself, as a user of LibreOffice -- a lot by testing the prereleases:

<a href="https://wiki.documentfoundation.org/BugHunting_Session_4.3.0"><img class="aligncenter size-full wp-image-846" src="/static/img/wp/2014/05/bughuntban.png" alt="" width="378" height="113" /></a>

Originally published on 2014-05-20 19:26:10 on [wordpress](https://skyfromme.wordpress.com/2014/05/20/train-stops/).

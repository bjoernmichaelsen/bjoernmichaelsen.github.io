+++
title="Announcing BundesGit for LibreOffice"
date=2014-04-07
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
Announcing BundesGit for LibreOffice
====================================

<p style="text-align:right;"><em>"I fought the law and the law won"</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=FKIzjF25sP8"><em>-- Sonny Curtis and the Crickets -- prominently covered by the Clash</em></a></p>
<p style="text-align:left;">So in a few minutes, I will be leaving for the meeting at <a href="http://pad.opendatacloud.de/p/OK-Lab-HH">Open Knowledge Lab in Hamburg</a> for <a href="http://codefor.de/">Code for Germany</a> in Hamburg -- but I dont want to show up empty-handed. Earlier I learned about <a href="http://okfnlabs.org/blog/2012/12/13/bundesgit-german-laws-on-github.html">BundesGit </a>which is a project to put <strong>all federal german laws</strong> in a git repository in easily parsable markdown language. This project was featured prominently e.g. on <a href="http://www.wired.com/2012/08/bundestag/">Wired</a>, <a href="http://www.heise.de/open/meldung/Entwicklungshistorie-von-Gesetzen-mit-Git-verfolgen-1662758.html">Heise</a> and got me wondering that having all those laws available at the tip of your hand would be quite useful for lawyers. So here I went and quickly wrote an extension to do just that. When you install the extension:</p>

<ul>
	<li>it downloads all the german federal laws from github and indexes them on the next restart of LibreOffice (completely in the background without annoying the user)</li>
	<li>that takes about ~5 minutes (and it only checks for updates on the next start, so no redownload)</li>
	<li>once indexed you can insert a part of a law easily in any text in Writer using the common abbreviations that lawyers use for these:</li>
	<li>Type the abbreviation of the paragraph on an otherwise empty line, e.g. "gg 1" for the first Artikel of the Grundgesetz</li>
	<li>press Ctrl-Shift-G (G for Git, Gesetz or whatever you intend it to mean)</li>
	<li>LibreOffice will replace the abbreviation with the part of that law</li>
</ul>
[caption id="attachment_797" align="aligncenter" width="519"]<a href="/static/img/wp/2014/04/bundesgit.png"><img class="size-large wp-image-797" src="/static/img/wp/2014/04/bundesgit.png?w=519" alt="BundesGit for LibreOffice" width="519" height="258" /></a> BundesGit for LibreOffice[/caption]

Now this is still a proof-of-concept:
<ul>
	<li>It requires a recent version (1.9 or higher) of git in the path. While that is for example true in the upcoming version of Ubuntu 14.04 LTS, other distributions might still have older versions of git, or -- on Windows -- none at all: Packing a git binary into the extension is left as an exercise for the reader.</li>
	<li>I have not checked it to parse all the different laws and find all the paragraphs. It also ignores some non-text content in the repository for now. Patches welcome!</li>
	<li>While it stays in the background most of the time intentionally to not get into the way of the user, it could use some error reporting or logging, so users are not left in the dark if it fails to work.</li>
</ul>
On the other hand, the extension is a good example what you can do with<strong> less than 300 lines of Python3</strong> (including tests) in LibreOffice extensions. Thus the code was hopefully verbosely enough commented and was uploaded to <a href="https://gerrit.libreoffice.org/gitweb?p=sdk-examples.git;a=tree;f=BundesGit;h=2f5700e7687f5506c729c29f8a306591314647e0;hb=42523afbd67b1f9ac3f7cece8ce93fed8e3f3435">sdk-examples repository</a>, where it lives alongside this <a href="http://skyfromme.wordpress.com/2013/04/01/libreoffice-prints-on-tuesdays-only/">LibreOffice does print on Tuesdays extension</a> that also serves as an example. Of course, if there other useful repositories of texts online, it can be quickly adapted to provide those too.

<strong>So download <a href="http://people.canonical.com/~bjoern/presentations/bundesgit.oxt">BundesGit for LibreOffice</a> and test it on<a href="http://cdimage.ubuntu.com/daily-live/current/"> Ubuntu 14.04 LTS (trusty)</a>.</strong>

&nbsp;

addendum: This has been featured on <a href="http://www.golem.de/news/dank-git-aktuelle-bundesgesetze-in-libreoffice-1404-105687.html">golem.de</a> and <a href="http://www.linux-magazin.de/NEWS/Bundesgit-fuer-Libre-Office/%28language%29/ger-DE">linux-magazin.de</a> (both german).

&nbsp;
Originally published on 2014-04-07 16:39:45 at https://skyfromme.wordpress.com/2014/04/07/announcing-bundesgit-for-libreoffice/.

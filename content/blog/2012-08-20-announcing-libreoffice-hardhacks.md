+++
title="Announcing LibreOffice HardHacks"
date=2012-08-20
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
Announcing LibreOffice HardHacks
================================

<p style="text-align:right;"><em>So when your waiting for the next attack </em>
<em>You'd better stand there's no turning back</em></p>
<p style="text-align:right;">-- Iron Maiden, The Trooper</p>
I teased in the <a href="http://skyfromme.wordpress.com/2012/08/16/on-the-importance-of-being-a-bug-confirmer/">last post</a>, that there will be a second important change in LibreOffice QA in this week. Well, here it is: In an combined effort by contributors in QA and development, we will try to improve the way that QA and development interact. The LibreOffice <a href="http://wiki.documentfoundation.org/Development/Easy_Hacks_by_required_Skill">EasyHacks</a> are a well known success story of our project. Now we introduce the <a href="http://nabble.documentfoundation.org/minutes-of-ESC-call-td4001845.html"><strong>LibreOffice HardHacks</strong></a>. When I first threw around that buzzword, one initial reaction by a core developer was:
<blockquote><em>HardHacks sounds too hard to me. I would be afraid to look at such bugs :-)</em></blockquote>
This is what is hiding behind that buzzword:
<ul>
	<li>The QA team will start in their next call to identify the 5 most critical bugs that need attention. It will repeat that from now on in each of their calls every second week.
To qualify, these bugs have to be:
<ul>
	<li>Among the <a href="http://wiki.documentfoundation.org/QA-FAQ#How_to_ad_Bugs_to_MAB_Tracking_Bugs">MAB (most annoying bugs)</a></li>
	<li>be triaged as far as possible (e.g. reproduction scenario, version the bug was introduced if it is a regression, best: bibisect for regressions)</li>
	<li>so they are both important/urgent and well-prepared</li>
</ul>
</li>
	<li>These bugs will be handed over to the core developers</li>
	<li>ESC will try to find a core developer for each bug to look at it and report back in the next week</li>
</ul>
The aim of this is to keep awareness of these critical bugs high in development and having some kind of qualified feedback in the given timeframe (ideally one week, but at least until the next QA call). "Qualified feedback" might also be: "I still need this information to get forward with this bug." or "This cannot easily be solved because of foo" giving the QA and the reporter a hint on what is needed to push the issue forward. Hopefully this will prevent that awkward silence on some bugs where everyone thinks its on the other to push this forward and help us collecting all the needed information on the most important and urgent bugs. It will certainly also keep the most pressing quality issues present in the minds of the developers, which might otherwise strive to implement the next shiny and exciting new feature a little too early or too often. Keeping these bugs present will thus also help getting these though solved too.

[caption id="" align="alignnone" width="800"]<img class="   " src="http://upload.wikimedia.org/wikipedia/commons/6/66/Drawing-1.png" alt="Lots of bugs" width="800" height="800" /> Lots of bugs around - HardHacks are about those that hardened their chitin armor in unfair ways (source: wikimedia)[/caption]

As you can see from the comment above these are the bugs that even the core developers have a healthy dose of respect of. But that should not scare you: If you are a developer in the LibreOffice community and already have a few patches for EasyHacks under your belt, feel free to look at the HardHacks too. The core developers will be relieved for any support they can get. Also note that these bugs are really hard nuts to crack, so<strong> failure is an option</strong> here. However, on the other hand, the benefits of solving the bug are huge: both core developers and QA will be full of utmost respect and gratitude for such an achievement.

The QA team will also try to blog about the results we got for the batch of bugs that were selected -- this bit of extra fame is hopefully is another piece of motivation for the developers looking at bugs. Oh, and of course your help in cornering the bugs from both the QA and the development side is most welcome!

Originally published on 2012-08-20 00:17:44 on [wordpress](https://skyfromme.wordpress.com/2012/08/20/announcing-libreoffice-hardhacks/).

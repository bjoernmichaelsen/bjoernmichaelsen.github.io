+++
title="Yak Shaving Progress Report"
date=2019-09-24
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "libreoffice", "programming", "SwClient", "writer"]
+++
Yak Shaving Progress Report
===========================

<p style="text-align:right;"><em>Take out the papers and the trash</em>
<em>Or you don't get no spendin' cash</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=-WfDYssJMqs"><em>-- Yakety Yak, The Coasters</em></a></p>
 

At last years LibreOffice conference in Tirana I gave a talk on how <a href="https://speakerdeck.com/bjoernmichaelsen/death-of-a-sw-client">SwClient is considered harmful</a>. At this years LibreOffice conference in Almeria, I presented a <a href="https://speakerdeck.com/bjoernmichaelsen/quo-vadis-writer-memory-management">lightning talk, giving some updates on the progress</a>.

Additionally, with some <a href="https://gerrit.libreoffice.org/#/q/status:merged+owner:%22Bj%25C3%25B6rn+Michaelsen+%253Cbjoern.michaelsen%2540libreoffice.org%253E%22">recent idle changes</a> not only the unocore directory in Writer is free of the error prone old SwClient/SwModify combo, but also the directories:
<ul>
	<li>sw/source/core/view</li>
	<li>sw/source/filter/html</li>
	<li>sw/source/filter/basflt</li>
	<li>sw/source/filter/ww8</li>
	<li>sw/source/filter/xml</li>
	<li>sw/source/ui</li>
	<li>sw/source/uibase</li>
</ul>
So far, it seems the hope I expressed at the conference that getting rid of SwClient and SwModify (leaving aside the core layout for now) seems to be quite doable and well worth it for the errors and fragility this will prevent in those areas of code.
Originally published on 2019-09-24 17:31:07 on [wordpress](https://skyfromme.wordpress.com/2019/09/24/yak-shaving-progress-report/).

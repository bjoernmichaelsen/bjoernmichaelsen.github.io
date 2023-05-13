+++
title="MessageBox API change"
date=2014-05-12
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
MessageBox API change
=====================

<p style="text-align:right;"><em>I hope that someone gets my message in a bottle</em></p>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=MbXWrmQW-OE"><em>-- Message in a Bottle, the Police</em></a></p>
<p style="text-align:left;">So, there was <a href="http://nabble.documentfoundation.org/Versionshinweise-fur-Version-4-2-4-tp4108023.html">some minor confusion</a> about the wording in the <a href="https://wiki.documentfoundation.org/Releases/4.2.4/RC2#Extension_Compatibility_Version">LibreOffice 4.2.4 release notes</a>.</p>
<p style="text-align:left;">This needs some background first: LibreOffice 4.2 modified the UNO API to pop up a message box in a slight way against LibreOffice 4.1. This was properly announced in our <a href="https://wiki.documentfoundation.org/ReleaseNotes/4.2#API_Changes">LibreOffice 4.2 release notes</a> many moons ago:</p>

<blockquote>
<p style="text-align:left;">The following UNO interfaces and services were changed [...] com.sun.star.awt.XMessageBox, com.sun.star.awt.XMessageBoxFactory</p>
</blockquote>
<p style="text-align:left;">Luckily, LibreOffice extensions can specify a minimal version, so extensions using the new MessageBox-API can explicitly request a version of LibreOffice 4.2 or newer. <a href="https://gerrit.libreoffice.org/gitweb?p=sdk-examples.git;a=commitdiff;h=61f9ca7b18de1adc58cf5d7cb295ba81eb3e5d92;hp=20b8edc74b846db6d143a9e327fb7f7eb2c1fb77">This change</a> in our sdk-examples shows how an extension can be updated to use the new API and explicitly require a version of LibreOffice 4.2 and higher. All this happened already with LibreOffice 4.2.0 being released and has nothing yet to do with the change in LibreOffice 4.2.4.</p>
<p style="text-align:left;">So what was changed in LibreOffice 4.2.4? Well, in addition to the LibreOffice version, old extensions sometimes just ask for an "OpenOffice.org version". Most LibreOffice versions answered its version was "3.4", so this old backwards compatible check was not very helpful anyway. So in LibreOffice 4.2.4 this value <a href="https://gerrit.libreoffice.org/gitweb?p=core.git;a=commit;h=f0c38c8505a523f3bde3ee4fca3e513cdd6044d5">was changed</a> to  "4.1", which might make some old extensions aware of the incompatible API change. That's all.</p>
<p style="text-align:left;">Note that:</p>

<ul>
	<li>Most extensions using the MessageBox API have already been changed at 4.2.0 (or have been fixed by <a href="https://launchpad.net/ubuntu/trusty/+source/accessodf/0.1-4ubuntu1">Linux distros</a>)</li>
	<li><a href="https://wiki.documentfoundation.org/Development/Extension_Development">Extensions should use "LibreOffice-minimal-version"</a> anyway by now (see example above on how to do that), it is the best way to ensure you get a well known API with welldocumented changes.</li>
</ul>
So, the short answer to the question to "what changed in LibreOffice 4.2.4?" is: Nothing, if your extension uses LibreOffice-minimal-version as recommended.

Originally published on 2014-05-12 17:47:39 on [wordpress](https://skyfromme.wordpress.com/2014/05/12/messagebox-api-change/).

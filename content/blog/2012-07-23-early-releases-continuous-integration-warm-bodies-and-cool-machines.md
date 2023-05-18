+++
title="early releases, continuous integration, warm bodies and cool machines"
date=2012-07-23
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
early releases, continuous integration, warm bodies and cool machines
=====================================================================

All the streamlining in LibreOffice package done upstream at LibreOffice itself, <a href="http://anonscm.debian.org/gitweb/?p=pkg-openoffice/libreoffice.git;a=shortlog;h=refs/heads/debian-experimental-3.6">by Rene at Debian</a> and the tiny bits and pieces I do on top of that at Ubuntu are beginning to pay off:

Yesterday a package containing a prerelease of the <a href="https://launchpad.net/ubuntu/+source/libreoffice/1:3.6.0~rc2-0ubuntu3">upcoming LibreOffice 3.6 major series</a> has been uploaded to the upcoming <a href="https://launchpad.net/ubuntu/quantal">Ubuntu Quantal series (due to be released as stable in October 2012)</a> -- a month earlier than we did <a href="https://launchpad.net/ubuntu/+source/libreoffice/1:3.4.1-4ubuntu1">a year ago on Oneiric</a>. While this package is far from perfect in its current state, having it in this early is giving us a lot more real world test coverage -- of which upstream LibreOffice will without doubt profit too. This will help improving the LibreOffice we will finally ship with in Quantal as it gives us more time to detect and fix errors in all of LibreOffice, LibreOffice packaging and Ubuntu Quantal -- the platform it is build upon. So if you want to help in testing: <a href="http://cdimage.ubuntu.com/daily-live/current/">get Quantal</a> and start torturing LibreOffice on it.

This is approaching continuous integration as it should be. Speaking of which, not only do we have LibreOffice on Ubuntu being tested earlier by warm bodies, but also by cool machines: The <a href="https://jenkins.qa.ubuntu.com/view/Quantal/view/All%20Quantal/job/quantal-pkg-libreoffice/">Jenkins Bot Job</a> that dutifully builds the LibreOffice package daily -- and already gave early warning twice about changes in the stack below LibreOffice that caused the LibreOffice build to fail, which was very helpful -- will now be switched to build the head of the libreoffice-3-6 upstream branch. Development on the release branch has slowed enough so that we can use that test builder now not only to detect conflicts/incompatibilities in Quantal as it is moving ahead, but also in upstream LibreOffice on the release branch.

One nitpick that we should probably fix is <a href="https://bugs.launchpad.net/ubuntu/+source/libreoffice/+bug/1026059">the collision of the LibreOffice and Ubuntu branding in the new splash screen</a> -- something that was bound to happen with the two brand colors being orange and green:

<img class="aligncenter" title="Green and Orange" src="https://launchpadlibrarian.net/110384495/Captura%20de%20pantalla%20de%202012-07-18%2008%3A32%3A41.png" alt="" width="544" height="361" />

There are multiple possible solutions to this, the most simple one: removing the ubuntufied orange progress bar and making it white like upstream, but I cant help but notice that some of the other proposals like 'clean' although not winning the <a href="//plus.google.com/102673546895803839652/posts/FRtrAazgb6W">vote on g+</a> would not be that problematic. Your input on this topic is welcome, but having installed Quantal (and testing LibreOffice) is a prerequisite. ;)

Originally published on 2012-07-23 10:33:33 on [wordpress](https://skyfromme.wordpress.com/2012/07/23/early-releases-continuous-integration-warm-bodies-and-cool-machines/).

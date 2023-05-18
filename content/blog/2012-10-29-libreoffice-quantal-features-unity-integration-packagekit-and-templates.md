+++
title="LibreOffice Quantal features: Unity Integration, PackageKit and Templates"
date=2012-10-29
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice Quantal features: Unity Integration, PackageKit and Templates
=========================================================================

So the <a href="http://uds.ubuntu.com/">Ubuntu Developer Summit for Raring Ringtail (UDS-R)</a> is starting tomorrow -- although not exactly a slide-heavy conference, this is still an excellent opportunity to point out some of the new features available in LibreOffice 3.6.2 that comes with the just released Ubuntu 12.10 (Quantal Quetzal):
<ul>
	<li>first of all: all the <a href="http://www.libreoffice.org/download/3-6-new-features-and-fixes/">nifty new upstream features</a> like color scales, data bars, pdf watermarks, Office SmartArt import, Corel Draw import and <a href="http://www.libreoffice.org/download/3-6-new-features-and-fixes/">much, much more ...</a></li>
	<li>thanks to the awesome work of <a href="http://www.aentos.com/en/team">Antonio Fernandez of Aentos</a>: unity menu integration making lo-menubar as a separate package obsolete</li>
	<li><a href="https://launchpad.net/sessioninstaller">PackageKit/Session Installer</a> integration and a new package with a set of excellent templates by Alexander Wilms. There were <a href="http://wiki.documentfoundation.org/Design/Call_for_Templates">even more high quality templates</a> created by other contributors -- I plan to add those with the <a href="http://cgit.freedesktop.org/libreoffice/templates">next release</a>.</li>
</ul>
The PackageKit/Session Installer integration is implemented in UNO, that allow extensions and macro creators to trigger the installation of software from trusted archives in general -- quite a nifty feature in itself. As we have this now in place, in the future we can also use it to complete the LibreOffice install by adding missing packages for certain actions that are not available in the default Ubuntu installation (which leaves out some parts of LibreOffice).

For the next LibreOffice release, the template installer has to be adopted to the <a href="http://npcdoomlibreoffice.wordpress.com/2012/06/28/gsoc-template-dialog-ui/">cool work of Rafael on the template dialog</a>. I plan to upstream both the unity and the session installer integration for the next LibreOffice release.

This series of screenshots shows all of this in action:

<dl id="attachment_175" class="wp-caption aligncenter" style="width:529px;"><dt class="wp-caption-dt">[slideshow]</dt><dd class="wp-caption-dd">Installing the template pack from LibreOffice</dd></dl>

Originally published on 2012-10-29 00:12:15 on [wordpress](https://skyfromme.wordpress.com/2012/10/29/libreoffice-quantal-features-unity-integration-packagekit-and-templates/).

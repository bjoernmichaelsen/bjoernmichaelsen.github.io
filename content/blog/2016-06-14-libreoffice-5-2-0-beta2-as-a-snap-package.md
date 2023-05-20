+++
title="LibreOffice 5.2.0 beta2 as a snap package"
date=2016-06-14
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
LibreOffice 5.2.0 beta2 as a snap package
=========================================

<p style="text-align:right;"><em>What's been happening in your world?</em>
<em> What have you been up to?</em>
<em><a href="https://www.youtube.com/watch?v=H8tLS_NOWLs">-- Arctic Monkeys, Snap out of it</a></em></p>
<p style="text-align:left;">So -- here is what I have been up to:</p>


![LibreOffice 5.2.0 beta2 installed as a snap on Ubuntu 16.04](/img/wp/2016/06/libreoffice-5-2snap.png)

The upcoming LibreOffice 5.2 packaged as a nice new <a href="https://developer.ubuntu.com/en/snappy/">snap package</a>. This:
<ul>
	<li>is pretty much a vanilla build of LibreOffice 5.2 beta2, using <a href="https://github.com/ubuntu-core/snapcraft">snapcraft,</a> which is making packaging <a href="https://git.launchpad.net/~bjoern-michaelsen/df-libreoffice/+git/libreoffice-snap-playground/tree/?h=xenial&amp;id=61a7acbd2c71ce34eca0a8f4c221fa47104af002">quite easy</a></li>
	<li>contains all the applications: Writer, Calc, Impress, Draw, Math, Base</li>
	<li>installs easily on the released current LTS version of Ubuntu: 16.04</li>
	<li>allows you to test and play with the upcoming LibreOffice version to your hearts delight without having to switch to a development version of Ubuntu</li>
</ul>
So -- how can you "test and play with the upcoming LibreOffice version to your hearts delight" with this on Ubuntu 16.04? Like this:
<pre>wget http://people.canonical.com/~bjoern/snappy/libreoffice_5.2.0.0.beta2_amd64.snap{,.sha512sum}
sha512sum -c libreoffice_5.2.0.0.beta2_amd64.snap.sha512sum &amp;&amp; sudo snap install --devmode libreoffice_5.2.0.0.beta2_amd64.snap
/snap/bin/libreoffice</pre>
and there you have a version of LibreOffice 5.2 running -- for example, you can prepare yourself for the upcoming <a href="https://blog.documentfoundation.org/blog/2016/06/13/coming-up-the-next-libreoffice-bug-hunting-session/">LibreOffice Bug Hunting Session</a>. And its even quite easy to remove again:
<pre>sudo snap remove libreoffice</pre>
This is one of the things that snap packages will make a lot easier: upgrading or  downgrading versions of an application, having multiple installed in parallel and much more. Watch out as there are more exciting news about this coming up!

<strong>Update: </strong>As this has been asked a few times: Yes, snap packages are available on Ubuntu. No, snap packages are not only available on Ubuntu. <a href="https://insights.ubuntu.com/2016/06/14/universal-snap-packages-launch-on-multiple-linux-distros/">This text</a> has more details.

<strong>Update 2: </strong>The original download included debug symbols and thus was quite big. The download now has 287MB. <a href="https://skyfromme.wordpress.com/2016/06/16/a-third-of-a-libreoffice-snap/">This post</a> has all the details.

Originally published on 2016-06-14 08:03:42 on [wordpress](https://skyfromme.wordpress.com/2016/06/14/libreoffice-5-2-0-beta2-as-a-snap-package/).

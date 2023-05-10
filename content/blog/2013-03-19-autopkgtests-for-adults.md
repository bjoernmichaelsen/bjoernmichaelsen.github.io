+++
title="autopkgtests for adults"
date=2013-03-19
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
autopkgtests for adults
=======================

<p style="text-align:right;"><em>"That's not a knife -- THAT's a knife."</em></p>
<p style="text-align:right;"><a href="http://www.youtube.com/watch?v=sLS3RGesIFQ"><em>-- Michael J. Crocodile Dundee</em></a></p>
I recently worked a bit to see this line showing up in my favorite editor:

<code>ubtree0t-junit-subsequentcheck PASS</code>

LibreOffice has <a href="http://nabble.documentfoundation.org/subsequenttests-now-run-headless-td2750447.html">multiple sets of testsuites</a> and during the build of the package we run them all (although not yet on all platforms). However, LibreOffice depends on ~1/3 of main -- <a href="https://bugs.launchpad.net/ubuntu/+bug/958781">so</a> <a href="https://bugs.launchpad.net/ubuntu/+bug/745836">there</a> <a href="https://bugs.launchpad.net/ubuntu/+bug/1017125">are</a> <a href="http://launchpadlibrarian.net/67204040/libreoffice_1%3A3.3.2-1ubuntu1_1%3A3.3.2-1ubuntu2.diff.gz">a</a> <a href="https://bugs.launchpad.net/qemu/+bug/1129571">lot</a> <a href="https://bugs.launchpad.net/ubuntu/+bug/1146903">of</a> <a href="https://bugs.launchpad.net/ubuntu/+bug/248619">things</a> that might break LibreOffice. A lot of things just break at build time and not at run time and thus prevent the such a broken package to enter the archive in the first place as we run the tests during the build already. Thats as: unless the breakage is caused by an update of a dependency of LibreOffice, therefore making the LibreOffice package in the archive FTBFS (or at least broken) in a sneaky way. Thats whats happened for the e.g. the libjpeg, boost, kdelibs examples above.

But lets keep those aside for now and concentrate on the runtime issues. Running the tests at build-time is a good early-warning already and prevents some serious breakage to enter the archive. On the other hand, these tests do not run against LibreOffice as we install it in the system from packages -- it runs them against a installation set aside in the build tree. While I can not come up with an immediate example, where LibreOffice was broken when installed from the packages in a way that would have been detected by tests but missed when run against the in-tree installation, it would still be good to have the additional confidence that:
<ul>
	<li>LibreOffice passes the tests as installed on the system</li>
	<li>LibreOffice is not broken at runtime by some update of a dependency</li>
</ul>
In short: Its highly desirable to test that <strong>LibreOffice does still run and work as the ground below it keeps moving</strong> -- this is even more important when Ubuntu is considering to move towards a more rolling way of releasing. And we have a means to do that: <a href="http://anonscm.debian.org/gitweb/?p=autopkgtest/autopkgtest.git;a=blob_plain;f=doc/README.package-tests;hb=HEAD">Autopkgtests</a>.

So, what was needed to get this working for LibreOffice?

First, some parts of the testsuites are quite large and -- as we run the tests during the build anyway -- are already build during the build. Therefore it made sense to package these, which was <a href="http://anonscm.debian.org/gitweb/?p=pkg-openoffice/libreoffice.git;a=commit;h=edba0909a013f8bb33b696607fb2b73f0c9bfd26">done very early in the cycle</a> (actually: during UDS).

Second, we would need to get LibreOffice to run the tests without trying to build the product. That originally wasnt as easy as it may seem. For one, LibreOffice build system was reasonably expecting that you need a product to test it and therefore would have dependencies on the product to be build. In addition, when I started considering this, we still had a lot of the old build system around -- which was a pain to bend to your will. Luckily, <a href="http://skyfromme.wordpress.com/2013/02/28/one/">these times are over</a>. So, by now <a href="http://anonscm.debian.org/gitweb/?p=pkg-openoffice/libreoffice.git;a=blob;f=tests/patches/java-subsequentcheck-standalone.diff;h=383e379a8abc9647091814a6e09c420602ed9e4b;hb=2a632ddabf45572e7e84ca7843db7dda5f8de6a0#l119">a patch changing some ~15 lines get us what we want</a>.

Third, we need a config_host.mk (the output of ./configure), so that we can run the LibreOffice build. And for that, we unfortunately need the build dependencies (which are generated) of LibreOffice -- otherwise we would not really test what we did build. But for a <a href="http://bugs.debian.org/cgi-bin/bugreport.cgi?bug=693540">missing feature of autopkgtests,</a> we can not reuse the existing dependencies, but have to do manual double bookkeeping there. Im not thrilled by the prospect of hunting false positives there. Some possible ways out would be:
<ul>
	<li>to package the config_host.mk file into the package containing the other testsuite helpers, but that would make that package architecture dependendant</li>
	<li>or to not really specify the dependencies at all and pragmatically and greedily request the restrictions needs-root and breaks-testbed and then -- as we are root now -- run this before starting the tests:
<pre>apt-get build-dep -y libreoffice</pre>
</li>
</ul>
Finally, we should be able to run:
<pre>apt-get build-dep libreoffice
apt-get install libreoffice-subsequentcheckbase
apt-get source libreoffice
cd libreoffice-*
./debian/tests/junit-subsequentcheck</pre>
and this should run the tests locally and headless -- and indeed it does and the tests happily finish and report success. Great, lets quickly check if it also runs in the 'official' VM with:
<pre>run-adt-test</pre>
Nope, and this is why I choose the Crocodile Dundee quote below the title for this post: The VM fails before it even starts the tests -- it does not even have enough discspace to copy in the LibreOffice source package. This needs to be fixed on the side of the image, there is nothing on the test side that could fix this. But to test if LibreOffice would finish if only the image could handle it, I began cannibalizing, removing one after another the directories of the icon-themes, translations and external sources from the package, each time getting a bit further: from failing to start to failing when installing the 501 additional packages and so on. With this hollowed out package, I could verify: <em>yes, the autopkgtest would pass in the image, if only it had enough discspace</em>.

Finally, once this is in the archive (or ppa) you will also be able to run:
<pre>apt-get build-dep libreoffice
apt-get install libreoffice-subsequentcheckbase
apt-get source libreoffice
cd libreoffice-*
libreoffice '--accept=pipe,name=blickenlights;urp'&amp;
./debian/tests/junit-subsequentcheck 'connect:pipe,name=blinkenlights' 1</pre>
This will connect to the LibreOffice you started in the second-to-last step (which is not headless, but running in your session) and run the tests against it. The "1" tells it not to use parallelization, but just run one suite at a time, as otherwise you have a very good chance to lock/hang your own session by compiz (or the dash or other components) being mightly confused by all the windows flashing up and closing in fast progression. With "1" you might still get some test failures (mostly from the a11y integration) -- but at least your session will survive:
<p style="text-align:center;"><strong>ZO RELAXEN UND WATSCHEN DER BLINKENLICHTEN.</strong></p>
<p style="text-align:left;"><strong>Addendum:</strong></p>
<p style="text-align:left;">Preparing an adt-image with:</p>

<pre style="text-align:left;">./bin/prepare-testbed -r raring amd64 -S12GB</pre>
seems to solve the issue. The "df -h" at the start of the test reports some 3GB of free space (with 2.6GB being needed still to create a rw-copy of the source tree after that point). So 12GB is likely the size the images on Jenkins roughly currently need (plus maybe another 1GB of wiggle room).
Originally published on 2013-03-19 16:54:22 at https://skyfromme.wordpress.com/2013/03/19/autopkgtests-for-adults/.

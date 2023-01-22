+++
title="Building Libreoffice in half an hour on Ubuntu Oneiric (without cheating)"
date=2011-10-24
[taxonomies]
originally-published-on=["livejournal"]
+++
Building Libreoffice in half an hour on Ubuntu Oneiric (without cheating)
=========================================================================

I recently bought myself an old IBM x3800. After the 65kg has been moved into my apartment and Oneiric has been installed on it, I tried to have a look at how fast I could get with my hardware. So I cleaned my ccache (this is the &quot;without cheating&quot; from the title) and started a build from my notebook with distccd running on the other machines.<br />So all in all I had:<br /><ul><li>one Lenovo W520 notebook (4 core i7-2720QM with 16GB RAM)</li><li>one Sun Ultra 24 workstation (4 core Q9650 with 8GB RAM)</li><li>one IBM x3800 (8 core Xeon MP 3.16Mhz with 8GB RAM)</li></ul>and the development build<sup>1)</sup> chugged in after 32 Minutes. Rebuilding from ccache took ~8 minutes on that machine, so &quot;pure compile time&quot; was ~25 Minutes. Of course, starting the build on one of the other machines would likely have been a bit faster still as:<ul><li>the Sun Ultra is about twice as fast alone as the notebook alone (full build in &lt;1 hour)</li><li>the notebook did not run on a software RAID-0 (like the others would)</li><li>the notebook did not mount the build platform with noatime, which would be good for a few additional minutes of saved time</li></ul>So: When doing a lot of work on Libreoffice, RAID-0 and noatime (together with distcc and ccache) are to be considered. However, If you are new the code, you probably only want to care about:<ul><li>ccache</li><li>--disable-mozilla</li><li>--disable-binfilter</li></ul>all the other stuff is only getting interesting once you are doing very regular work on Libreoffice. And an since it is getting winter again on the northern hemisphere: An IBM x3800 is a nice way to heat your apartment, if it was not for the noise and the electricity bills.<br /><br /><span style="font-size: smaller;"><sup>1)</sup> 2 jobs per core and:</span><br />

    ./autogen.sh \
      --disable-mozilla --disable-binfilter --disable-zenity \
      --with-junit=${HOME}/.jenkins/junit-4.9b2.jar \
      --with-external-tar=`readlink -f ${WORKSPACE}/../../tarfiles/workspace` \
      --without-help --without-myspell-dicts \
      --with-system-libs --without-system-libvisio \
      --without-system-libcmis --without-system-jars \
      --without-system-graphite --without-system-lpsolve \
      --without-system-libexttextcat --without-system-poppler \
      --enable-librsvg=no --with-num-cpus=35 --with-max-jobs=35

This was originally published at 2011-10-24 17:13:00/2011-10-24 15:13:39 on [livejournal](https://sweetshark.livejournal.com/5905.html).

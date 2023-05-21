+++
title="trusty old horses"
date=2018-10-18
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice"]
+++
trusty old horses
=================

<p style="text-align:right;"><em>Will you still need me, will you still feed me</em></p>
<p style="text-align:right;"><em>When I'm sixty-four?</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=8AglUMCKyns">— When I'm sixty-four,  Sgt. Pepper's Lonely Hearts Club Band, The Beatles</a></em></p>

So almost six years ago I started using <a href="https://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/">my LibreOffice development rig "Big Bertha"</a>. Back then I thought: "This likely will be my last big machine, the next time I wont buy anything for under my desk, I will just build in the cloud.". Six years on, that is partially true: I did not buy a new "desktop" machine. On the other hand, I am not building on some cloud machine either: I still use good old Bertha. So, why that?

For one, the speed of processors stopped improving at the insane rate it had before, so indeed machines that one buys today are not that much faster than those six years ago. However, cloud computing prices also stopped dropping like they used to. Which leaves me with not much reasons to buy new hardware -- but also with little reason to consider to accept the additional inconveniences that come with building on remote hardware.

In 2012, the fastest build of a then-master checkout of LibreOffice from scratch and without caches I got out of Big Bertha was <a href="https://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/">in ~18 minutes</a>. I have not tried it again with present day LibreOffice -- I assume it to be quite a bit slower, if only because e.g. we added tests left and right. But still: the old machine under the desk is still competing. A look at the numbers: A <a href="https://aws.amazon.com/ec2/pricing/on-demand/">c4.8xlarge</a> "compute-optizimed" instance on EC2 promises 36 vCPUs and 60GiB of memory. Now those 36 cores might be faster than those on the old Opteron 6272s I am running on. But I assume not much: CPUs really did not get much faster in the last six years -- especially for workloads like compiling LibreOffice. For the most part they got better in other ways though: they use less power and push out less heat. So in the end the two Opteron 6272 with 32GB are likely still competing quite well with the stuff available from cloud providers.

So, why am I posting this? Buying a modern Server CPU still costs a fortune: <a href="https://www.heise.de/preisvergleich/amd-epyc-7281-ps7281bevgaaf-a1643088.html?hloc=at&amp;hloc=de">a 16-core AMD Epyc 7281 costs 672 EUR</a> at the time of posting. A new full machine with two of those comparable to Big Bertha will cost 2000-3000 EUR. Everybody loves AMDs Epycs apparently. But this challenge is also an opportunity: If one does not buy new hardware. Looking for "Opteron 6272" and "Opteron 6276" I found full systems available for <a href="https://www.ebay.com/itm/A2-UXS-Server-1U-Supermicro-H8DGU-F-2x-AMD-G34-Opteron-6272-16-Core-32GB-2x-PS/132691878465">410 USD</a> or even a 64-core, 256MB RAM system for <a href="https://www.ebay.com/itm/DELL-PowerEdge-R815-4x-Opteron-6276-64-cores-2-3Ghz-256GB-H700-2-5-6-bay-Server/283080863447">999 USD</a>. These systems might be making too much heat and eating too much electricity: When I measured Bertha back in the day, <a href="https://skyfromme.wordpress.com/2013/08/20/powerplay/">compiling used ~400 Watt</a>. At local electricity prices of 0.30 EUR/KWh that is 0.12 EUR per hour. Even if the c4.8xlarge on EC2 might be a bit faster, it still costs 10x as much. One should not make decisions without measuring, but its unlikely to be that much faster.

So I guess Im saying: If your LibreOffice build is too slow, have a look at the results on ebay for "opteron 6272" or "opteron 6276". Those beasts might have served their time in the datacenter, but they may still be a steal for LibreOffice development. Or at least they should be worth a consideration ...

Originally published on 2018-10-18 20:05:14 on [wordpress](https://skyfromme.wordpress.com/2018/10/18/1462/).

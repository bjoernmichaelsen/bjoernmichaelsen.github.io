+++
title="LibreOffice goes money printing"
date=2012-04-01
[taxonomies]
originally-published-on=["livejournal"]
+++
LibreOffice goes money printing
===============================


You may have heard the rumors already although there is nothing official yet:
There is an exciting new idea making the rounds in LibreOffice circles.
LibreOffice made and continues to make amazing progress pushing not only the
product, but also the project as a whole to new levels. While the development
of LibreOffice at the Document Foundation is widely supported by a diverse set
of contributors and supporters from all over the world, there is always more we
could do, if we had more time and money.

At least for the second problem, we are confident of having found the ultimate
solution<sup>(1)</sup>: We will just print whatever money we could use. Since 
[PostScript](http://en.wikipedia.org/wiki/PostScript) is:

* the de facto standard to communicate layout to printers
* and is turing-complete
* and most modern printers are actually rather capable computers

we will use this to make printers generate a fraction of a [Bitcoin](http://bitcoin.org/)
with every page that is printed. As modern printers have serious computation
power the performance impact will hardly be noticeable to the end user. 

Now, quite a few documents rarely ever get printed: They get distributed for
consumption in formats like PDF ([LibreOffice can generate those in an editable
form, which is truely a great feature](http://blogs.computerworlduk.com/simon-says/2012/03/the-magic-of-editable-pdfs/index.htm)). To also tap into that potential
resource, PDFs generated by LibreOffice will also inject a small Bitcoin
generating implementation written in JavaScript, which will execute when the
document is viewed with Software like Adobe Acrobat. Since -- in the light of
cloud computing -- a lot of progress has been recently made in creating
high-performance implementations of the carefully designed language that is
JavaScript, we are confident that the Bitcoin generation in the background will
hardly have any impact on the user experience for the viewer also in this case.

We are looking forward to complete the generation of a prototype for this at
the [Hackfest on April 14th/15th, 2012 in Hamburg](http://wiki.documentfoundation.org/Hackfest/Hamburg2012):

![Hackfest Logo](/img/lj/2012-04-01-hamburg-hackfest.png)

Now we recognize that while the generation of Bitcoins on printers and in
document viewers is universally possible, there might be some environments
(locked down corporate ones, for example) where transferring the coins as a
donation to the benefit of the LibreOffice project might be problematic. To
ensure these users are not left out in the cold when it comes to contributing
back, we will leave our
[traditional donation page](http://www.libreoffice.org/get-involved/donate/) online even after this feature is introduced.
[Feel free to use it!](http://www.libreoffice.org/get-involved/donate/)

<sup>(1)</sup> Although we cannot claim to be the sole originator of the idea:
We took inspiration from central banks all over the world. 

This was originally published at 2012-04-01 04:11:00/2012-04-01 04:11:00 on [livejournal](https://sweetshark.livejournal.com/10372.html).
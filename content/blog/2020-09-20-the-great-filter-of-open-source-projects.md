+++
title="The great filter of open source projects"
date=2020-09-20
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
The great filter of open source projects
========================================

<!-- wp:paragraph {"align":"right"} -->
<p class="has-text-align-right"><em>This is how you make yourself vanish into nothing</em><br><em><a href="https://www.youtube.com/watch?v=ZtgPeNKpnyw">-- 24 frames, Something More Than Free, Jason Isbell and the 400 unit</a></em><br></p>
<!-- /wp:paragraph -->

<!-- wp:paragraph {"className":"has-text-align-left"} -->
<p class="has-text-align-left">So, with the recent layoffs at Mozilla -- among other things -- a bit of discussion on the sustainability of open source projects has been reignited.  There was a wide range of takes: from <a href="https://twitter.com/zkat__/status/1293627087488598016?s=19">"FOSS is dead"</a> (no) to <a href="https://twitter.com/jwildeboer/status/1296009553105104896?s=19">"we need to re-decentralize the internet"</a> (yes). I could not quite help putting forth opinions on the matter myself and did so on a <a href="https://twitter.com/Sweet5hark/status/1303065894533427201">short twitter thread</a>. Fundamentally though, the opinions expressed on this matter seem to almost talk past each other -- and I think the reasons for this might be found in history of open source<sup>(1)</sup>.</p>
<!-- /wp:paragraph -->

<!-- wp:heading {"level":3} -->
<h3>users are contributors</h3>
<!-- /wp:heading -->

<!-- wp:paragraph -->
<p>In the beginning, pretty much all open source projects had an almost complete overlap of users and contributors. Or at least <em>potential</em> contributors. E.g. the original GNU projects pretty much solved the problems of their own contributors and even most of those that where not contributors (yet) where non the less in the majority able to contribute to all parts of them. You can also see it in the original announcement of Linux as </p>
<!-- /wp:paragraph -->

<!-- wp:quote -->
<blockquote class="wp-block-quote"><p><a href="https://groups.google.com/forum/?hl=en#!msg/comp.os.minix/dlNtH7RRrGA/SwRavCzVE7gJ">"just a hobby, won't be big and professional like gnu"</a></p></blockquote>
<!-- /wp:quote -->

<!-- wp:paragraph -->
<p>and while Linux grew way beyond that, there are still indications of this being true today when the main (and only) collaboration tool of the project, the LKML, <a href="https://linux.slashdot.org/story/18/01/14/1545250/the-linux-kernel-mailing-list-is-down">was down for an extended time in 2018, because a mail server in a cabinet at someones home was not booting through after an power outage and the person to kick it was on vacation</a>. The Linux kernel project was still self-hosting its infrastructure in 2018.</p>
<!-- /wp:paragraph -->

<!-- wp:paragraph -->
<p>Another -- later -- project, that I am assuming to have been quite resilient and which I am assuming will continue to be quite resilient is gentoo linux: By requiring users to compile all software themselves, this distribution makes their users either give up on their installs or gets them at least halfway to be packagers (and for a distribution, packagers are contributors) themselves. Also, by not having to deal with binaries, gentoo reduces its infrastructure needs to a minimum. And even while there are some signs of <a href="https://blogs.gentoo.org/mgorny/2020/08/25/is-an-umbrella-organization-a-good-choice-for-gentoo/">downsizing at gentoo</a>, I am hopeful that the flexibility mentioned above makes gentoo more sustainable and self-reliant than others for quite some time to come.</p>
<!-- /wp:paragraph -->

<!-- wp:heading {"level":3} -->
<h3>users are not all contributors anymore</h3>
<!-- /wp:heading -->

<!-- wp:image {"id":1491,"sizeSlug":"large"} -->
<figure class="wp-block-image size-large"><img src="https://skyfromme.files.wordpress.com/2020/09/4fnakd.jpg?w=888" alt="" class="wp-image-1491" /></figure>
<!-- /wp:image -->

<!-- wp:paragraph -->
<p>In the 2000-2010 decade, especially in the second half, a lot of open source projects joined the choir were only a minority of users were contributors too. For many, the majority of users were not even possible contributors across the project. For a few, contribution was even implicitly limited to a closed circle. This gave rise to the perception that something like an "open source product" exists -- especially by users. Here are some examples:</p>
<!-- /wp:paragraph -->

<!-- wp:table -->
<figure class="wp-block-table"><table><tbody><tr><td><strong>project</strong></td><td><strong>complement</strong></td><td><strong>contributors goal</strong></td></tr><tr><td>Mozilla</td><td>web</td><td>prevent Microsoft from monopolizing the web, later generalized</td></tr><tr><td>OpenOffice.org</td><td>enterprise productivity</td><td></td></tr><tr><td>Android</td><td>web/apps</td><td>prevent Apple from monopolizing the web, app and smartphone market</td></tr><tr><td>Chrome</td><td>web</td><td>protect a cloud, an advertising business and a search engine</td></tr><tr><td>Ubuntu desktop</td><td>many</td><td>changed over time, my guesses: initially, make <a href="https://launchpad.net/">launchpad</a> what github is today, then complement the <a href="https://en.wikipedia.org/wiki/Ubuntu_Touch">Ubuntu phone</a>, then cloud offerings, but <a href="https://www.theregister.com/2020/09/14/ubuntu_community_council_revived/">now something new maybe up</a> </td></tr></tbody></table></figure>
<!-- /wp:table -->

<!-- wp:paragraph -->
<p>Of course, <a href="https://www.linux.com/audience/enterprise/what-are-open-source-products/">no such thing as an "open source product" exists</a> and all of the above are variations of <a href="https://www.joelonsoftware.com/2002/06/12/strategy-letter-v/">Strategy Letter V</a>, which -- being from 2002 -- was already old by the time most of those were started:</p>
<!-- /wp:paragraph -->

<!-- wp:quote -->
<blockquote class="wp-block-quote"><p><strong>Smart companies try to commoditize their products’ complements.</strong></p></blockquote>
<!-- /wp:quote -->

<!-- wp:paragraph -->
<p>All of the above projects, commoditized their complements and this allowed users, who were not contributors to still benefit from the work of those who were as these contributors were interested in protecting the complement. With technology moving to the web and the cloud you can see this pattern repeating there in a few examples:</p>
<!-- /wp:paragraph -->

<!-- wp:table -->
<figure class="wp-block-table"><table><tbody><tr><td><strong>project</strong></td><td><strong>complement</strong></td><td><strong>contributors goal</strong></td></tr><tr><td>react</td><td>facebook</td><td>ensure all browser stay compatible with own use by making the framework widely used</td></tr><tr><td>istio</td><td>cloud hosting</td><td>raising the barrier to entry for microservice hosting (embrace and extend style)</td></tr></tbody></table></figure>
<!-- /wp:table -->

<!-- wp:heading {"level":3} -->
<h3>the great filter of open source</h3>
<!-- /wp:heading -->

<!-- wp:paragraph -->
<p>So, what does this have to do with the layoffs at Mozilla and the current struggles at other open source projects? Well, the complements from strategy letter V that might motivate contributors to work on projects beyond their own needs exist at a given point in time, but ... <em>panta rhei</em>, and this gives different outlooks for projects depending on how broad the complements are that it is serving:</p>
<!-- /wp:paragraph -->

<!-- wp:list -->
<ul><li>Mozilla is financed by Google for selection of the default search engine as almost single source of income, but once Chrome came around and hugely successful, <a href="https://www.zdnet.com/article/sources-mozilla-extends-its-google-search-deal/">the motivation of the biggest/single contributor is becoming more and more questionable.</a></li><li>Given online collaboration platforms, desktop enterprise productivity is not much of a business anymore these days -- certainly not as a product, maybe still as a service, <a href="https://lwn.net/Articles/825602/">challenging the biggest contributors to LibreOffice</a>.</li><li>The Linux kernel on the other hand is continuously serving a lot of users that are not contributors: While it very early on started self sustaining by just serving its contributors, it grew so many complements quickly that it is not endangered by one contributing business failing.</li><li>The Blender project created <a href="https://fund.blender.org/">a broad alliance of content creators, gaming platforms, gaming hardware producers all pushing (and financially supporting) the project to exist as a commodity to enjoy their complements</a>.</li></ul>
<!-- /wp:list -->

<!-- wp:paragraph -->
<p>Ultimately, open source projects provide a commodity. If their infrastructure needs are limited and their users also contributing they should be quite resilient (see e.g. gentoo). If they have many non-contributing users but have multiple complementing products, they will likely do well too.</p>
<!-- /wp:paragraph -->

<!-- wp:paragraph -->
<p> However, having non-contributing users and only one complement might just be <a href="https://en.wikipedia.org/wiki/Great_Filter">the Great Filter for open source projects</a>: Once this complement is vanishing, so will the project -- at least for non-contributing users. The remaining contributors -- those that work without the need for a product, a complement, a business or users -- will not feel that too much. But the non-contributing users will, as they wont be relevant at all anymore when the project downsizes itself to serving its contributing hobbyists alone.</p>
<!-- /wp:paragraph -->

<!-- wp:heading {"level":3} -->
<h3>conclusions and unrequested advice</h3>
<!-- /wp:heading -->

<!-- wp:paragraph -->
<p>So, at least for Mozilla and LibreOffice, I have some opinions and unrequested advice. In general, I see an urgent need for open source projects to establish a shared understanding among contributors what currently is a commodity -- and thus is governed by the project and its institutions -- and what are the complements of the commodity. For Mozilla and LibreOffice this might mean:</p>
<!-- /wp:paragraph -->

<!-- wp:list -->
<ul><li><a href="https://blog.fefe.de/">Felix von Leitner </a>recently suggested at heise.de that the German government could make earmarked donations to Mozilla so protect digital sovereignty: <a href="https://www.heise.de/meinung/Kommentar-Digitale-Souveraenitaet-zum-Schnaeppchenpreis-von-Europa-und-Mozilla-4874038.html">"Die Bundesregierung könnte der Mozilla-Stiftung zweckgebundene Spenden zukommen lassen." </a><br>While I agree with both the goal and concerns over spending the money on a "cyber agency" might dilute results, earmarked donations are a also a very painful device to use: The administrative friction between a donor and the foundation will imply a huge overhead.<br>Mozillas projects might be better off opening themselves to outside contributions and diversifying its contributions<sup>(2)</sup>. And if there is indeed money in the German or European government to spend on strategic goals like digital sovereignty, tendering those to capable local providers with a crystal clear purpose will have much better outcomes. Both <a href="https://www.heise.de/news/Open-Source-Konzept-fuer-Depot-zum-Code-Austausch-in-der-Verwaltung-steht-4891319.html">FSFE and OSBA</a> might contribute starting points and help find suitable partners.<br>All of this might seem very local, but likely it is not: Mozilla should look globally into diversifying the contributions to their projects, even if it might reduce their control as sole maintainer of them.</li><li>LibreOffice needs to decide where it really wants to provide a commodity and what its complements are: Both in the dimension of online collaboration vs. desktop and enterprise users vs. home users. The space claimed for commodity should be broad enough to motivate non-commercial contributors and to allow to grow into new complements when they appear, but overextending will leave too little room for complements, products and ultimately users. When the space allocated to be a commodity is overextended, the project will downsize to be a project solely serving its most active contributors<sup>(3)</sup> leaving aside non-contributing users.<br>I see this responsibility at the board of the Document Foundation and there alone. It is elected for this as representatives of the community of contributors for exactly decisions like this.</li></ul>
<!-- /wp:list -->

<!-- wp:paragraph -->
<p>Maybe not all of <a href="https://twitter.com/hondanhon/status/1306446536490905601">the dream</a> is lost.</p>
<!-- /wp:paragraph -->

<!-- wp:paragraph {"align":"left"} -->
<p class="has-text-align-left">  (1) Note that I am no expert on the history of open source nor do I know the internal workings and politics of all the open source projects, so there is unfortunately a lot of conjecture over the limited body of my own experience.</p>
<!-- /wp:paragraph -->

<!-- wp:paragraph -->
<p>(2) This is where I might disagree with <a href="https://people.gnome.org/~michael/blog/2020-08-11.html">Michael Meeks' take on that</a>.</p>
<!-- /wp:paragraph -->

<!-- wp:paragraph -->
<p>(3) The Document Foundation receives significant donations from individuals compared to other open source projects. <a href="https://people.gnome.org/~michael/blog/2020-09-15.html">Michael Meeks has some numbers on the value of developer contributions in kind to compare them to and put them in perspective.</a></p>
<!-- /wp:paragraph -->

Originally published on 2020-09-20 20:49:56 on [wordpress](https://skyfromme.wordpress.com/2020/09/20/the-great-filter-of-open-source-projects/).

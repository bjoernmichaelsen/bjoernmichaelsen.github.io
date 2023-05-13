+++
title="Powerplay"
date=2013-08-20
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
Powerplay
=========

<p style="text-align:right;"><em>I've got the power.</em></p>
<p style="text-align:right;"><a href="http://www.youtube.com/watch?v=z33tH-JdPDg"><em>-- The power, Snap</em></a></p>
<p style="text-align:left;">So, Im back from vacation. One of the things I did was reorganizing my hardware, and for doing so, I bought a <a href="https://en.wikipedia.org/wiki/Wattmeter">wattmeter</a> to measure what my machines and toys actually consume. A lot of the stuff was what I expected, but there where a few nasty surprises:</p>

<table border="0" cellspacing="0">
<tbody>
<tr>
<td align="LEFT" height="17"><i> (all values in Watt)
</i></td>
<td align="CENTER"><i>Ideapad S12</i></td>
<td align="CENTER"><i>Thinkpad W520</i></td>
<td align="CENTER"><a href="http://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/"><i>Bertha</i></a></td>
<td align="CENTER"><i>TV</i></td>
<td align="CENTER"><i>Pandaboard ES</i></td>
</tr>
<tr>
<td align="LEFT" height="17"><i>power supply only</i></td>
<td align="CENTER">0</td>
<td align="CENTER">0.2</td>
<td align="CENTER">2.5</td>
<td align="CENTER">0.3</td>
<td align="CENTER">0.1</td>
</tr>
<tr>
<td align="LEFT" height="17"><i>standby</i></td>
<td align="CENTER">0.3</td>
<td align="CENTER">0.2</td>
<td align="CENTER">2.5</td>
<td align="CENTER" bgcolor="#CCCCCC"><b>15</b></td>
<td align="CENTER">3.2</td>
</tr>
<tr>
<td align="LEFT" height="16"><i>desktop w/o display</i></td>
<td align="CENTER" bgcolor="#CCCCCC"><b>13</b></td>
<td align="CENTER">10</td>
<td align="CENTER">122</td>
<td align="CENTER" bgcolor="#CCCCCC"><b>130</b></td>
<td align="CENTER">6.1</td>
</tr>
<tr>
<td align="LEFT" height="16"><i>with display</i></td>
<td align="CENTER">18</td>
<td align="CENTER">16</td>
<td align="CENTER">180</td>
<td align="CENTER">100</td>
<td align="CENTER"></td>
</tr>
<tr>
<td align="LEFT" height="17"><i>g+/gmail</i></td>
<td align="CENTER">20</td>
<td align="CENTER" bgcolor="#CCCCCC"><b>20</b></td>
<td align="CENTER" bgcolor="#CCCCCC"><b>212</b></td>
<td align="CENTER"></td>
<td align="CENTER"></td>
</tr>
<tr>
<td align="LEFT" height="16"><i>compiling</i></td>
<td align="CENTER"></td>
<td align="CENTER">90/70/35</td>
<td align="CENTER">417</td>
<td align="CENTER"></td>
<td align="CENTER">8.1</td>
</tr>
</tbody>
</table>
From this set a few surprising takeaways:
<ul>
	<li>The wimpy Ideapad S12 with its Atom CPU eats <strong>more</strong> power when idling than the Thinkpad W520 with its beefy i7 Quad-Core and 16GB of RAM (13 Watts vs. 10 Watts).</li>
	<li>My TV doing nothing but waiting for the remote to tell it to turn itself on eats more power that each of my notebooks (15 Watts vs. 10/13 Watts).</li>
	<li>Just opening Firefox with one tab google plus and one tab google mail eats 4 extra Watts on my notebook and <strong>32 extra Watts on my desktop</strong>. It seems all that JavaScript voodoo does not come free at all: ~6 Euros per month when I leave it open on my desktop all the time.</li>
	<li>Running my desktop (<a href="http://skyfromme.wordpress.com/2012/11/12/dicke-bertha-online/">Bertha)</a> as an tinderbox for LibreOffice 24/7 would cost me ~1.000EUR per annum. Doing it with three of those boxes would a very expensive and noisy alternative to what others sell as a <a href="http://www.ebay.com/itm/iLIVING-1500-Watts-Electric-Portable-Fireplace-Space-Heater-Remote-w-Flame-/350529899499?pt=US_Fireplaces&amp;hash=item519d35cbeb">room heater</a>.</li>
	<li>My TV eats 30 Watts more when displaying the black screen of a disconnected HDMI signal than with normal TV display. Maybe its expensive to search for a signal?</li>
	<li>Compiling LibreOffice without ccache on my Notebook kicks the power consumption to 90 Watts -- but only for a few minutes. Then the thermal controls throttle the machine down to 70 or even 35 Watts, which seems all the machine can disperse over sustained periods.</li>
</ul>
[caption id="" align="aligncenter" width="600"]<a href="http://www.flickr.com/photos/tolomea/7067177993/sizes/c/in/photolist-bLv8Nr-78moA1-78hv6e-78hv24-78hvrZ-78hwFX-78huqp-78hw4T-78hvhT-y5teo-bTczhe-a3giZD-9nQuFg-8mVVXk-24baQ-atkT63-5usNEo-5usKiC-5uoo9P-ath8qL-7A2TfH-533GkB-9zXxP4-9zXzdX-9zXASH-P8V8J-89bnSv-6kbuMj-73czGZ-4kymTY-dEBaUS-dELMuj-5hmBZs-6aXQez-6aXSd2-6aXRBZ-6b2ZgW-aBA61h-aBzA1y-aBwYMV-aBzYn5-aBzCvo-aBA2TL-aBxhGe-aBzwWu-aBwU4a-aBzH1Q-aBxoeD-aBxeUF-aByNPY-aBxdqX/"><img alt="" src="http://farm8.staticflickr.com/7092/7067177993_4989070568_c.jpg" width="600" height="800" /></a> My electricity is 100% from water power, btw. Admittedly -- its unlikely to come from the Hoover dam, though (Image copyright CC BY 2.0 by Gordon Wrigley)[/caption]

And then there where these leftover pieces to measure, no surprises there, just a confirmation of my suspicion that the old Asus notebook I run as a home server is eating way too much power:
<table border="0" cellspacing="0">
<tbody>
<tr>
<td align="LEFT" height="17"><i>(all values in Watt)
</i></td>
<td align="CENTER"><i>bits and pieces</i></td>
</tr>
<tr>
<td align="LEFT" height="17">mic preamp off</td>
<td align="CENTER"><i>1.1</i></td>
</tr>
<tr>
<td align="LEFT" height="17">mic preamp on</td>
<td align="CENTER"><i>10</i></td>
</tr>
<tr>
<td align="LEFT" height="16">hub</td>
<td align="CENTER"><i>5</i></td>
</tr>
<tr>
<td align="LEFT" height="16">phone</td>
<td align="CENTER"><i>4</i></td>
</tr>
<tr>
<td align="LEFT" height="16">"home server" (decommissioned Asus Z53 notebook)</td>
<td align="CENTER"><i>30</i></td>
</tr>
</tbody>
</table>
My tentative conclusions are:
<ul>
	<li>replacing my old "home server" with something ARM-based like a Raspberry Pi or a Pandaboard breaks even after one year -- I should do that.</li>
	<li>Even when under load, a ARM-based Pandaboard has a modest power consumption.</li>
	<li>I will completely turn off my TV on principle as the standby consumption is just pure impudence. As a bonus it prevents my BluRay player from kicking on the 100 Watt TV when I throw in a audio CD (Thanks Panasonic, for providing this excellent and "useful" integration).</li>
	<li>A cheap Netbook might be less powerful, but it hardly consumes less than a high-end Notebook when idling. You get what you pay for.</li>
	<li>I bought a <a href="http://www.coolermaster.com/product/Detail/mobile/laptops-cooling/notepal-u3.html">cooler</a> for my Notebook, hoping to unlock it from choking itself with thermal restriction. It should be a good idea in general as the logs not only talked about throttling, but also about more scary <a href="https://en.wikipedia.org/wiki/Machine_Check_Exception">MCEs</a>.</li>
	<li>Buying a wattmeter is a good decision, when you run nontrivial amounts of hardware.</li>
</ul>
Addendum: The 2.5 Watts for Bertha when off may seem bad -- but its not at all, if you consider it is running a <a href="https://en.wikipedia.org/wiki/Out-of-band_management">lights-out management</a> on that.
Originally published on 2013-08-20 17:06:18 on [wordpress](https://skyfromme.wordpress.com/2013/08/20/powerplay/).

+++
title="Following the White Rabbit"
date=2015-03-12
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "libreoffice", "performance", "ubuntu"]
+++
Following the White Rabbit
==========================

<p style="text-align:right;"><em>When logic and proportion have fallen sloppy dead</em></p>
<p style="text-align:right;">And the white knight is talking backwards</em></p>
<p style="text-align:right;">And the red queen's off with her head</em></p>
<p style="text-align:right;">Remember what the dormouse said</em></p>
<p style="text-align:right;">Feed your head, feed your head</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=WANNqr-vcx0">-- Jefferson Airplane, White Rabbit</a></em></p>

So, this was intended as a quick and smooth addendum to the <a href="https://skyfromme.wordpress.com/2015/03/02/50-ways-to-fill-your-vector/">"50 ways to fill your vector" post</a>, bringing <a href="http://valgrind.org/docs/manual/cl-manual.html">callgrind</a> into the game and ensuring everyone that its instructions counts are a good proxy for walltime performance of your code. This started out as mostly as expected, when measuring the instructions counts in two scenarios:
<table>
<tbody>
<tr>
<td>implementation/cflags</td>
<td><code>-O2</code> not inlined</td>
<td><code>-O3</code> inlined</td>
</tr>
<tr>
<td>A1</td>
<td>2610061438</td>
<td>2510061428</td>
</tr>
<tr>
<td>A2</td>
<td>2610000025</td>
<td>2510000015</td>
</tr>
<tr>
<td>A3</td>
<td>2610000025</td>
<td>2510000015</td>
</tr>
<tr>
<td>B1</td>
<td>3150000009</td>
<td>2440000009</td>
</tr>
<tr>
<td>B2</td>
<td>3150000009</td>
<td>2440000009</td>
</tr>
<tr>
<td>B3</td>
<td>3150000009</td>
<td>2440000009</td>
</tr>
<tr>
<td>C1</td>
<td>3150000009</td>
<td>2440000009</td>
</tr>
<tr>
<td>C3</td>
<td>3300000009</td>
<td>2440000009</td>
</tr>
</tbody>
</table>
The good news here is, that this mostly faithfully reproduces some general observations on the timings from the last post on this topic, although the differences in callgrind are more pronounced in callgrind than in reality:
<ul>
	<li>The A implementations are faster than the B and C implementations on <code>-O2</code> without inlining</li>
	<li>The A implementations are slower (by a smaller amount) than the B and C implementations on <code>-O3</code> with inlining</li>
</ul>
The last post also suggested the expectation that all implementations could -- and with a good compiler: should -- have the same code and same speed when everything is inline. Apart from the A implementations still differing from the B and C ones, callgrinds instruction count suggest to actually be the case. Letting gcc compile to assembler and comparing the output, one finds:
<ul>
	<li>Inline A1-3 compile to the same output on <code>-Os</code>, <code>-O2</code>, <code>-O3</code> each. There is no difference between <code>-O2</code> and <code>-O3</code> for these.</li>
	<li>Inline B1-3 compile to the same output on <code>-Os</code>, <code>-O2</code>, <code>-O3</code> each, but they differ between optimization levels.</li>
	<li>Inline C3 output differs from the others and between optimization levels.</li>
	<li>Without inlinable constructors, the picture is the same, except that A3 and B3 now differ slightly from their kin as expected.</li>
</ul>
So indeed most of the implementations generate the same assembler code. However, this is quite a bit at odd with the significant differences in performance measured in the last post, e.g. B1/B2/B3 on <code>-O2</code> created widely different walltimes. So time to test the assumption that running one implementation for a minute is producing reasonable statistically stable result, by doing 10 1-minute runs for each implementation and see what the standard deviation is. The following is found for walltimes (no inline constructors):
<table>
<tbody>
<tr>
<td>implementation/cflags</td>
<td><code>-Os</code></td>
<td><code>-O2</code></td>
<td><code>-O3</code></td>
<td><code>-O3 -march=</code></td>
</tr>
<tr>
<td>A1</td>
<td>80.6 s</td>
<td>78.9 s</td>
<td>78.9 s</td>
<td>79.0 s</td>
</tr>
<tr>
<td>A2</td>
<td>78.7 s</td>
<td>78.1 s</td>
<td>78.0 s</td>
<td>79.2 s</td>
</tr>
<tr>
<td>A3</td>
<td>80.7 s</td>
<td>78.9 s</td>
<td>78.9 s</td>
<td>78.9 s</td>
</tr>
<tr>
<td>B1</td>
<td>84.8 s</td>
<td>80.8 s</td>
<td>78.0 s</td>
<td>78.0 s</td>
</tr>
<tr>
<td>B2</td>
<td>84.8 s</td>
<td>86.0 s</td>
<td>78.0 s</td>
<td>78.1 s</td>
</tr>
<tr>
<td>B3</td>
<td>84.8 s</td>
<td>82.3 s</td>
<td>79.7 s</td>
<td>79.7 s</td>
</tr>
<tr>
<td>C1</td>
<td>84.4 s</td>
<td>85.4 s</td>
<td>78.0 s</td>
<td>78.0 s</td>
</tr>
<tr>
<td>C3</td>
<td>86.6 s</td>
<td>85.7 s</td>
<td>78.0 s</td>
<td>78.9 s</td>
</tr>
</tbody>
</table>

![no inline measurements](/img/wp/2015/03/noinline1.png)

And with inlining:
<table>
<tbody>
<tr>
<td>implementation/cflags</td>
<td><code>-Os</code></td>
<td><code>-O2</code></td>
<td><code>-O3</code></td>
<td><code>-O3 -march=</code></td>
</tr>
<tr>
<td>A1</td>
<td>76.4 s</td>
<td>74.5 s</td>
<td>74.7 s</td>
<td>73.8 s</td>
</tr>
<tr>
<td>A2</td>
<td>75.4 s</td>
<td>73.7 s</td>
<td>73.8 s</td>
<td>74.5 s</td>
</tr>
<tr>
<td>A3</td>
<td>76.3 s</td>
<td>74.6 s</td>
<td>75.5 s</td>
<td>73.7 s</td>
</tr>
<tr>
<td>B1</td>
<td>80.6 s</td>
<td>77.1 s</td>
<td>72.7 s</td>
<td>73.7 s</td>
</tr>
<tr>
<td>B2</td>
<td>81.4 s</td>
<td>78.9 s</td>
<td>72.0 s</td>
<td>72.0 s</td>
</tr>
<tr>
<td>B3</td>
<td>80.6 s</td>
<td>78.9 s</td>
<td>72.8 s</td>
<td>73.7 s</td>
</tr>
<tr>
<td>C1</td>
<td>81.4 s</td>
<td>78.9 s</td>
<td>72.0 s</td>
<td>72.0 s</td>
</tr>
<tr>
<td>C3</td>
<td>79.7 s</td>
<td>80.5 s</td>
<td>72.9 s</td>
<td>77.8 s</td>
</tr>
</tbody>
</table>

![inline measurements](/img/wp/2015/03/inline1.png)

The standard deviation for all the above values is less than 0.2 seconds. That is ... interesting: For example, on <code>-O2</code> without inlining, B1 and B2 generate the same assembler output, but execute with a very significant difference in hardware (5.2 s difference, or more than 25 standard deviations). So how have logic and proportion fallen sloppy dead here? If the same code is executed -- admittedly from two different locations in the binary -- how can that create such a significant difference in walltime performance, while not being visible at all on callgrind? A wild guess, which I have not confirmed yet, is cache locality: When not inlining constructors, those might be in CPU cache from one copy of the code in the binary, but not for the other. And by the way, it might also hint at the reasons for the <code>-march=</code> flag (which creates bigger code) seeming so uneffective. And it might explain, why performance is rather consistent when using inline constructors. If so, the impact of this is certainly interesting. It also suggest that allowing inlining of hotspots, like <a href="https://skyfromme.wordpress.com/2015/01/15/swnodeindex-ludicious-speed/">recently done with the low-level <code>sw::Ring</code> class</a>, produces much more performance improvement on real hardware than the meager results measured with callgrind. And it reinforces the warning made in that post about not falling in the trap of mistaking the map for the territory: callgrind is not a <a href="https://en.wikipedia.org/wiki/Map%E2%80%93territory_relation">"map in the scale of a mile to the mile"</a>.

Addendum: As said in the previous post, I am still interested in such measurements on other hardware or compilers. All measurements above done with gcc 4.8.3 on Intel i5-4200U@1.6GHz.

Originally published on 2015-03-12 10:44:58 on [wordpress](https://skyfromme.wordpress.com/2015/03/12/following-the-white-rabbit/).

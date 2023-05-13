+++
title="SwNodeIndex: Ludicious Speed"
date=2015-01-15
[taxonomies]
originally-published-on=["wordpress"]
categories=["libreoffice", "ubuntu"]
+++
SwNodeIndex: Ludicious Speed
============================

<p style="text-align:right;"><em>No-no-no, light speed is too slow!<br>Yes, we'll have to go right to... ludicrous speed!</em>
<p style="text-align:right;"><a href="https://www.youtube.com/watch?v=ygE01sOhzz0"><em>-- Dark Helmet, Spaceballs</em></a></p>

So, I recently brought up the topic of writers notes in the <a href="http://nabble.documentfoundation.org/minutes-of-ESC-call-td4132277.html">LibreOffice ESC call</a>. More specifically: the <code>SwNodeIndex</code> class, which is, if one broadly simplifies an iterator over the <a href="http://docs.libreoffice.org/sw/html/classSwNodes.html">container holding all the paragraphs</a> of a text document. Before my modifications, the <code>SwNodes</code> container class had all these SwNodeIndices in a homegrown intrustive double linked list, to be able to ensure these stay valid e.g. if a <a href="http://docs.libreoffice.org/sw/html/classSwNode.html"><code>SwNode</code></a> gets deleted/removed. Still -- as usual with performance topics -- wild guesses arent helpful, and measurements should trump over intuition. I used valgrind for that, and measured the number of instructions needed for loading the ODF spec. Since I did the same years and years ago on the <a href="https://wiki.openoffice.org/wiki/Performance/WriterInProgress">old OpenOffice.org performance project</a>, I just checked if we regressed against that. Its comforting that we did not at all -- we were much faster, but that measurement has to be taken with a few pounds of salt, as a lot of other things differ between these two measurements (e.g. we now have a completely new build system, compiler versions etc.). But its good we are moving in the right direction.

<table>
<tr>
<td>implementation</td>
<td>SwNodes </td>
<td>SwNodeIndex</td>
<td>total instructions</td>
<td>performance</td>
<td>linedelta</td>
<tr />
<tr>
<td>DEV300_m45</td>
<td>71,727,655</td>
<td>73,784,052</td>
<td>9,823,158,471</td>
<td>?</td>
<td>?</td>
</tr>
<tr>
<td><a href="http://cgit.freedesktop.org/libreoffice/core/commit/?id=fc93c17a">master@fc93c17a</a></td>
<td>84,553,232</td>
<td>60,987,760</td>
<td>6,170,762,825</td>
<td>0%</td>
<td>0</td>
</tr>
<tr>
<td>std::list</td>
<td>18,461,317</td>
<td>103,461,317</td>
<td>14,502,230,571</td>
<td>-5,725%<br>(-235% of total)</td>
<td>+12/-70</td>
</tr>
<tr>
<td>std::vector</td>
<td>18,986,848</td>
<td>3,707,286,032</td>
<td>9,811,541,380</td>
<td>-2,502%</td>
<td>+22/-70</td>
</tr>
<tr>
<td>std::unordered_map</td>
<td>18,984,984</td>
<td>82,843,000</td>
<td>7,083,620,244</td>
<td>-627%<br>(-15% of total)</td>
<td>+16/-70</td>
</tr>
<tr>
<td>std::vector rbegin</td>
<td>18,986,848</td>
<td>143,851,229</td>
<td>6,214,602,532</td>
<td>-30%<br>(-7% of total)</td>
<td>+23/-70</td>
</tr>
<tr>
<td>sw::Ring&lt;&gt;</td>
<td>23,447,256</td>
<td>inlined</td>
<td>6,154,660,709</td>
<td>11%<br>(2.6% of total)</td>
<td>+108/-229</td>
</tr>
<table>

<p>With that comforting knowledge, I started to play around with the code. The first thing I did was to replace the handcrafted intrusive list with a <code>std::list</code> pointing to the <code>SwNodeIndex</code> instances as a member in the <code>SwNodes</code> class. This is expected to slow down things, as now two allocs are needed: one for the <code>SwNodeIndex</code> class and one for the node entry in the <code>std::list</code>. To be honest though, I didnt expect this to slow down the code handling the nodes by a factor of ~57 for the loading of the example document. This whole document loading time (not just the node handling) slows by a factor of ~2.4. So ok, this establishes for certain that this part of the code is highly performance sensitive.</p>

<p>The next thing I tried to get a feel for how the performance reacts was using a <code>std::vector</code> in the <code>SwNodes</code> class. When reserving some memory early, this should severely reduce the amount of allocs needed. And indeed this was quicker than the <code>std::list</code> even with a naive approach just doing a <code>push_back()</code> for insertion and a <code>std::find()</code>/<code>std::erase()</code> for removal. However, the node indices are often temporarily created and quickly destroyed again. Thus adding new indices at the end and searching from the start certainly is not ideal: Thus this is also slower than the intrusive list that was on master by a factor of ~25 for the code doing the node handling.</p>

<p>Searching for a <code>SwNodeIndex</code> from the end of the vector, where we likely just inserted it and then swapping it with the last entry makes the <code>std::vector</code> almost compatitive with the original implementation: but still 30% slower than the original implementation. (The total loading time would only have increased by 0.7% using the vector like this.)</p>

<p>For completeness, I also had a look at a <code>std::unordered_map</code>. It did a bit better than I expected, but still would have slowed down loading by 15% for the example experiment.</p>

<p>Having ruled out that standard containers would do much good here without lots of tweaking, I tried the <code>sw::Ring&lt;&gt;</code> class that I recently rewrote based on <a href="http://www.boost.org/doc/libs/1_55_0/doc/html/intrusive.html">Boost.Intrusive</a> as a inline header class. This was 11% quicker than the old implementation, resulting in 2.6% quicker loading for the whole document. Not exactly a heroic archivement, but also not too bad for just some 200 lines touched. So this is now on <a href="http://cgit.freedesktop.org/libreoffice/core/tree/sw/inc/ndindex.hxx#n35">master</a>.</p>

<p>Why do this linked list outperform the old linked list? Inlining. Especially, the non-inlined <a href="http://cgit.freedesktop.org/libreoffice/core/tree/sw/source/core/docnode/ndindex.cxx?id=fc93c17a#n44">constructors</a> and the destructor calling a trivial non-inlined <a href="http://cgit.freedesktop.org/libreoffice/core/tree/sw/source/core/docnode/ndindex.cxx?id=fc93c17a#n72">member function</a>. And on top of that, the contructors and the function called by the destructor called two non-inlined <a href="http://cgit.freedesktop.org/libreoffice/core/tree/sw/source/core/docnode/nodes.cxx?id=fc93c17a#n2291">friend</a> <a href="http://cgit.freedesktop.org/libreoffice/core/tree/sw/source/core/docnode/nodes.cxx?id=fc93c17a#n2310">functions</a> from a different compilation unit, making it extra hard for a compiler to optimize that. Now, <a href="https://gcc.gnu.org/wiki/LinkTimeOptimization">link time optimization (LTO)</a> could maybe do something about that someday. However, with LTO being in different states on different platforms and with developers possibly building without LTO for build time performance for some time, requiring the compiler/linker to be extra clever might be a mixed blessing: The developers might run into <a href="https://en.wikipedia.org/wiki/Map%E2%80%93territory_relation">"the map is not the territory"</a> problems.</p>

<p>my personal take-aways:</p>
<ul>
<li>The <code>SwNodeIndex</code> has quite a relevant impact on performance. If you touch it, handle with care (and with <code>valgrind</code>).</li>
<li>The current code has decent performance, further improvement likely need deeper structual work (see e.g. <a href="http://cgit.freedesktop.org/libreoffice/core/log/?h=feature/bplustree">Kendys bplustree stuff</a>).</li>
<li>Intrusive linked lists might be cumbersome, but for some scenarios, they are really fast.</li>
<li>Inlining can really help (doh).</li>
<li>LTO might help someday (or not).</li>
<li>friend declarations for non-inline functions across compilation units can be a code smell for possible performance optimization.</li>
</ul>

Please excuse the extensive writing for a meager 2.6% performance improvement -- the intention is to avoid somebody (including me) to redo some or all of the work above just to come to the same conclusion.

<small>
<strong>Note:</strong> Here is how this was measured:
<ul>
<li>gcc 4.8.3</li>
<li>boost 1.55.0</li>
<li>test document: <a href="http://docs.oasis-open.org/office/v1.1/OS/OpenDocument-v1.1.odt">ODF spec</a>
<li><code>valgrind --tool=callgrind "--toggle-collect=*LoadOwnFormat*" --callgrind-out-file=somefilename.cg ./instdir/program/soffice.bin</code></li>
<li><code>./autogen.sh --disable-gnome-vfs --disable-odk --disable-postgresql-sdbc --disable-report-builder --disable-scripting-beanshell --enable-gio --enable-symbols --with-external-tar=... --with-junit=... --with-hamcrest=... --with-system-libs --without-doxygen --without-help --without-myspell-dicts --without-system-libmwaw --without-system-mdds --without-system-orcus --without-system-sane --without-system-vigra --without-system-libodfgen --without-system-libcmis --disable-firebird-sdbc --without-system-libebook --without-system-libetonyek --without-system-libfreehand --without-system-libabw --disable-gnome-vfs --without-system-glm --without-system-glew --without-system-librevenge --without-system-libcdr --without-system-libmspub --without-system-libvisio --without-system-libwpd --without-system-libwps --without-system-libwpg --without-system-libgltf --without-system-libpagemaker --without-system-coinmp --with-jdk-home=...</code></li>
</ul><small>

Originally published on 2015-01-15 12:14:34 at https://skyfromme.wordpress.com/2015/01/15/swnodeindex-ludicious-speed/.

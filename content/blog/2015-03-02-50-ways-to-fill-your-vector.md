+++
title="50 ways to fill your vector ..."
date=2015-03-02
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "libreoffice", "performance", "ubuntu"]
+++
50 ways to fill your vector ...
===============================

<p style="text-align:right;"><i>"The problem is all inside your head" she said to me
"The answer is easy if you take it logically"
<a href="https://www.youtube.com/watch?v=298nld4Yfds">— Paul Simon, 50 ways to leave your lover</a></i></p>
So recently I tweaked around with these newfangled C++11 initializer lists and created an EasyHack to use them to <a href="https://bugs.documentfoundation.org/show_bug.cgi?id=89592">initialize property sequences in a readable way</a>. This caused a <a href="http://nabble.documentfoundation.org/Re-Libreoffice-commits-use-init-lists-for-property-sequences-td4141205.html">short exchange on the LibreOffice mailing list</a>, which I assumed had its part in motivating Stephans interesting post <a href="https://whatofhow.wordpress.com/2015/02/25/on-filling-a-vector/">"On filling a vector"</a>. For all the points being made (also in the quick follow up on <a href="irc://chat.freenode.net/libreoffice-dev">IRC</a>), I wondered how much the theoretical "can use a move constructor" discussed etc. really meant when the C++ is translated to e.g. <a href="https://gcc.gnu.org/onlinedocs/gccint/GENERIC.html#GENERIC">GENERIC</a>, then <a href="https://gcc.gnu.org/onlinedocs/gccint/GIMPLE.html#GIMPLE">GIMPLE</a>, then <a href="https://en.wikipedia.org/wiki/X86_assembly_language">amd64 assembler</a>, then to the <a href="http://sunnyeves.blogspot.de/2009/07/intel-x86-processors-cisc-or-risc-or.html">internal RISC instructions of the CPU </a>-- with multiple levels of caching in addition.

So I quickly wrote the following (thanks so much for C++11 having the nice <code>std::chrono</code> now).

data.hxx:
<code>
#include &lt;vector&gt;
struct Data {
&nbsp;&nbsp;&nbsp;&nbsp;Data();
&nbsp;&nbsp;&nbsp;&nbsp;Data(int a);
&nbsp;&nbsp;&nbsp;&nbsp;int m_a;
};
void DoSomething(std::vector&lt;Data&gt;&amp;);
</code>

data.cxx:
<code>
#include "data.hxx"
// noop in different compilation unit to prevent optimizing out what we want to measure
void DoSomething(std::vector&lt;Data&gt;&amp;) {};
Data::Data() : m_a(4711) {};
Data::Data(int a) : m_a(a+4711) {};
</code>

main.cxx:
<code>
#include "data.hxx"
#include &lt;iostream&gt;
#include &lt;vector&gt;
#include &lt;chrono&gt;
#include &lt;functional&gt;

void A1(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec { Data(), Data(), Data() };
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void A2(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec { {}, {}, {} };
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void A3(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec { 0, 0, 0 };
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void B1(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.reserve(3);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void B2(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.reserve(3);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back({});
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back({});
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back({});
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void B3(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.reserve(3);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.push_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void C1(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.reserve(3);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(Data());
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

void C3(long count) {
&nbsp;&nbsp;&nbsp;&nbsp;while(--count) {
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;std::vector&lt;Data&gt; vec;
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.reserve(3);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;vec.emplace_back(0);
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;DoSomething(vec);
&nbsp;&nbsp;&nbsp;&nbsp;}
}

double benchmark(const char* name, std::function&lt;void (long)&gt; testfunc, const long count) {
&nbsp;&nbsp;&nbsp;&nbsp;const auto start = std::chrono::system_clock::now();
&nbsp;&nbsp;&nbsp;&nbsp;testfunc(count);
&nbsp;&nbsp;&nbsp;&nbsp;const auto end = std::chrono::system_clock::now();
&nbsp;&nbsp;&nbsp;&nbsp;const std::chrono::duration&lt;double&gt; delta = end-start;
&nbsp;&nbsp;&nbsp;&nbsp;std::cout &lt;&lt; count &lt;&lt; " " &lt;&lt; name &lt;&lt; " iterations took " &lt;&lt; delta.count() &lt;&lt; " seconds." &lt;&lt; std::endl;
&nbsp;&nbsp;&nbsp;&nbsp;return delta.count();
}

int main(int, char**) {
&nbsp;&nbsp;&nbsp;&nbsp;long count = 10000000;
&nbsp;&nbsp;&nbsp;&nbsp;while(benchmark("A1", &amp;A1, count) &lt; 60l)
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;count &lt;&lt;= 1;
&nbsp;&nbsp;&nbsp;&nbsp;std::cout &lt;&lt; &quot;Going with &quot; &lt;&lt; count &lt;&lt; &quot; iterations.&quot; &lt;&lt; std::endl;
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;A1&quot;, &amp;A1, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;A2&quot;, &amp;A2, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;A3&quot;, &amp;A3, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;B1&quot;, &amp;B1, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;B2&quot;, &amp;B2, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;B3&quot;, &amp;B3, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;C1&quot;, &amp;C1, count);
&nbsp;&nbsp;&nbsp;&nbsp;benchmark(&quot;C3&quot;, &amp;C3, count);
&nbsp;&nbsp;&nbsp;&nbsp;return 0;
}
</code>

Makefile:
<code>
CFLAGS?=-O2
main: main.o data.o
&nbsp;&nbsp;&nbsp;&nbsp;g++ -o $@ $^

%.o: %.cxx data.hxx
&nbsp;&nbsp;&nbsp;&nbsp;g++ $(CFLAGS) -std=c++11 -o $@ -c $&lt;
</code>

Note the object here is small and trivial to copy as one would expect from objects passed around as values (as expensive to copy objects mostly can be passed around with a <code>std::shared_ptr</code>). So what did this measure? Here are the results:

Time for 1280000000 iterations on a Intel i5-4200U@1.6GHz (<code>-march=core-avx2</code>) compiled with gcc 4.8.3 without inline constructors:
<table>
<tbody>
<tr>
<td><strong>implementation / CFLAGS</strong></td>
<td><strong>-Os</strong></td>
<td><strong>-O2</strong></td>
<td><strong>-O3</strong></td>
<td><strong>-O3 -march=...</strong></td>
</tr>
<tr>
<td><strong>A1</strong></td>
<td><strong>89.1 s</strong></td>
<td><strong>79.0 s</strong></td>
<td><strong>78.9 s</strong></td>
<td>78.9 s</td>
</tr>
<tr>
<td><strong>A2</strong></td>
<td><strong>89.1 s</strong></td>
<td><strong>78.1 s</strong></td>
<td><strong>78.0 s</strong></td>
<td>80.5 s</td>
</tr>
<tr>
<td><strong>A3</strong></td>
<td><strong>90.0 s</strong></td>
<td><strong>78.9 s</strong></td>
<td><strong>78.8 s</strong></td>
<td>79.3 s</td>
</tr>
<tr>
<td><strong>B1</strong></td>
<td>103.6 s</td>
<td>97.8 s</td>
<td><strong>79.0 s</strong></td>
<td>78.0 s</td>
</tr>
<tr>
<td><strong>B2</strong></td>
<td>99.4 s</td>
<td>95.6 s</td>
<td><strong>78.5 s</strong></td>
<td>78.0 s</td>
</tr>
<tr>
<td><strong>B3</strong></td>
<td>107.4 s</td>
<td>90.9 s</td>
<td>79.7 s</td>
<td>79.9 s</td>
</tr>
<tr>
<td><strong>C1</strong></td>
<td>99.4 s</td>
<td>94.4 s</td>
<td><strong>78.0 s</strong></td>
<td>77.9 s</td>
</tr>
<tr>
<td><strong>C3</strong></td>
<td>98.9 s</td>
<td>100.7 s</td>
<td><strong>78.1 s</strong></td>
<td>81.7 s</td>
</tr>
</tbody>
</table>
<a href="/img/wp/2015/03/noinline.png"><img class="aligncenter size-large wp-image-979" src="/img/wp/2015/03/noinline.png?w=660" alt="creating a three element vector without inlined constructors" width="660" height="450" /></a>
And, for comparison, following are the results, if one allows the constructors to be inlined.
Time for 1280000000 iterations on a Intel i5-4200U@1.6GHz (<code>-march=core-avx2</code>) compiled with gcc 4.8.3 with inline constructors:
<table>
<tbody>
<tr>
<td>implementation / CFLAGS</td>
<td>-Os</td>
<td>-O2</td>
<td>-O3</td>
<td>-O3 -march=...</td>
</tr>
<tr>
<td>A1</td>
<td><strong>85.6 s</strong></td>
<td><strong>74.7 s</strong></td>
<td>74.6 s</td>
<td>74.6 s</td>
</tr>
<tr>
<td>A2</td>
<td><strong>85.3 s</strong></td>
<td><strong>74.6 s</strong></td>
<td>73.7 s</td>
<td>74.5 s</td>
</tr>
<tr>
<td>A3</td>
<td>91.6 s</td>
<td><strong>73.8 s</strong></td>
<td>74.4 s</td>
<td>74.5 s</td>
</tr>
<tr>
<td>B1</td>
<td>93.4 s</td>
<td>90.2 s</td>
<td><strong>72.8 s</strong></td>
<td>72.0 s</td>
</tr>
<tr>
<td>B2</td>
<td>93.7 s</td>
<td>88.3 s</td>
<td><strong>72.0 s</strong></td>
<td>73.7 s</td>
</tr>
<tr>
<td>B3</td>
<td>97.6 s</td>
<td>88.3 s</td>
<td><strong>72.8 s</strong></td>
<td>72.0 s</td>
</tr>
<tr>
<td>C1</td>
<td>93.4 s</td>
<td>88.3 s</td>
<td><strong>72.0 s</strong></td>
<td>73.7 s</td>
</tr>
<tr>
<td>C3</td>
<td>96.2 s</td>
<td>88.3 s</td>
<td><strong>71.9 s</strong></td>
<td>73.7 s</td>
</tr>
</tbody>
</table>
<a href="/img/wp/2015/03/inlinefixed.png"><img src="/img/wp/2015/03/inlinefixed.png?w=660" alt="creating a three element vector without inlined constructors" width="660" height="450" class="aligncenter size-large wp-image-1001" /></a>
Some observations on these measurements:
<ul>
	<li><code>-march=...</code> is at best neutral: The measured times do not change much in general, they only even slightly improve performance in five out of 16 cases, and the two cases with the most significant change in performance (over 3%) are actually hurting the performance. So for the rest of this post, <code>-march=...</code> will be ignored. Sorry gentooers. ;)</li>
	<li>There is no silver bullet with regard to the different implementations: A1, A2 and A3 are the faster implementations when not inlining constructors and using <code>-Os</code> or <code>-O2</code> (the quickest A* is ~10% faster than the quickest B*/C*). However when inlining constructors and using -O3, the same implementations are the slowest (by 2.4%).</li>
	<li>Most common release builds are still done with <code>-O2</code> these days. For those, using initializer lists (A1/A2/A3) seem too have a significant edge over the alternatives, whether constructors are inlined or not. This is in contrast to the conclusions made from <a href="https://whatofhow.wordpress.com/2015/02/25/on-filling-a-vector/">"constructor counting"</a>, which assumed these to be slow because of additional calls needed.</li>
	<li>The numbers printed in bold are either the quickest implementation in a build scenario or one that is within 1.5% of the quickest implementation. A1 and A2 are sharing the title here by being in that group five times each.</li>
	<li>With constructors inlined, everything in the loop except <code>DoSomething()</code> could be inline. It seems to me that the compiler could -- at least in theory -- figure out that it is asked the same thing in all cases. Namely, reserve space for three ints on the heap, fill them each with 4711 and make the <code>::std::vector&lt;int&gt;</code> data structure on the stack reflect that, then hand that to the <code>DoSomething()</code> function that you know nothing about. If the compiler would figure that out, it would take the same time for all implementations. This doesnt happen either on <code>-O2</code> (differ by ~18% from quickest to slowest) nor on <code>-O3</code> (differ by ~3.6%).</li>
</ul>
One common mantra in applications development is "trust the compiler to optimize". The above observations show a few cracks in the foundations of that, esp. if you take into account that this is all on the same version of the same compiler running on the same platform and hardware with the same STL implementation. For huge objects with expensive constructors, the constructor counting approach might still be valid. Then again, those are rarely statically initialized as a bigger bunch into a vector. For the more common scenario of smaller objects with cheap constructors, my tentative conclusion so far would be to go with A1/A2/A3 -- not so much because they are quickest in the most common build scenarios on my platform, but rather because the readability of them is a value on its own while the performance picture is muddy at best.

And hey, if you want to run the tests above on other platforms or compilers, I would be interested in results!

Note: I did these runs for each scenario only once, thus no standard deviation is given. In general, they seemed to be rather stable, but this being wallclock measurements, one or the other might be outliers. caveat emptor.

Originally published on 2015-03-02 22:47:07 on [wordpress](https://skyfromme.wordpress.com/2015/03/02/50-ways-to-fill-your-vector/).

+++
title="Blast from the Past: the Pimpl?"
date=2015-07-18
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "c++11", "designpattern", "libreoffice", "pimpl", "ubuntu"]
+++
Blast from the Past: the Pimpl?
===============================

<p style="text-align:right;"><em>They sentenced me to twenty years of boredom
For trying to change the system from within
<a href="https://www.youtube.com/watch?v=JTTC_fD598A">-- Leonard Cohen, I'm your man, First we take Manhattan</a></em></p>
<p style="text-align:left;"><em><strong>Advance warning:</strong></em> This blog post talks about C++ coding style, and given the "expressiveness" (aka a severe infection with <a href="http://c2.com/cgi/wiki?TimTowTdi">TimTowTdi</a>) this is bound to contain significant amounts of bikeshedding, personal opinion/preference. As such, be invited to ignore all this as the ramblings of a raging lunatic.</p>
<p style="text-align:left;">Anyone who observed me spotting <a href="http://c2.com/cgi/wiki?PimplIdiom">a Pimpl </a>in code will know that I am not a fan of this idom. Its intend is to reduce build times by using a design pattern to move implementation details out of headers -- a workaround for C++s misfeature of by default needing a recompile even for changing implementation details only without changing the public interface. Now I personally always thought a <a href="http://www.cprogramming.com/tips/tip/in-cplusplus-a-pure-abstract-base-class-is-a-contract">pure abstract base class</a> to be a more "native" and less ugly way to tell this to the compiler. However, without real testing, such gut feelings are rarely good advisors in a complex language like C++.</p>
<p style="text-align:left;">So I did some testing on the real life performance of a pure abstract base class vs. a pimpl (each of course in a different compilation unit to prevent the compiler to optimize away what we want to measure) -- and for reference, a class with functions that can be completely inlined. These are the three test implementations, inline:</p>

<pre style="font-family:monospace;">
-- header (hxx) --
class InlineClass final
{
	public:
		InlineClass(int nFirst, int nSecond)
			: m_nFirst(nFirst), m_nSecond(nSecond), m_nResult(0)
		{};
		void Add()
			{ m_nResult = m_nFirst + m_nSecond; };
		int GetResult() const
			{ return m_nResult; };
	private:
		const int m_nFirst;
		const int m_nSecond;
		int m_nResult;
};
</pre>
<p style="text-align:left;">Pimpl, as suggested by <a href="http://scottmeyers.blogspot.de/2014/07/draft-version-of-effective-modern-c-now.html">Effective Modern C++</a> when using C++11, but not C++14:</p>

<pre style="font-family:monospace;">
-- header (hxx) --
#include &lt;memory&gt;
class PimplClass final
{
	public:
		PimplClass(int nFirst, int nSecond);
		~PimplClass();
		void Add();
		int GetResult() const;
	private:
		struct Impl;
		std::unique_ptr&lt;Impl&gt; m_pImpl;
};
-- implementation (cxx) --
#include "pimpl.hxx"
struct PimplClass::Impl
{
	Impl(int nFirst, int nSecond)
		: m_nFirst(nFirst), m_nSecond(nSecond), m_nResult(0)
	{};
	const int m_nFirst;
	const int m_nSecond;
	int m_nResult;
};
PimplClass::PimplClass(int nFirst, int nSecond)
	: m_pImpl(std::unique_ptr&lt;Impl&gt;(new Impl(nFirst, nSecond)))
{}
PimplClass::~PimplClass()
	{}
void PimplClass::Add()
	{ m_pImpl-&gt;m_nResult = m_pImpl-&gt;m_nFirst + m_pImpl-&gt;m_nSecond; }
int PimplClass::GetResult() const
	{ return m_pImpl-&gt;m_nResult; }
</pre>
<p style="text-align:left;">Pure abstract base class:</p>

<pre style="font-family:monospace;">
-- header (hxx) --
#include &lt;memory&gt;
struct AbcClass
{
	static std::shared_ptr&lt;AbcClass&gt; Create(int nFirst, int nSecond);
	virtual ~AbcClass() {};
	virtual void Add() =0;
	virtual int GetResult() const =0;
};
-- implementation (cxx) --
#include "abc.hxx"
#include &lt;memory&gt;
struct AbcClassImpl final : public AbcClass
{
	AbcClassImpl(int nFirst, int nSecond)
		: m_nFirst(nFirst), m_nSecond(nSecond)
	{};
	virtual void Add() override
		{ m_nResult = m_nFirst + m_nSecond; };
	virtual int GetResult() const override
		{ return m_nResult; };
	const int m_nFirst;
	const int m_nSecond;
	int m_nResult;
};
std::shared_ptr&lt;AbcClass&gt; AbcClass::Create(int nFirst, int nSecond)
	{ return std::shared_ptr&lt;AbcClass&gt;(new AbcClassImpl(nFirst, nSecond)); }
</pre>
<p style="text-align:left;">Comparing these we find:</p>

<table>
<tbody>
<tr>
<th>implementation</th>
<th>lines added for GetResult()</th>
<th>source entropy</th>
<th>added source entropy for GetResult()</th>
<th>runtime</th>
</tr>
<tr>
<td>inline</td>
<td style="text-align:center;">2</td>
<td style="text-align:center;">187</td>
<td style="text-align:center;">17</td>
<td style="text-align:center;">100%</td>
</tr>
<tr>
<td>Pimpl</td>
<td style="text-align:center;">3</td>
<td style="text-align:center;">316</td>
<td style="text-align:center;">26</td>
<td style="text-align:center;">168% (174%)</td>
</tr>
<tr>
<td>pure ABC</td>
<td style="text-align:center;">3</td>
<td style="text-align:center;">295 (273)</td>
<td style="text-align:center;">19 (16)</td>
<td style="text-align:center;">158%</td>
</tr>
</tbody>
</table>
So the abstract base class has less complex source code (entropy)<sup>1</sup>, needs less additional entropy to expand and <strong>is still faster</strong> in the end on common hardware (Intel i5-4200U) with common compiler optimization switches (<code>-O2</code>)<sup>2</sup>.

Additionally, in a non-trivial code base you might actually need to use virtual functions for your implementation anyway as you are deriving from or implementing an existing interface. In the Pimpl case, this means using <em>two indirections</em> (resolving the virtual function and then resolving the <code>m_pImpl</code> pointer in that function on top of that). In the abstract base class case thats not happening and in addition, it means that you can spare yourself the pure virtual declarations in the <code>*.hxx</code> (the <code>virtual ... =0</code> ones), as those are already declared in the class derived from. In LibreOffice, this is true for any class implementing UNO interfaces. So the first numbers are actually biased against an abstract base class for real world code bases -- the numbers in parathesis show the results when an interface is already defined elsewhere.

So unless the synthetic example used here is some kind of weird cornercase, this suggests abstract base classes being the better alternative over a Pimpl once the class goes beyond being a plain value type with completely inlineable accessor member functions.

Thanks for bearing with me on this rant about one of my personal pet peeves here!

<sup>1</sup> entropy is measured as <code>cat abc.[hc]xx|gzip|wc -c</code> or <code> cat pimpl.[hc]xx|sed -e 's/Pimpl/Abc/g'|gzip|wc -c</code>.
<sup>2</sup> Here is the code run for that comparision:
<pre style="font-family:monospace;">
constexpr int repeats = 100000;

int pimplrun(long count)
//int abcrun(long count)
{
        std::vector&lt; std::shared_ptr&lt;PimplClass /* AbcClass */ &gt; &gt; vInstances;
        vInstances.reserve(count);
        while(--count)
                vInstances.emplace_back(std::make_shared&lt;PimplClass&gt;(4711, 4711));
                //vInstances.emplace_back(AbcClass::Create(4711, 4711));
        int result(0);
        count = vInstances.size();
        while(--count)
                for(auto pInstance : vInstances)
                {
                        pInstance-&gt;Add();
                        result += pInstance-&gt;GetResult();
                }
        return result;
}

</pre>
Instances are stored in shared pointers as anything that a Pimpl is considered for would be "heavy" enough to be handled by reference instead of by value.

<strong>Update 1:</strong> Out of curiosity, I looked a bit deeper at this with callgrind. This is what I found for running the above (with 1000 repeats) and <code>--cache-sim=yes</code>:
<code>
I1 cache: 32768 B, 64 B, 8-way
D1 cache: 32768 B, 64 B, 8-way
LL cache: 3145728 B, 64 B, 12-way
</code>
<table>
<tbody>
<tr>
<th style="text-align:left;">event</th>
<th style="text-align:right;">inline</th>
<th style="text-align:right;">ABC</th>
<th style="text-align:right;">Pimpl</th>
</tr>
<tr>
<td>Ir</td>
<td style="text-align:right;">23,356,163</td>
<td style="text-align:right;">38,652,092</td>
<td style="text-align:right;">38,620,878</td>
</tr>
<tr>
<td>Dr</td>
<td style="text-align:right;">5,066,041</td>
<td style="text-align:right;">14,109,098</td>
<td style="text-align:right;">12,107,992</td>
</tr>
<tr>
<td>Dw</td>
<td style="text-align:right;">3,060,033</td>
<td style="text-align:right;">5,094,790</td>
<td style="text-align:right;">5,099,991</td>
</tr>
<tr>
<td>I1ir</td>
<td style="text-align:right;">34</td>
<td style="text-align:right;">127</td>
<td style="text-align:right;">29</td>
</tr>
<tr>
<td>D1mr</td>
<td style="text-align:right;">499,952</td>
<td style="text-align:right;">253,006</td>
<td style="text-align:right;"><strong>999,013</strong></td>
</tr>
<tr>
<td>D1mw</td>
<td style="text-align:right;">501,636</td>
<td style="text-align:right;">998,312</td>
<td style="text-align:right;">500,097</td>
</tr>
<tr>
<td>ILmr</td>
<td style="text-align:right;">28</td>
<td style="text-align:right;">126</td>
<td style="text-align:right;">24</td>
</tr>
<tr>
<td>DLmr</td>
<td style="text-align:right;">2</td>
<td style="text-align:right;">845</td>
<td style="text-align:right;">0</td>
</tr>
<tr>
<td>DLmw</td>
<td style="text-align:right;">0</td>
<td style="text-align:right;">1,285</td>
<td style="text-align:right;">250</td>
</tr>
</tbody>
</table>
I dont know exactly what to derive from that, but what is clear is that purely by instruction counts <code>Ir</code> this can not be explained. So you need <code>--cache-sim=yes</code> which gives the additional event counts. Actually Pimpl looks slightly better on most stats, so as it is slower in real life, the cache misses on the first level data cache <code>D1mr</code> might have quite an impact?

<strong>Update 2:</strong> This post made it to <a href="https://www.reddit.com/r/cpp/comments/3druwq/blast_from_the_past_the_pimpl/">reddit</a>, so I looked into some of the feedback from there. A common suggestion was to use <code>for(auto&amp; pInstance : vInstances)</code> instead of <code>for(auto pInstance : vInstances)</code> in the benchmarking function. This had no significant impact on walltime measurements nor made it callgrind event counts show some clearer picture. I also played around with the order of linked objects to see if it has any impact (via cache locality etc.). While runtime measurements fluctuated quite a bit (even when using the same binary), the order was always the same: inlining quickest, then abstract base class and pimpl slowest.
Originally published on 2015-07-18 07:16:23 at https://skyfromme.wordpress.com/2015/07/18/blast-from-the-past-the-pimpl/.

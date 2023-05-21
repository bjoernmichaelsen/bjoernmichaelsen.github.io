+++
title="50 ways to fill your vector ..."
date=2015-03-02
[taxonomies]
originally-published-on=["wordpress"]
categories=["c++", "libreoffice", "performance", "ubuntu"]
+++
50 ways to fill your vector ...
===============================

<p style="text-align:right;"><em>"The problem is all inside your head" she said to me</em></p>
<p style="text-align:right;"><em>"The answer is easy if you take it logically"</em></p>
<p style="text-align:right;"><em><a href="https://www.youtube.com/watch?v=298nld4Yfds">— Paul Simon, 50 ways to leave your lover</a></em></p>

So recently I tweaked around with these newfangled C++11 initializer lists and created an EasyHack to use them to <a href="https://bugs.documentfoundation.org/show_bug.cgi?id=89592">initialize property sequences in a readable way</a>. This caused a <a href="http://nabble.documentfoundation.org/Re-Libreoffice-commits-use-init-lists-for-property-sequences-td4141205.html">short exchange on the LibreOffice mailing list</a>, which I assumed had its part in motivating Stephans interesting post <a href="https://whatofhow.wordpress.com/2015/02/25/on-filling-a-vector/">"On filling a vector"</a>. For all the points being made (also in the quick follow up on <a href="irc://chat.freenode.net/libreoffice-dev">IRC</a>), I wondered how much the theoretical "can use a move constructor" discussed etc. really meant when the C++ is translated to e.g. <a href="https://gcc.gnu.org/onlinedocs/gccint/GENERIC.html#GENERIC">GENERIC</a>, then <a href="https://gcc.gnu.org/onlinedocs/gccint/GIMPLE.html#GIMPLE">GIMPLE</a>, then <a href="https://en.wikipedia.org/wiki/X86_assembly_language">amd64 assembler</a>, then to the <a href="http://sunnyeves.blogspot.de/2009/07/intel-x86-processors-cisc-or-risc-or.html">internal RISC instructions of the CPU </a>-- with multiple levels of caching in addition.

So I quickly wrote the following (thanks so much for C++11 having the nice <code>std::chrono</code> now).

data.hxx:
```C++
#include <vector>
struct Data {
    Data();
    Data(int a);
    int m_a;
};
void DoSomething(std::vector<Data>&);
```

data.cxx:
```C++
#include "data.hxx"
// noop in different compilation unit to prevent optimizing out what we want to measure
void DoSomething(std::vector<Data>&) {};
Data::Data() : m_a(4711) {};
Data::Data(int a) : m_a(a+4711) {};
```

main.cxx:
```C++
#include "data.hxx"
#include <iostream>
#include <vector>
#include <chrono>
#include <functional>

void A1(long count) {
    while(--count) {
        std::vector<Data> vec { Data(), Data(), Data() };
        DoSomething(vec);
    }
}

void A2(long count) {
    while(--count) {
        std::vector<Data> vec { {}, {}, {} };
        DoSomething(vec);
    }
}

void A3(long count) {
    while(--count) {
        std::vector<Data> vec { 0, 0, 0 };
        DoSomething(vec);
    }
}

void B1(long count) {
    while(--count) {
        std::vector<Data> vec;
        vec.reserve(3);
        vec.push_back(Data());
        vec.push_back(Data());
        vec.push_back(Data());
        DoSomething(vec);
    }
}

void B2(long count) {
    while(--count) {
        std::vector<Data> vec;
        vec.reserve(3);
        vec.push_back({});
        vec.push_back({});
        vec.push_back({});
        DoSomething(vec);
    }
}

void B3(long count) {
    while(--count) {
        std::vector<Data> vec;
        vec.reserve(3);
        vec.push_back(0);
        vec.push_back(0);
        vec.push_back(0);
        DoSomething(vec);
    }
}

void C1(long count) {
    while(--count) {
        std::vector<Data> vec;
        vec.reserve(3);
        vec.emplace_back(Data());
        vec.emplace_back(Data());
        vec.emplace_back(Data());
        DoSomething(vec);
    }
}

void C3(long count) {
    while(--count) {
        std::vector<Data> vec;
        vec.reserve(3);
        vec.emplace_back(0);
        vec.emplace_back(0);
        vec.emplace_back(0);
        DoSomething(vec);
    }
}

double benchmark(const char* name, std::function<void (long)> testfunc, const long count) {
    const auto start = std::chrono::system_clock::now();
    testfunc(count);
    const auto end = std::chrono::system_clock::now();
    const std::chrono::duration<double> delta = end-start;
    std::cout << count << " " << name << " iterations took " << delta.count() << " seconds." << std::endl;
    return delta.count();
}

int main(int, char**) {
    long count = 10000000;
    while(benchmark("A1", &A1, count) < 60l)
        count <<= 1;
    std::cout << "Going with " << count << " iterations." << std::endl;
    benchmark("A1", &A1, count);
    benchmark("A2", &A2, count);
    benchmark("A3", &A3, count);
    benchmark("B1", &B1, count);
    benchmark("B2", &B2, count);
    benchmark("B3", &B3, count);
    benchmark("C1", &C1, count);
    benchmark("C3", &C3, count);
    return 0;
}
```

Makefile:
```C++
CFLAGS?=-O2
main: main.o data.o
    g++ -o $@ $^

%.o: %.cxx data.hxx
    g++ $(CFLAGS) -std=c++11 -o $@ -c $<
```

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
	<li>With constructors inlined, everything in the loop except <code>DoSomething()</code> could be inline. It seems to me that the compiler could -- at least in theory -- figure out that it is asked the same thing in all cases. Namely, reserve space for three ints on the heap, fill them each with 4711 and make the <code>::std::vector&lt;int></code> data structure on the stack reflect that, then hand that to the <code>DoSomething()</code> function that you know nothing about. If the compiler would figure that out, it would take the same time for all implementations. This doesnt happen either on <code>-O2</code> (differ by ~18% from quickest to slowest) nor on <code>-O3</code> (differ by ~3.6%).</li>
</ul>
One common mantra in applications development is "trust the compiler to optimize". The above observations show a few cracks in the foundations of that, esp. if you take into account that this is all on the same version of the same compiler running on the same platform and hardware with the same STL implementation. For huge objects with expensive constructors, the constructor counting approach might still be valid. Then again, those are rarely statically initialized as a bigger bunch into a vector. For the more common scenario of smaller objects with cheap constructors, my tentative conclusion so far would be to go with A1/A2/A3 -- not so much because they are quickest in the most common build scenarios on my platform, but rather because the readability of them is a value on its own while the performance picture is muddy at best.

And hey, if you want to run the tests above on other platforms or compilers, I would be interested in results!

Note: I did these runs for each scenario only once, thus no standard deviation is given. In general, they seemed to be rather stable, but this being wallclock measurements, one or the other might be outliers. caveat emptor.

Originally published on 2015-03-02 22:47:07 on [wordpress](https://skyfromme.wordpress.com/2015/03/02/50-ways-to-fill-your-vector/).

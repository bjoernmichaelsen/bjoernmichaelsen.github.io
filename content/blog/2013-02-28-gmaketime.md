+++
title="gbuild noop times"
date=2013-02-28
[taxonomies]
originally-published-on=["wordpress"]
categories=[]
+++
gbuild noop times
=================

How gbuild spends the 37 seconds to ensure that nothing need to be rebuild: orange = reading the definition of targets (singlethreaded, CPU-bound), grey = stat'ing and checking the filesystem, blue = running sanity tests (multithreaded)
Originally published on 2013-02-28 14:27:28 at https://skyfromme.wordpress.com/2013/02/28/one/gmaketime/.

#Description

This is an attempt at writing a thread safe, distributed key/value store that supports transactions.

###Note
Rust is ultimately not the best
language choice for such a design since it lacks a Garbage Collector, which is necessary to handle old roots. That is not to say it can't
be done in Rust, but it would require writing a GC implementation, which is outside the scope of what I was trying to achieve

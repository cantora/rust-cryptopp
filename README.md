# rust-cryptopp
rust bindings for the cryptopp c++ library

i got distracted from this project so its in kind of
a weird state. however, the core development is
actually pretty much done: the code in `gen` implements
a sort of DSL using rust macros that makes it really
easy to make new bindings to cryptopp components. it
generally takes care of the boring stuff like constructors,
drop, and some common traits that are desirable for c++
classes.

the current phase of this development effort is
to create a "rustic" interface to the c++ classes of
cryptopp (see `src/hash/mod.rs` for an example of the
kind of API i'm going for). however, im not actively
working on this now, though i may return to it at some
point.

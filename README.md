
# AssociativePositionalList (Rust crate)

AssociativePositionalList is a positional
container in which each value is associated with an index,
starting at 0 for the first element. Values can be `insert`ed
and `remove`d at any index (with logarithmic time complexity).
The value at any index can be accessed with `get`. But
unlike other list containers such as `Vec`, the association
between index and value is reversible, and the index
for a value may be determined using `find`.

* [Documentation at docs.rs](https://docs.rs/associative_positional_list)
* [Source code on Github](https://github.com/jwhitham/associative_positional_list)

## Notes

Items in AssociativePositionalList must be unique so that
`find` is unambiguous. They must also be `Clone`able and `Hash`able.

This crate was developed by a relative newcomer to Rust as part
of a learning exercise. It may not be very efficient. Some of
the interfaces you may expect as part of a list container
(or a set) are not present.


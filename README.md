Demonstration of possible m4a tag reading bug.

To reproduce, run `cargo run`.

The output is 0 but should be 1. AtomicParsley reports the tag as this:

```
❯ AtomicParsley sample.m4a -t
Atom "----" [com.apple.iTunes;iTunSMPB] contains:  00000000 00000840 0000000C 000000000025E3B4 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
Atom "©alb" contains: Mind Maps 4
Atom "©ART" contains: The Future Sound Of London
Atom "©day" contains: 2023
Atom "©gen" contains: Electronic
Atom "----" [com.apple.iTunes;LABEL] contains: Touched - Music For Macmillan Cancer Support
Atom "----" [com.apple.iTunes;LABEL] contains: fsoldigital.com
Atom "©nam" contains: Skuttle
Atom "©too" contains: reference libFLAC 1.3.2 20170101
Atom "cpil" contains: 1
Atom "trkn" contains: 18 of 18
Atom "disk" contains: 1 of 1
```

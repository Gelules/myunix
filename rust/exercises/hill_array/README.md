# Docs

**This is just an exercise**

## Algorithm

A hill is represented as an array of u32.

A valid hill is increasing until a top, then decreasing.

If there are several contiguously max as a top, the top is the first index
among them.

This is a valid hill: [0, 1, 3, 5, 7, 7, 5, 4] // Top is 7 at index 6
This is *not* a valid hill: [-42, -11, 0, 1, 3, 5, 7, 6, 7, 5, 4] // two
reasons: negative integers and two tops not contiguous.

## Documentation

You can generate the doc with:
```shell
rustdoc README.md
```

## Testsuite

Execute *cargo test* to execute the testsuite.

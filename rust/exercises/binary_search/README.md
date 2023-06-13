# Docs

**This is just an exercise**

You should use the real
```rust
vector.binary_search(&item);
```
to test presence of an item in a sorted list.

## Algorithm
The array must be sorted.

Binary search get the index of a searched element in a logarithmic complexity
using dichotomy.

If the min and max indices of the sub-vector are equal or reversed (max < min),
the search is negative (we did not find the element).

Otherwise, the middle element of the vector is chosen as a pivot.
* If searched element = pivot, it returns its position;
* If the searched element is greater than the pivot, it loops back the search on
  the sub-vector, starting at the (pivot + 1) position.
* If the searched element is less than the pivot, it loops back the search on
  the sub-vector, ending at the (pivot - 1) position.

You can generate the doc with:
```shell
rustdoc README.md
```

## Testsuite
Execute *cargo test* to execute the testsuite.

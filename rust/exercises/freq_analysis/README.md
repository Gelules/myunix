# Docs

**This is just an exercise**

## Algorithm

This is a monoalphabetic substitution analysis tool.

I have two text in argument, the ciphered one and a table listing all the
characters from the original text from the most common to the least common.

The tool diplays the associations according to the occurence frequencies of each
letters.

Because it's an algorithm exercise and not a real tool, only uppercase letters
are used.

## Documentation

You can generate the doc with:
```shell
rustdoc README.md
```

## Testsuite

Execute *cargo test --jobs=1 -- -no-capture --test-threads=1* to execute the testsuite.

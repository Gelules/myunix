# My UNIX tools
I sometimes want to train on a new programming language. This repo offers me the
possibility to recode UNIX tools in different programming languages.

## Why UNIX tools?
Because UNIX has a nice philosophy:

1. Make each program do one thing well.
2. Make each program use text as data.
3. Expect the output to each program to become the input to another.

And more elegant rules you should read.

Each tool should be usable like the original UNIX one. Only the code differs.
Although, a README file is included in each project to present how the program
is actually working, since I don't write a tool exactly like the original one on
a single commit.

Manpages are my specs. Else, documents from my schools or from Internet.

## Examples
Here are some tools I could recode to train:
* C functions, libraries and computer science functions:
    * rotn
    * atoi and itoa
    * fnmatch
    * libc functions
    * libstring (functions in string.h)
    * libstream (IO functions)
    * mathematical library (generics)
    * sets of data structures (FIFOs, LIFOs, ...)
    * sorters for these sets
    * lexers, parsers and ASTs
    * compilers
    * ... and so on. Even if the language is OOP or already have these. It's
      still good to recode existing and documented implementations to train.

* The "shell" ones (builtins and binaries):
    * cd
    * echo
    * history
    * kill
    * printf
    * pwd
    * type
    * which
    * cat
    * cp
    * cut
    * dd
    * find
    * ls
    * mkdir
    * mv
    * rmdir
    * rm
    * touch
    * uniq
    * wc
    * ... and so on.
* A POSIX shell-like with the sh's builtins.
* A curl-like program using a libcurl-like librarie.
* A netcat client and server to train to code with sockets.
* The EPITA ones: kinda already listed, but not all of them. Exercises from the
  Piscines, programming workshops and the projects.

## Methodology
I use the [Test-Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) process in this repo.
My goal is also to improve my skills in [Software craftsmanship](https://en.wikipedia.org/wiki/Software_craftsmanship), [Agile](https://en.wikipedia.org/wiki/Agile_software_development) and [XP](https://en.wikipedia.org/wiki/Extreme_programming).

## Architecture
At first, I choose which language I want to train on. Then, if it is a libc
function, a shell's builtin or a UNIX program. This is a project.

Secondly, I wrote some documentation files, a README, a TODO, whatsoever is
needed to understand the project.

Thirdly, I code at least a part of the testsuite. Remember that an exhaustive
testsuite is hardly possible.

Finally, I write the project's code. I should document the code to help
programmers understand it.

The project is over when the testsuite is passing all the tests, the
documentation is complete and I'm happy with my work.

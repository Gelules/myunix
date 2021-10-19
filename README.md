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
Although, a README file in included in each project to present how the program
is actually working, since I don't write a tool exactly like the original one on
a single commit.

## Examples
Here are some tools I could recode to train:

* The "shell" ones:
    * echo
    * cat
    * mkdir
    * rmdir
    * cp
    * mv
    * rm
    * ls
    * cd
    * pwd
    * touch
    * dd
    * find
    * uniq
    * wc
    * cut
    * ... and so on.
* A shell-like using the previous listed tools as builtin commands.
* A curl-like program using a libcurl-like librarie.
* A netcat client and server to train to code with sockets.

## Methodology
I use the [Test-Driven Development](https://en.wikipedia.org/wiki/Test-driven_development) process in this repo.
My goal is also to improve my skills in [Software craftsmanship](https://en.wikipedia.org/wiki/Software_craftsmanship), [Agile](https://en.wikipedia.org/wiki/Agile_software_development) and [XP](https://en.wikipedia.org/wiki/Extreme_programming).

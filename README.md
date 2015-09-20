# Gist

A command-line tool for publishing gists, inspired by [icholy/gist][gogist].

[gogist]: https://github.com/icholy/gist

## Usage:

Read a file from stdin:

```sh
cat notes.md | gist
```

Set a file name:

```sh
cat error.log | gist -f "weird-bug.log"
```

Make a public Gist:

```sh
cat file.sh | gist -p
```

Multiple files?

```sh
gist src/*.rs
```

I want the Gist's URL copied to my clipboard, when it's done:

```sh
# *nix / X.org
echo stuff | gist | xclip

# mac
echo something | gist | pbcopy
```

## Install:

``` sh
$ git clone https://github.com/lespepitos/gist.git
$ cd gist
$ cargo build --release
$ ./target/release/gist
```

For authentication, the program looks for an environment variable called
`GITHUB_TOKEN`: it is mandatory to create "secret" gist (the default).

You can generate one at: https://github.com/settings/tokens

Then append it to your `.profile`, or something with:

```sh
export GITHUB_TOKEN="blah blah blah"
```

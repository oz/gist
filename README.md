# Gist

A command-line tool for publishing gists, inspired by [icholy/gist][gogist].

[![Build Status](https://travis-ci.org/LesPepitos/gist.svg?branch=master)](https://travis-ci.org/LesPepitos/gist)

## Usage

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

## Installation

Installation currently requires [cargo][cargo], just `cargo install gist`.

For authentication, the program looks for an environment variable called
`GITHUB_GIST_TOKEN` or `GITHUB_TOKEN`: it is mandatory to create a "secret"
gist (the default).

You can generate one at: https://github.com/settings/tokens

Then append it to your `.profile`, or something with:

```sh
export GITHUB_TOKEN="blah blah blah"
# or
export GITHUB_GIST_TOKEN="blah blah blah"
```

## License

MIT.

## Hacking & bug reports

Yes please: file issues, or better send patches and pull-requests.

[cargo]: https://crates.io
[gogist]: https://github.com/icholy/gist

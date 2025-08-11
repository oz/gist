# Gist

A command-line tool for publishing gists, inspired by [icholy/gist][gogist].

## Usage

Publish a single file, read from stdin:

```sh
cat notes.md | gist
```

Set a file name with `-f`:

```sh
cat error.log | gist -f "weird-bug.log"
```

Make a public Gist with `-p`:

```sh
cat file.sh | gist -p
```

Make a single gist with multiple files, preserving their names:

```sh
gist src/*.rs
```

I want the Gist's URL copied to my clipboard, when it's done:

```sh
# *nix / Wayland
echo stuff | gist | wl-copy

# mac
echo something | gist | pbcopy
```

Show me a brief list of public gists:

```sh
gist -l
```

... or for a single user:

```sh
gist -l some_login
```

BTW, did you know that Github gists are also git repositories? You can
get a local copy of a gist by passing its URL (this uses `git clone`
behind the scenes):

```sh
gist https://gist.github.com/oz/123478097
```


## Installation

Installation currently requires [cargo][cargo], just `cargo install gist`.

For authentication, the program requires an environment variable called
`GITHUB_GIST_TOKEN` or `GITHUB_TOKEN`. It is mandatory as Github killed
anonymous Gists in 2018.

You can generate one at: https://github.com/settings/tokens

Then append it to your `.profile`, or something with:

```sh
export GITHUB_TOKEN="blah blah blah"
# or
export GITHUB_GIST_TOKEN="blah blah blah"
```

Or you can place it in the global configuration file (`~/.gist/config.json`):

```json
{
  "gist_token": "blah blah blah"
}
```

## Github Enterprise

To use with Github Enterprise, set the env. var
`GITHUB_GIST_API_ENDPOINT` to your private Gist API endpoint.

## License

MIT.

## Hacking & bug reports

Yes please: file issues, or better send patches and pull-requests.

[cargo]: https://crates.io
[gogist]: https://github.com/icholy/gist

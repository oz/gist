# Gist

> Command line tool for publishing gists

## Usage:

``` sh
# read from stdin
cat file.sh | gist

# set file name
cat file.sh | gist -f "myfile.sh"

# make public
cat file.sh | gist -p

# multiple files
gist *.rs
```

## Install:

``` sh
$ git clone https://github.com/oz/gist.git
$ cd gist
$ cargo build --release
$ ./target/release/gist
```

For auth, the tool looks for an environment variable called `GITHUB_TOKEN`
You can generate one at: https://github.com/settings/tokens

``` sh
export GITHUB_TOKEN="blah blah blah"
```

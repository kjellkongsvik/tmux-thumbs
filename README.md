# thumbs

A pure standalone fork of [tmux-thumbs](https://github.com/fcsonline/tmux-thumbs)

## Usage

If you want terminal hints, you can do this via pipes. Works great with zellij:

```sh
> git log | thumbs
> zellij ac dump-screen | thumbs
```

### Matched patterns

- File paths
- File in diff
- Git SHAs
- IPFS CID's
- Colors in hex
- Numbers ( 4+ digits )
- Hex numbers
- Markdown urls
- IPv4, IPv6 addresses
- Docker images
- kubernetes resources
- UUIDs

These are the list of matched patterns that will be highlighted by default. If
you want to highlight a pattern that is not in this list you can add one or
more with `--regexp` parameter.

#### Colors

This is the list of predefined colors:

- black
- red
- green
- yellow
- blue
- magenta
- cyan
- white
- default

There is also support for using hex colors in the form of `#RRGGBB`.

#### Alphabets

This is the list of available alphabets:

- `numeric`: 1234567890
- `abcd`: abcd
- `qwerty`: asdfqwerzxcvjklmiuopghtybn
- `qwerty-homerow`: asdfjklgh
- `qwerty-left-hand`: asdfqwerzcxv
- `qwerty-right-hand`: jkluiopmyhn
- `azerty`: qsdfazerwxcvjklmuiopghtybn
- `azerty-homerow`: qsdfjkmgh
- `azerty-left-hand`: qsdfazerwxcv
- `azerty-right-hand`: jklmuiophyn
- `qwertz`: asdfqweryxcvjkluiopmghtzbn
- `qwertz-homerow`: asdfghjkl
- `qwertz-left-hand`: asdfqweryxcv
- `qwertz-right-hand`: jkluiopmhzn
- `dvorak`: aoeuqjkxpyhtnsgcrlmwvzfidb
- `dvorak-homerow`: aoeuhtnsid
- `dvorak-left-hand`: aoeupqjkyix
- `dvorak-right-hand`: htnsgcrlmwvz
- `colemak`: arstqwfpzxcvneioluymdhgjbk
- `colemak-homerow`: arstneiodh
- `colemak-left-hand`: arstqwfpzxcv
- `colemak-right-hand`: neioluymjhk

## Extra features

- **Arrow navigation:** You can use the arrows to move around between all matched items.
- **Auto paste:** If your last typed hint character is uppercase, you are going to pick and paste the desired hint.

### Multi selection

If you want to enable the capability to choose multiple matches, you have to
press <kbd>Space</kbd>. Then, choose the matches with highlighted hints or
<kbd>Enter</kbd> (moving with cursors) and then <kbd>Space</kbd> again to
output all of them.

If you run standalone `thumbs` with multi selection mode (-m) you will be able to choose multiple hints pressing the desired letter and <kbd>Space</kbd> to finalize the selection.

```
> git log | thumbs -m
1df9fa69c8831ac042c6466af81e65402ee2a007
4897dc4ecbd2ac90b17de95e00e9e75bb540e37f
```

## Background

When I swithced from tmux to zellij I really missed tmux-fingers.

# License

[MIT](https://github.com/fcsonline/tmux-thumbs/blob/master/LICENSE)

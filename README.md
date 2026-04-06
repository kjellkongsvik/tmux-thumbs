# thumbs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A pure standalone version of [tmux-thumbs](https://github.com/fcsonline/tmux-thumbs)

This readme is not up to date yet

## Usage

If you want to enjoy terminal hints, you can do things like this without `tmux`:

```
> git log | thumbs
> zellij ac dump-screen | thumbs | wl-copy
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

## Deprecated tmux Configuration, kept for now

TODO: Move into help

If you want to customize how is shown your tmux-thumbs hints those all available
parameters to set your perfect profile.

NOTE: for changes to take effect, you'll need to source again your `.tmux.conf` file.

* [@thumbs-key](#thumbs-key)
* [@thumbs-alphabet](#thumbs-alphabet)
* [@thumbs-reverse](#thumbs-reverse)
* [@thumbs-unique](#thumbs-unique)
* [@thumbs-position](#thumbs-position)
* [@thumbs-regexp-N](#thumbs-regexp-N)
* [@thumbs-command](#thumbs-command)
* [@thumbs-upcase-command](#thumbs-upcase-command)
* [@thumbs-multi-command](#thumbs-multi-command)
* [@thumbs-bg-color](#thumbs-bg-color)
* [@thumbs-fg-color](#thumbs-fg-color)
* [@thumbs-hint-bg-color](#thumbs-hint-bg-color)
* [@thumbs-hint-fg-color](#thumbs-hint-fg-color)
* [@thumbs-select-fg-color](#thumbs-select-fg-color)
* [@thumbs-select-bg-color](#thumbs-select-bg-color)
* [@thumbs-multi-fg-color](#thumbs-multi-fg-color)
* [@thumbs-multi-bg-color](#thumbs-multi-bg-color)
* [@thumbs-contrast](#thumbs-contrast)
* [@thumbs-osc52](#thumbs-osc52)



### @thumbs-reverse

`default: disabled`

Choose in which direction you want to assign hints. Useful to get shorter hints closer to the cursor.

For example:

```
set -g @thumbs-reverse enabled
```

### @thumbs-unique

`default: disabled`

Choose if you want to assign the same hint for the same matched strings.

For example:

```
set -g @thumbs-unique enabled
```

### @thumbs-position

`default: left`

Choose where do you want to show the hint in the matched string. Options (left, right, off_left, off_right).

For example:

```
set -g @thumbs-position right
```

### @thumbs-regexp-N

Add extra patterns to match. This parameter can have multiple instances.

For example:

```
set -g @thumbs-regexp-1 '[\w-\.]+@([\w-]+\.)+[\w-]{2,4}' # Match emails
set -g @thumbs-regexp-2 '[a-f0-9]{2}:[a-f0-9]{2}:[a-f0-9]{2}:[a-f0-9]{2}:[a-f0-9]{2}:[a-f0-9]{2}:' # Match MAC addresses
set -g @thumbs-regexp-3 'Vlan\d+' # match Vlan interface on network devices
set -g @thumbs-regexp-4 "Vlan\\d+" # alternative method of defining regexp
set -g @thumbs-regexp-5 Vlan\\d+ # alternative method of defining regexp
```

### @thumbs-command

`default: 'tmux set-buffer -- {} && tmux display-message \"Copied {}\"'`

Choose which command execute when you press a hint. `tmux-thumbs` will replace `{}` with the picked hint.

For example:

```
set -g @thumbs-command 'echo -n {} | pbcopy'
```

### @thumbs-upcase-command

`default: 'tmux set-buffer -- {} && tmux paste-buffer && tmux display-message \"Copied {}\"'`

Choose which command execute when you press a upcase hint. `tmux-thumbs` will replace `{}` with the picked hint.

For example:

```
set -g @thumbs-upcase-command 'echo -n {} | pbcopy'
```

### @thumbs-multi-command

`default: 'tmux set-buffer -- {} && tmux paste-buffer && tmux send-keys ' ' && tmux display-message \"Copied multiple items!\"'`

Choose which command execute when you select multiple items. `tmux-thumbs` will replace `{}` with the picked hint for each one.

For example:

```
set -g @thumbs-multi-command 'echo -n {}'
```

### @thumbs-bg-color

`default: black`

Sets the background color for matches

For example:

```
set -g @thumbs-bg-color blue
```

### @thumbs-fg-color

`default: green`

Sets the foreground color for matches

For example:

```
set -g @thumbs-fg-color green
```

### @thumbs-hint-bg-color

`default: black`

Sets the background color for hints

For example:

```
set -g @thumbs-hint-bg-color blue
```

### @thumbs-hint-fg-color

`default: yellow`

Sets the foreground color for hints

For example:

```
set -g @thumbs-hint-fg-color green
```

### @thumbs-select-fg-color

`default: blue`

Sets the foreground color for selection

For example:

```
set -g @thumbs-select-fg-color red
```

### @thumbs-select-bg-color

`default: black`

Sets the background color for selection

For example:

```
set -g @thumbs-select-bg-color red
```

### @thumbs-multi-fg-color

`default: yellow`

Sets the foreground color for multi selected item

For example:

```
set -g @thumbs-multi-fg-color green
```

### @thumbs-multi-bg-color

`default: black`

Sets the background color for multi selected item

For example:

```
set -g @thumbs-multi-bg-color red
```

### @thumbs-contrast

`default: 0`

Displays hint character in square brackets for extra visibility.

For example:

```
set -g @thumbs-contrast 1
```

### @thumbs-osc52

`default: 0`

If this is set to `1`, `tmux-thumbs` will print a OSC52 copy escape sequence when you select a match, in addition to running the pick command. This sequence, in terminals that support it (e.g. iTerm), allows the content to be copied into the system clipboard in addition to the tmux copy buffer.

For example:

```
set -g @thumbs-osc52 1
```

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

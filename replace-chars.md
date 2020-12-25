with replacing charachters with vim we can use the `/` in command mode.
This will search some given pattern.
For eample search all the let decalration in ajavascript file.
`/let`
will highlight all let declrations.


## Search and replace with vim
on the same line replacing matched pattern.
So let replace on `one` line our let decalration to a constant instead.
```vim
:s/let/const
```

For a gloabl replacement we will use:
```vim
:%s/let/const
```

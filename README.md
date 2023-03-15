Fold ASCII trees in Vim
====

Supported trees
----

* systemctl status
* tree
* cargo tree

Install
----

Install the rust toolchain and run:

```
cargo install vim-foldtree
```

Copy `foldtree.vim` into your Vim's plugin directory `~/.vim/plugin`.

Usage
----

Edit the tree graph into a Vim buffer and run `:FoldTree`.

Or use a pipeline like this:

```sh
$ systemctl status | vim -c FoldTree -R -
```

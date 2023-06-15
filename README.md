# checkplus

score the moves in a chess game with a UCI engine

# Usage

You can pass an optional depth at which to examine each move and a required path
to a PGN file:

```shell
checkplus --depth 20 testfiles/sample.pgn
```

This produces output like:

```text
0 0.37
1 0.35
2 0.37
3 0.26
4 0.37
5 0.28
6 0.41
7 0.45
8 0.49
9 0.50
```

You can also use the included `gamecheck` script to run the command above on a
PGN piped from your clipboard and visualize the results in `gnuplot`:

```shell
gamecheck [DEPTH]
```

Like `checkplus` itself, `gamecheck` takes an optional argument specifying the
depth of search.

# Dependencies

The only engine currently supported is
[Stockfish](https://github.com/official-stockfish/Stockfish), which needs to be
on your `PATH`. To use the `gamecheck` script, both
[xclip](https://github.com/astrand/xclip) and
[gnuplot](http://www.gnuplot.info/) must also be installed and available on your
`PATH`.

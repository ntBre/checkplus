# checkplus

score the moves in a chess game with a UCI engine

# Usage

You can pass an optional depth at which to examine each move and a required path to a PGN file:

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

# Dependencies

The only engine currently supported is [Stockfish](https://github.com/official-stockfish/Stockfish), 
which needs to be on your `PATH`.

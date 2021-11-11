# N-Puzzel_with_Rust
42 school project: solve the N-puzzle ("taquin" in French) game using the A* search algorithm or one of its variants

## Usage
```
cargo run - [options]

Options:
    -h, --help          print this help menu
    -u, --unsolvable    generate an unsolvable starting board
    -g, --generate Ex: 4
                        generate a starting board radomly with given size
    -i, --iteration Ex: 500
                        define nb of iterations when generating the starting
                        board, default is 1000
    -f, --file Ex: testfiles/test3-1
                        get starting board from a filepath
    -h, --heuristic Ex: euclidean
                        choose heuristic in [manhatten](default) [euclidean]
                        [tiles_out_of_place]

```
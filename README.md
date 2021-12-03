# N-Puzzle_with_Rust
42 school project: solve the N-puzzle ("taquin" in French) game using the A* search algorithm
<img width="614" alt="屏幕快照 2021-11-11 11 56 14" src="https://user-images.githubusercontent.com/39769383/141286801-2316831f-d5e2-4cdb-8635-c052b2b77dfd.png">

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

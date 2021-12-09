use puzzle::algo::search;
use puzzle::generator::generator;
use puzzle::Heuristic;
use puzzle::types::Algo;

#[test]
fn test_puzzle_size_three() {
    assert_eq!(
        search(generator(3, 50, false), Algo::Astar, Heuristic::Manhattan, 3).unwrap(),
        [1, 2, 3, 8, 0, 4, 7, 6, 5]
    );
}

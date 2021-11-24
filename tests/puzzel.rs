use puzzle::a_star::a_star;
use puzzle::generator::generator;
use puzzle::Heuristic;


#[test]
fn test_puzzle_size_three() {
    assert_eq!(a_star(generator(3, 50, false), Heuristic::Manhattan).unwrap(), [1, 2, 3, 8, 0, 4, 7, 6, 5]);
}

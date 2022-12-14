use rstest::rstest;

use crate::stop_swapper::StopSwapper;

use crate::tests::fixtures::stop_swapper_fixture::{path_factory, stop_swapper, PathFactory};

#[rstest]
fn can_calculate_path_swap_cost(stop_swapper: StopSwapper, path_factory: PathFactory) {
    let path1 = path_factory.make_path(1).unwrap();
    let path2 = path_factory.make_path(3).unwrap();

    let swap_cost = stop_swapper.calculate_swap_cost(&path1, &path2);

    assert_eq!(swap_cost, -4.0);
}

#[rstest]
fn can_get_the_minimum_swap_cost(stop_swapper: StopSwapper, path_factory: PathFactory) {
    let path = path_factory.make_path(1).unwrap();

    let swap_cost = stop_swapper
        .get_minimum_swap_cost(&path, &path_factory.stops)
        .unwrap();

    assert_eq!(swap_cost.1, -5.0);
}

#[rstest]
fn can_calculate_path_swap_cost_of_consecutive_swaps(
    stop_swapper: StopSwapper,
    path_factory: PathFactory,
) {
    let path1 = path_factory.make_path(1).unwrap();
    let path2 = path_factory.make_path(2).unwrap();

    let swap_cost1 = stop_swapper.calculate_swap_cost(&path1, &path2);
    let swap_cost2 = stop_swapper.calculate_swap_cost(&path2, &path1);

    assert_eq!(swap_cost1, -4.5);
    assert_eq!(swap_cost2, -4.5);
}

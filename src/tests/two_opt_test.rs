use crate::{
    domain::{route::Route, vehicle::Vehicle},
    local_search::{two_opt::{TwoOptSearcher}, path::Path},
    services::distance::distance_service::DistanceService,
};
use rstest::rstest;

use crate::domain::stop::Stop;

use super::fixtures::{distance_service, stops_with_crossings};

#[rstest]
fn can_calculate_insertion_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let path = Path::from_stop_index(&stops_with_crossings, 1, &distance_service).unwrap();

    assert_eq!(path.cost, 8.0);
}

#[rstest]
fn can_calculate_path_swap_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let path1 = Path::from_stop_index(&stops_with_crossings, 1, &distance_service).unwrap();
    let path2 = Path::from_stop_index(&stops_with_crossings, 3, &distance_service).unwrap();

    let swap_cost = TwoOptSearcher::calculate_swap_cost(&path1, &path2, &distance_service);

    assert_eq!(swap_cost - (path1.cost + path2.cost), -4.0);
}

#[rstest]
fn can_get_the_minimum_swap_cost(
    distance_service: DistanceService,
    stops_with_crossings: Vec<Stop>,
) {
    let stop_index = 1;
    let path = Path::from_stop_index(&stops_with_crossings, stop_index, &distance_service).unwrap();

    let swap_cost = TwoOptSearcher::get_minimum_swap_cost(&path, &stops_with_crossings, &distance_service).unwrap();
        
    assert_eq!(swap_cost.1, 9.0);
}

#[rstest]
fn can_optimize_route(distance_service: DistanceService, stops_with_crossings: Vec<Stop>) {
    let vehicle = Vehicle::new(0, 100);

    let mut route = Route::new(vehicle);

    route.add_stop(stops_with_crossings[0], 0.0).unwrap();
    for (index, stop) in stops_with_crossings.iter().enumerate().skip(1) {
        route
            .add_stop(
                *stop,
                distance_service
                    .get_distance(&stops_with_crossings[index-1], stop)
                    .unwrap(),
            )
            .unwrap();
    }

    TwoOptSearcher::search(&mut route, &distance_service).unwrap();

    assert_eq!(route.stops.get(0).unwrap().id, 0);
    assert_eq!(route.stops.get(1).unwrap().id, 2);
    assert_eq!(route.stops.get(2).unwrap().id, 3);
    assert_eq!(route.stops.get(3).unwrap().id, 1);
    assert_eq!(route.stops.get(4).unwrap().id, 4);
    assert_eq!(route.stops.get(5).unwrap().id, 0);
}
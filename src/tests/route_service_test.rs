use rstest::rstest;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop},
    services::route::route_service::RouteService,
    tests::fixtures::VehicleFactory,
};

use super::fixtures::{distances, stops, vehicle_factory};

#[rstest]
fn route_service_started(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let vehicles = vehicle_factory(2);

    let route_service = RouteService::new(vehicles, distances, stops);

    assert_eq!(route_service.get_available_stops().len(), 5);
    assert_eq!(route_service.get_all_routes().len(), 2);
    assert_eq!(route_service.get_vehicles().len(), 2);
}

#[rstest]
fn can_assign_stop_to_route(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let vehicles = vehicle_factory(1);

    let mut route_service = RouteService::new(vehicles, distances, stops);

    route_service.assign_stop_to_route(0, 0).unwrap();

    assert_eq!(route_service.get_route(0).unwrap().get_stops().len(), 1)
}

#[rstest]
fn can_get_nearest_stop(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let vehicles = vehicle_factory(1);

    let mut route_service = RouteService::new(vehicles, distances, stops);
    route_service.assign_stop_to_route(0, 0).unwrap();

    assert_eq!(route_service.get_nearest_stop(0).unwrap().get_id(), 2);
}

#[rstest]
fn can_get_k_nearest_stops(
    vehicle_factory: VehicleFactory,
    stops: Vec<Stop>,
    distances: DistanceMatrix,
) {
    let vehicles = vehicle_factory(1);

    let mut route_service = RouteService::new(vehicles, distances, stops);
    route_service.assign_stop_to_route(0, 0).unwrap();

    let k_nearest = route_service.get_k_nearest_stops(0, 3).unwrap();

    assert_eq!(k_nearest.len(), 3);

    assert_eq!(k_nearest[0].get_id(), 2);
    assert_eq!(k_nearest[1].get_id(), 1);
    assert_eq!(k_nearest[2].get_id(), 3);
}

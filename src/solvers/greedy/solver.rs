use std::collections::HashMap;

use crate::{
    domain::{route::DistanceMatrix, stop::Stop, vehicle::Vehicle},
    services::route::route_service::RouteService,
    solvers::solver::{Solution, Solver},
};

pub struct GreedySolver<'a> {
    solution: Solution,
    route_service: RouteService<'a>,
}

impl<'a> Solver<'a, GreedySolver<'a>> for GreedySolver<'a> {
    fn new(
        vehicles: &'a mut Vec<Vehicle>,
        distances: &'a DistanceMatrix,
        stops: &'a Vec<Stop>,
    ) -> GreedySolver<'a> {
        GreedySolver {
            solution: HashMap::new(),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    fn construct_routes_in_parallel(route_service: &mut RouteService, vehicle_ids: &Vec<u32>) {
        for vehicle_id in vehicle_ids.iter() {
            let stop_id = match route_service.get_nearest_stop(*vehicle_id) {
                None => break,
                Some(stop) => stop.get_id(),
            };

            route_service
                .assign_stop_to_route(*vehicle_id, stop_id)
                .unwrap();
        }
    }

    fn solve(&mut self) {
        self.route_service.assign_starting_points();
        Self::construct_all_routes(&mut self.route_service);
        self.solution = Self::construct_solutions(&self.route_service);
    }

    fn get_solution(&self) -> &Solution {
        &self.solution
    }

    fn solution_total_distance(&self) -> f64 {
        self.route_service.total_distance()
    }
}

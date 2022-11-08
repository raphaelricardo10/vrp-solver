use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

use crate::{
    domain::stop::Stop,
    services::route::route_service::RouteService,
    solvers::solver::{Solution, Solver},
};

pub struct GraspSolver<'a> {
    rcl_size: usize,
    solution: Solution,
    route_service: RouteService<'a>,
}

impl<'a> GraspSolver<'a> {
    pub fn new(
        rcl_size: usize,
        vehicles: &'a mut Vec<crate::domain::vehicle::Vehicle>,
        distances: &'a crate::domain::route::DistanceMatrix,
        stops: &'a Vec<crate::domain::stop::Stop>,
    ) -> GraspSolver<'a> {
        GraspSolver {
            rcl_size,
            solution: HashMap::new(),
            route_service: RouteService::new(vehicles, distances, stops),
        }
    }

    pub fn get_random_near_stop(&self, vehicle_id: u32) -> Option<&Stop> {
        let near_stops = self.route_service.get_k_nearest_stops(vehicle_id, self.rcl_size)?;
        let chosen = *near_stops.choose(&mut thread_rng())?;

        Some(chosen)
    }
}

impl<'a> Solver<'a, GraspSolver<'a>> for GraspSolver<'a> {
    fn solve(&mut self) {
        self.route_service.assign_starting_points();
        self.run_all_iterations();
        self.solution = Self::construct_solutions(&self.route_service);
    }

    fn get_solution(&self) -> &Solution {
        &self.solution
    }

    fn solution_total_distance(&self) -> f64 {
        self.route_service.total_distance()
    }

    fn run_iteration(&mut self) {
        let vehicle_ids = Self::get_all_vehicle_ids(&self.route_service);

        for vehicle_id in vehicle_ids {
            let stop_id = match self.get_random_near_stop(vehicle_id) {
                None => break,
                Some(stop) => stop.get_id(),
            };

            self.route_service
                .assign_stop_to_route(vehicle_id, stop_id)
                .unwrap();
        }
    }

    fn stop_condition_met(&self) -> bool {
        !self.route_service.has_available_stop().unwrap()
    }
}

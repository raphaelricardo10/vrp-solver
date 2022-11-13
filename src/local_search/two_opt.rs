use std::collections::BTreeMap;

use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceService,
};

use super::{path::Path, path_node::PathNode};

pub(crate) fn calculate_swap_cost<'a>(
    path1: &Path,
    path2: &Path,
    distance_service: &DistanceService,
) -> f64 {
    let swapped_path_1 = Path::new(
        *path1.get_prev(),
        *path2.get_current(),
        *path1.get_next(),
        distance_service,
    )
    .unwrap();

    let swapped_path_2 = Path::new(
        *path2.get_prev(),
        *path1.get_current(),
        *path2.get_next(),
        distance_service,
    )
    .unwrap();

    swapped_path_1.get_cost() + swapped_path_2.get_cost()
}

pub(crate) fn get_minimum_swap_cost<'a>(
    stops: &'a Vec<Stop>,
    distance_service: &'a DistanceService,
    path: &'a Path<'a>,
) -> Option<(usize, f64)> {
    stops[.. stops.len() - 1]
        .iter()
        .enumerate()
        .skip(path.get_next().get_index() + 1)
        .map(|(stop_index, _)| {
            (
                stop_index,
                calculate_swap_cost(
                    path,
                    &Path::from_stop_index(stops, stop_index, distance_service)
                        .unwrap(),
                    distance_service,
                ),
            )
        })
        .min_by(|(_, cost1), (_, cost2)| cost1.partial_cmp(cost2).unwrap())
}

fn should_swap_stops(path1: &Path, path2: &Path, swap_cost: &f64) -> bool {
    *swap_cost < path1.get_cost() + path2.get_cost()
}

fn find_improvements(
    stops: &Vec<Stop>,
    distance_service: &DistanceService,
    path: &Path,
) -> Option<usize> {
    let (swap_candidate_index, swap_cost) = get_minimum_swap_cost(stops, distance_service, &path)?;

    let swap_candidate = Path::from_stop_index(stops, swap_candidate_index, distance_service)?;

    if should_swap_stops(&path, &swap_candidate, &swap_cost) {
        return Some(swap_candidate.get_prev().get_index())
    }

    None
}

fn map_paths<'a>(
    stops: &'a Vec<Stop>,
    distance_service: &'a DistanceService,
) -> BTreeMap<usize, Path<'a>> {
    stops
        .windows(3)
        .enumerate()
        .map(|(base_index, window)| {
            (
                base_index,
                Path::from_window(window, base_index, distance_service).unwrap(),
            )
        })
        .collect()
}

pub fn search(route: &mut Route, distance_service: &DistanceService) -> Option<()> {
    for stop_index in 1..route.get_stops().len() - 1  {
        let path = Path::from_stop_index(route.get_stops(), stop_index, distance_service)?;
        
        let swap_candidate_index = match find_improvements(route.get_stops(), distance_service, &path) {
            Some(candidate_index) => candidate_index,
            None => continue,
        };
        
        let base_index = path.get_current().get_index();
        route.get_stops_mut().swap(base_index, swap_candidate_index);
    }

    Some(())
}
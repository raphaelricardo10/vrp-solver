use crate::{
    domain::{route::Route, stop::Stop},
    services::distance::distance_service::DistanceMatrix,
    stop_swapper::{path::Path, StopSwapper},
};

pub struct TwoOptSearcher {
    stop_swapper: StopSwapper,
}

impl TwoOptSearcher {
    pub fn new(stops: Vec<Stop>, distances: &DistanceMatrix) -> Self {
        Self {
            stop_swapper: StopSwapper::new(stops, distances),
        }
    }

    fn should_swap_stops(swap_cost: f64) -> bool {
        swap_cost < 0.0
    }

    fn find_improvements(&self, stops: &Vec<Stop>, path: &Path) -> Option<(usize, f64)> {
        let (swap_candidate_index, swap_cost) =
            self.stop_swapper.get_minimum_swap_cost(path, stops)?;

        let swap_candidate = Path::from_stop_index(
            stops,
            swap_candidate_index,
            &self.stop_swapper.distance_service,
        )?;

        if Self::should_swap_stops(swap_cost) {
            return Some((swap_candidate.current.index, swap_cost));
        }

        None
    }

    pub fn run(&self, route: &mut Route) -> Option<()> {
        for stop_index in 1..route.stops.len() - 1 {
            let path = Path::from_stop_index(
                &route.stops,
                stop_index,
                &self.stop_swapper.distance_service,
            )?;

            let (swap_candidate_index, swap_cost) =
                match self.find_improvements(&route.stops, &path) {
                    Some(candidate_index) => candidate_index,
                    None => continue,
                };

            let base_index = path.current.index;
            route.swap_stops(base_index, swap_candidate_index, swap_cost);
        }

        Some(())
    }
}

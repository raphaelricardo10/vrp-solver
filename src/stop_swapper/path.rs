use crate::{domain::stop::Stop, services::distance::distance_service::DistanceService};

use super::path_node::PathNode;

#[derive(Copy, Clone)]
pub(crate) struct Path<'a> {
    pub(crate) prev: PathNode<'a>,
    pub(crate) current: PathNode<'a>,
    pub(crate) next: PathNode<'a>,
    pub(crate) cost: f64,
}

impl<'a> Path<'a> {
    pub(crate) fn new(
        prev: PathNode<'a>,
        current: PathNode<'a>,
        next: PathNode<'a>,
        distance_service: &DistanceService,
    ) -> Option<Path<'a>> {
        let mut path = Path {
            prev,
            current,
            next,
            cost: 0.0,
        };

        path.cost = path.calculate_cost(distance_service)?;

        Some(path)
    }

    pub(crate) fn from_stop_index(
        stops: &'a [Stop],
        stop_index: usize,
        distance_service: &DistanceService,
    ) -> Option<Path<'a>> {
        Self::new(
            PathNode::new(stop_index - 1, &stops[stop_index - 1]),
            PathNode::new(stop_index, &stops[stop_index]),
            PathNode::new(stop_index + 1, &stops[stop_index + 1]),
            distance_service,
        )
    }

    fn calculate_cost(&self, distance_service: &DistanceService) -> Option<f64> {
        Some(
            distance_service.get_distance(self.prev.stop, self.current.stop)?
                + distance_service.get_distance(self.current.stop, self.next.stop)?,
        )
    }
}

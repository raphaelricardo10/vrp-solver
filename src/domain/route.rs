use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};
use std::collections::HashMap;

type DistanceMap = HashMap<u32, f64>;
pub type DistanceMatrix = HashMap<(u32, u32), f64>;
pub type DistanceMatrixLine<'a> = (&'a (u32, u32), &'a f64);

pub struct Route<'a> {
    stops: Vec<&'a Stop>,
    distances: DistanceMap,
    vehicle: &'a mut Vehicle,
}

impl<'a> Route<'a> {
    pub fn new(vehicle: &'a mut Vehicle) -> Route<'a> {
        Route {
            vehicle,
            stops: Vec::new(),
            distances: HashMap::new(),
        }
    }

    pub fn get_current_stop(&self) -> Option<&Stop> {
        Some(self.stops.last()?)
    }

    pub fn can_add_stop(&self, stop: &Stop) -> bool {
        self.vehicle.can_support_load(stop.usage)
    }

    pub fn add_stop(&mut self, stop: &'a Stop, distance: f64) -> Result<(), VehicleOverloadError> {
        if let Err(e) = self.vehicle.load(stop.usage) {
            return Err(e);
        }

        self.stops.push(stop);
        self.distances.insert(stop.get_id(), distance);

        Ok(())
    }

    pub fn total_distance(&self) -> f64 {
        self.distances.values().sum()
    }

    pub fn get_vehicle(&self) -> &Vehicle {
        &self.vehicle
    }

    pub fn get_stops(&self) -> &Vec<&'a Stop> {
        &self.stops
    }
}

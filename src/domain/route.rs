use crate::errors::vehicle::vehicle_overload::VehicleOverloadError;

use super::{stop::Stop, vehicle::Vehicle};

pub struct Route {
    pub stops: Vec<Stop>,
    pub vehicle: Vehicle,
    total_distance: f64,
}

impl Route {
    pub fn new(vehicle: Vehicle) -> Route {
        Route {
            vehicle,
            stops: Vec::new(),
            total_distance: f64::default(),
        }
    }

    pub fn get_current_stop(&self) -> Option<&Stop> {
        Some(self.stops.last()?)
    }

    pub fn can_add_stop(&self, stop: &Stop) -> bool {
        self.vehicle.can_support_load(stop.usage)
    }

    pub fn add_stop(&mut self, stop: Stop, distance: f64) -> Result<(), VehicleOverloadError> {
        if let Err(e) = self.vehicle.load(stop.usage) {
            return Err(e);
        }

        self.stops.push(stop);
        self.total_distance += distance;

        Ok(())
    }

    pub fn total_distance(&self) -> f64 {
        self.total_distance
    }

    pub fn swap_stops(&mut self, index1: usize, index2: usize) {
        let aux = self.stops[index1];

        self.stops[index1] = self.stops[index2];
        self.stops[index2] = aux;
    }
}

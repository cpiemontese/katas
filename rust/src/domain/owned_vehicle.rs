use derive_new::new;
use getset::Getters;

use super::{PersonId, VehicleId};

#[derive(Clone, Getters, new)]
pub struct OwnedVehicle {
    person_id: PersonId,
    vehicle_id: VehicleId,
}

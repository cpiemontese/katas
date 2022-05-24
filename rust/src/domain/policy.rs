use derive_new::new;
use getset::Getters;

use super::{PersonId, VehicleId};

#[derive(Getters, new)]
pub struct Policy {
    person_id: PersonId,
    vehicle_id: VehicleId,
}

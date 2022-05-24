use derive_new::new;
use getset::Getters;

use super::{PersonId, VehicleId};

#[derive(Debug, Getters, new)]
pub struct UpsellOpportunity {
    person_id: PersonId,
    vehicle_id: VehicleId,
}

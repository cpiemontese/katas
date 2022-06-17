use derive_new::new;
use getset::Getters;

use super::{owned_vehicle::OwnedVehicle, PersonId, VehicleId};

#[derive(Debug, Getters, new)]
pub struct UpsellOpportunity {
    #[getset(get = "pub")]
    person_id: PersonId,
    #[getset(get = "pub")]
    vehicle_id: VehicleId,
}

impl From<&OwnedVehicle> for UpsellOpportunity {
    fn from(owned_vehicle: &OwnedVehicle) -> Self {
        UpsellOpportunity {
            person_id: owned_vehicle.person_id().clone(),
            vehicle_id: owned_vehicle.vehicle_id().clone(),
        }
    }
}

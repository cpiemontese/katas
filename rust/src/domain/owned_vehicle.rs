use derive_new::new;
use getset::Getters;

use super::{PersonId, VehicleId};

#[derive(Clone, Getters, new)]
pub struct OwnedVehicle {
    #[getset(get = "pub")]
    person_id: PersonId,
    #[getset(get = "pub")]
    vehicle_id: VehicleId,
}

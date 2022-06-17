use derive_new::new;
use getset::Getters;

use super::{PersonId, VehicleId};

#[derive(Getters, new)]
pub struct Policy {
    #[getset(get = "pub")]
    person_id: PersonId,
    #[getset(get = "pub")]
    vehicle_id: VehicleId,
}

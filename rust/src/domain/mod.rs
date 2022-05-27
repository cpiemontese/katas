pub mod owned_vehicle;
pub mod policy;
pub mod upsell_opportunity;

use derive_new::new;
use thiserror::Error;

use crate::services::vehicles::OwnedVehicleAPI;

use self::{policy::Policy, upsell_opportunity::UpsellOpportunity};

pub type PersonId = String;
pub type VehicleId = String;

#[derive(Debug, Error)]
pub enum Error {}

#[derive(new)]
pub struct Upselling {
    owned_vehicle_api: OwnedVehicleAPI,
}

impl Upselling {
    pub async fn find_potential_upsells(
        &self,
        policies: Vec<Policy>,
    ) -> Result<Vec<UpsellOpportunity>, Error> {
        let person_ids: Vec<PersonId> = policies
            .iter()
            .map(|policy| policy.person_id())
            .cloned()
            .collect();
        let owned_vehicles = self.owned_vehicle_api.get_owned_vehicles(&person_ids).await;
        let upsell_opportunities: Vec<UpsellOpportunity> = owned_vehicles
            .iter()
            .filter(|owned_vehicle| {
                policies
                    .iter()
                    .any(|policy| policy.vehicle_id() == owned_vehicle.vehicle_id())
            })
            .map(|owned_vehicle| UpsellOpportunity::from(owned_vehicle))
            .collect();

        Ok(upsell_opportunities)
    }
}

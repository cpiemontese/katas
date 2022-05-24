pub mod owned_vehicle;
pub mod policy;
pub mod upsell_opportunity;

use thiserror::Error;

use self::{policy::Policy, upsell_opportunity::UpsellOpportunity};

pub type PersonId = &'static str;
pub type VehicleId = &'static str;

#[derive(Debug, Error)]
pub enum Error {}

pub fn find_potential_upsells(policies: Vec<Policy>) -> Result<Vec<UpsellOpportunity>, Error> {
    todo!()
}

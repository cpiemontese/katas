use std::collections::HashMap;

use thiserror::Error;

use crate::domain::{owned_vehicle::OwnedVehicle, PersonId};

#[derive(Debug, Error)]
pub enum Error {}

#[derive(Clone)]
pub struct OwnedVehicleAPI {
    person_to_owned_vehicles: HashMap<PersonId, Vec<OwnedVehicle>>,
}

impl OwnedVehicleAPI {
    pub fn new() -> Self {
        // Hardcoded for now
        OwnedVehicleAPI {
            person_to_owned_vehicles: HashMap::from([
                (
                    "P1".to_string(),
                    vec![
                        OwnedVehicle::new("P1".to_string(), "V3".to_string()),
                        OwnedVehicle::new("P1".to_string(), "V8".to_string()),
                    ],
                ),
                (
                    "P2".to_string(),
                    vec![OwnedVehicle::new("P2".to_string(), "V6".to_string())],
                ),
            ]),
        }
    }

    pub async fn get_owned_vehicles(&self, person_ids: &Vec<PersonId>) -> Vec<OwnedVehicle> {
        person_ids
            .into_iter()
            .flat_map(|person_id| {
                self.person_to_owned_vehicles
                    .get(person_id)
                    .cloned()
                    .unwrap_or_default()
            })
            .collect()
    }
}

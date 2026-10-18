use crate::model::WifiObservation;
use macaddr::MacAddr6;
use std::collections::{HashMap, hash_map::Entry};

pub fn deduplicate(observations: Vec<WifiObservation>) -> Vec<WifiObservation> {
    let mut unique: HashMap<MacAddr6, WifiObservation> = HashMap::new();

    for observation in observations {
        match unique.entry(observation.bssid) {
            Entry::Vacant(entry) => {
                entry.insert(observation);
            }
            Entry::Occupied(mut entry) => {
                if observation.signal_dbm > entry.get().signal_dbm {
                    entry.insert(observation);
                }
            }
        }
    }

    unique.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::deduplicate;
    use crate::model::WifiObservation;
    use macaddr::MacAddr6;

    #[test]
    fn keeps_strongest_signal_for_duplicate_bssid() {
        let bssid = MacAddr6::new(0x00, 0x11, 0x22, 0x33, 0x44, 0x55);

        let observations = vec![
            WifiObservation {
                bssid,
                signal_dbm: None,
            },
            WifiObservation {
                bssid,
                signal_dbm: Some(-55),
            },
            WifiObservation {
                bssid,
                signal_dbm: Some(-80),
            },
            WifiObservation {
                bssid,
                signal_dbm: None,
            },
        ];

        let unique = deduplicate(observations);

        assert_eq!(unique.len(), 1);
        assert_eq!(unique[0].bssid, bssid);
        assert_eq!(unique[0].signal_dbm, Some(-55));
    }
}

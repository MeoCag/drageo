use macaddr::MacAddr6;

#[derive(Debug)]
pub struct WifiObservation {
    pub bssid: MacAddr6,
    pub signal_dbm: Option<i16>,
}

use drageo::location::checked_point;
use drageo::model::WifiObservation;
use geo::{Distance, Geodesic};
use macaddr::MacAddr6;

fn main() -> Result<(), &'static str> {
    // Example usage of the checked_point function and distance calculation
    let point_1 = checked_point(31.2304, 121.4737)?;
    let point_2 = checked_point(39.9042, 116.4074)?;

    let distance = Geodesic.distance(point_1, point_2);

    println!("Distance: {distance:.2} meters");

    // Example usage of the WifiObservation struct
    let parsed_bssid = "00:11:22:33:44:55".parse::<MacAddr6>();

    match parsed_bssid {
        Ok(bssid) => {
            let observation = WifiObservation {
                bssid,
                signal_dbm: Some(-55),
            };

            println!("Wi-Fi observation: {observation:?}");
        }
        Err(error) => {
            eprintln!("Invalid BSSID: {error}");
        }
    }

    Ok(())
}

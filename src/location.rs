use geo::Point;

pub fn checked_point(latitude: f64, longitude: f64) -> Result<Point<f64>, &'static str> {
    if !(-90.0..=90.0).contains(&latitude) {
        return Err("Latitude must be between -90 and 90 degrees.");
    }

    if !(-180.0..=180.0).contains(&longitude) {
        return Err("Longitude must be between -180 and 180 degrees.");
    }

    Ok(Point::new(longitude, latitude))
}

#[cfg(test)]
mod tests {
    use super::checked_point;

    #[test]
    fn accepts_valid_coordinate() -> Result<(), &'static str> {
        let point = checked_point(31.2304, 121.4737)?;

        assert_eq!(point.y(), 31.2304);
        assert_eq!(point.x(), 121.4737);

        Ok(())
    }

    #[test]
    fn rejects_invalid_latitude() {
        assert!(checked_point(91.0, 0.0).is_err());
        assert!(checked_point(-91.0, 0.0).is_err());
    }

    #[test]
    fn rejects_invalid_longitude() {
        assert!(checked_point(0.0, 181.0).is_err());
        assert!(checked_point(0.0, -181.0).is_err());
    }

    #[test]
    fn accepts_boundary_values() {
        assert!(checked_point(-90.0, -180.0).is_ok());
        assert!(checked_point(90.0, 180.0).is_ok());
    }

    #[test]
    fn rejects_non_finite_values() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(checked_point(value, 0.0).is_err());
            assert!(checked_point(0.0, value).is_err());
        }
    }
}

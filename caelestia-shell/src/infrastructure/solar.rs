//! Solar time and automatic day/night schedule calculations
//! Ported with 100% parity from `shell/utils/scripts/solartime.js`.

use std::f64::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolarTimes {
    pub sunrise_minutes: u32,
    pub sunset_minutes: u32,
}

pub struct SolarCalculator;

impl SolarCalculator {
    /// Parse "HH:MM" into minutes since midnight (0..1439), or None if invalid
    pub fn parse_time(str_val: &str) -> Option<u32> {
        let parts: Vec<&str> = str_val.trim().split(':').collect();
        if parts.len() != 2 {
            return None;
        }
        let h: u32 = parts[0].trim().parse().ok()?;
        let m: u32 = parts[1].trim().parse().ok()?;
        if h > 23 || m > 59 {
            return None;
        }
        Some(h * 60 + m)
    }

    /// Parse "lat,lon" coordinates string
    pub fn parse_coords(raw: &str) -> Option<Coordinates> {
        let parts: Vec<&str> = raw.trim().split(',').collect();
        if parts.len() != 2 {
            return None;
        }
        let lat: f64 = parts[0].trim().parse().ok()?;
        let lon: f64 = parts[1].trim().parse().ok()?;
        if !lat.is_finite() || !lon.is_finite() || lat.abs() > 90.0 || lon.abs() > 180.0 {
            return None;
        }
        Some(Coordinates { lat, lon })
    }

    /// NOAA low-precision solar position calculation
    /// Returns sunrise and sunset as minutes since local midnight (UTC approximate based on lon)
    pub fn calculate_solar_times(epoch_timestamp_secs: i64, coords: Coordinates) -> Option<SolarTimes> {
        let rad = PI / 180.0;
        // 2000-01-01 12:00:00 UTC epoch in seconds
        let epoch_2000_secs: f64 = 946728000.0;

        let days = ((epoch_timestamp_secs as f64 - epoch_2000_secs) / 86400.0).floor();
        let mean_solar_noon = days + 0.0008 - (coords.lon / 360.0);

        let anomaly = (357.5291 + 0.98560028 * mean_solar_noon).rem_euclid(360.0);
        let centre = 1.9148 * (anomaly * rad).sin()
            + 0.02 * (2.0 * anomaly * rad).sin()
            + 0.0003 * (3.0 * anomaly * rad).sin();
        let ecliptic_lon = (anomaly + centre + 180.0 + 102.9372).rem_euclid(360.0);

        let transit = mean_solar_noon + 0.0053 * (anomaly * rad).sin()
            - 0.0069 * (2.0 * ecliptic_lon * rad).sin();

        let sin_decl = (ecliptic_lon * rad).sin() * (23.44 * rad).sin();
        let cos_decl = (sin_decl.asin()).cos();

        // -0.833° accounts for atmospheric refraction and solar disc radius
        let cos_hour_angle = ((-0.833 * rad).sin() - (coords.lat * rad).sin() * sin_decl)
            / ((coords.lat * rad).cos() * cos_decl);

        if cos_hour_angle > 1.0 || cos_hour_angle < -1.0 {
            return None; // Polar day or night
        }

        let hour_angle = cos_hour_angle.acos() / rad;

        let sunrise_julian = transit - (hour_angle / 360.0);
        let sunset_julian = transit + (hour_angle / 360.0);

        // Convert julian fraction of day to local minutes (0..1439) with longitude offset
        let to_minutes = |julian: f64| -> u32 {
            let day_fraction = julian.fract();
            let utc_minutes = ((day_fraction + 0.5).rem_euclid(1.0) * 1440.0).round() as i64;
            // Approximate local offset by longitude (4 min per deg)
            let local_offset_mins = (coords.lon * 4.0).round() as i64;
            ((utc_minutes + local_offset_mins).rem_euclid(1440)) as u32
        };

        Some(SolarTimes {
            sunrise_minutes: to_minutes(sunrise_julian),
            sunset_minutes: to_minutes(sunset_julian),
        })
    }

    /// Whether current minutes falls in the light window
    pub fn is_light_at(minutes: u32, light_start: u32, dark_start: u32) -> bool {
        if light_start < dark_start {
            minutes >= light_start && minutes < dark_start
        } else {
            minutes >= light_start || minutes < dark_start
        }
    }
}

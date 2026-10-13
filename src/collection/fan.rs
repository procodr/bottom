//! Active Cooler tachometer readings from Linux's pwm-fan hwmon driver.

pub fn read_rpm() -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        return read_hwmon(std::path::Path::new("/sys/class/hwmon"));
    }
    #[cfg(not(target_os = "linux"))]
    None
}

#[cfg(any(target_os = "linux", test))]
fn read_hwmon(root: &std::path::Path) -> Option<u32> {
    use std::fs;
    for entry in root.read_dir().ok()?.flatten() {
        let path = entry.path();
        if fs::read_to_string(path.join("name"))
            .ok()
            .as_deref()
            .map(str::trim)
            == Some("pwmfan")
            && let Ok(reading) = fs::read_to_string(path.join("fan1_input"))
            && let Ok(rpm) = reading.trim().parse()
        {
            return Some(rpm);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn discovers_fan_without_assuming_hwmon_number() {
        let root = tempfile::tempdir().unwrap();
        let other = root.path().join("hwmon0");
        let fan = root.path().join("hwmon7");
        fs::create_dir(&other).unwrap();
        fs::create_dir(&fan).unwrap();
        fs::write(other.join("name"), "other\n").unwrap();
        fs::write(other.join("fan1_input"), "9999\n").unwrap();
        fs::write(fan.join("name"), "pwmfan\n").unwrap();
        for (reading, expected) in [
            ("3916\n", Some(3916)),
            ("0\n", Some(0)),
            ("-1", None),
            ("invalid", None),
        ] {
            fs::write(fan.join("fan1_input"), reading).unwrap();
            assert_eq!(read_hwmon(root.path()), expected);
        }
        fs::remove_file(fan.join("fan1_input")).unwrap();
        assert_eq!(read_hwmon(root.path()), None);
    }
}

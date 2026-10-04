//! Raspberry Pi 5 PMIC rail power, excluding loads connected directly to 5V.

pub fn read_power() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;

        let direct = Command::new("vcgencmd").arg("pmic_read_adc").output();
        let output = match direct {
            Ok(output) if output.status.success() => output,
            // Never prompt inside the TUI; use an existing sudo authorization.
            _ => Command::new("sudo")
                .args(["-n", "--", "vcgencmd", "pmic_read_adc"])
                .output()
                .ok()?,
        };
        if output.status.success() {
            return parse_power(std::str::from_utf8(&output.stdout).ok()?);
        }
    }
    None
}

#[cfg(any(target_os = "linux", test))]
fn parse_power(output: &str) -> Option<f64> {
    use std::collections::HashMap;
    let mut currents = HashMap::new();
    let mut voltages = HashMap::new();
    for line in output.lines() {
        let mut fields = line.split_whitespace();
        let Some(name) = fields.next() else { continue };
        let Some(reading) = fields.next() else {
            continue;
        };
        let Some((_, value)) = reading.split_once('=') else {
            continue;
        };
        let (rail, value, readings) = if let Some(rail) = name.strip_suffix("_A") {
            (rail, value.strip_suffix('A')?, &mut currents)
        } else if let Some(rail) = name.strip_suffix("_V") {
            (rail, value.strip_suffix('V')?, &mut voltages)
        } else {
            continue;
        };
        let value: f64 = value.parse().ok()?;
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        readings.insert(rail, value);
    }
    if currents.is_empty() {
        return None;
    }
    // Reject incomplete pairs instead of silently reporting too little power.
    let mut watts = 0.0;
    for (rail, current) in currents {
        watts += current * voltages.get(rail)?;
    }
    watts.is_finite().then_some(watts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_matching_rails_without_counting_input_voltage() {
        let data = " VDD_CORE_A current(7)=2.0A\n3V3_SYS_V volt(9)=3.3V\nVDD_CORE_V volt(15)=0.8V\n3V3_SYS_A current(1)=0.5A\nEXT5V_V volt(24)=5.1V\n";
        assert!((parse_power(data).unwrap() - 3.25).abs() < 1e-9);
    }

    #[test]
    fn missing_or_invalid_readings_are_unavailable() {
        for data in [
            "",
            "error=1",
            "CORE_A current(0)=1A",
            "CORE_A current(0)=NaNA\nCORE_V volt(1)=1V",
            "CORE_A current(0)=-1A\nCORE_V volt(1)=1V",
        ] {
            assert_eq!(parse_power(data), None);
        }
        assert_eq!(
            parse_power("CORE_A current(0)=0A\nCORE_V volt(1)=1V"),
            Some(0.0)
        );
    }
}

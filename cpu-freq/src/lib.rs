//! A crate to obtain CPU frequency information
//!
//! The crates attempts to obtain the information with as little permissions as
//! possible, in the fastest way possible, while still returning a result.
//!
//! ## Examples
//!
//! Fetch CPU frequency
//!
//! ```
//! let cpus = cpu_freq::get();
//! ```
#[derive(Clone, Debug)]
/// Struct for CPU frequencies, you can get the current frequency as well as the
/// minimum and maximum values available. When a value is not available it return None
pub struct CpuFreqs {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub cur: Option<f32>,
}

/// Returns cpu frequency
pub fn get() -> Vec<CpuFreqs> {
    get_cpu_freqs()
}

#[cfg(target_os = "linux")]
/// Return CPU frequency on Linux system. First
fn get_cpu_freqs() -> Vec<CpuFreqs> {
    use std::fs;
    use std::path::Path;

    fn get_path(num: usize) -> Option<String> {
        let policy_path = format!("/sys/devices/system/cpu/cpufreq/policy{}", num);
        let cpu_path = format!("/sys/devices/system/cpu/cpu{}/cpufreq", num);
        let base_path = match (
            Path::new(&policy_path).exists(),
            Path::new(&cpu_path).exists(),
        ) {
            (true, true) => Some(cpu_path),
            (true, false) => Some(policy_path),
            (false, true) => Some(cpu_path),
            (false, false) => None,
        };
        base_path
    }

    fn get_measurement(cpu: usize, measurement_file: &str) -> Option<f32> {
        let base_path = get_path(cpu);
        match base_path {
            Some(path) => {
                let val_ = fs::read_to_string(Path::new(&path).join(measurement_file));
                let val = match val_ {
                    Ok(val) => Some(val.trim().parse::<f32>().unwrap() / 1000.0),
                    Err(_) => None,
                };
                val
            }
            None => None,
        }
    }

    let mut res: Vec<CpuFreqs> = vec![];
    let num_cpus = num_cpus::get();
    if Path::new("/sys/devices/system/cpu/cpufreq").exists()
        || Path::new("/sys/devices/system/cpu/cpu0/cpufreq").exists()
    {
        for cpu in 0..num_cpus {
            let min = get_measurement(cpu, "scaling_min_freq");
            let max = get_measurement(cpu, "scaling_max_freq");
            let cur = get_measurement(cpu, "scaling_cur_freq");
            let r = CpuFreqs {
                min: min,
                max: max,
                cur: cur,
            };
            res.push(r);
        }
    } else {
        // Read from /proc/cpuinfo
        let data = fs::read_to_string("/proc/cpuinfo");
        match data {
            Ok(data) => {
                for line in data.lines() {
                    if line.to_lowercase().starts_with("cpu mhz") {
                        let fields: Vec<&str> = line.split_whitespace().collect();
                        // Expect 4 tokens - 'cpu', 'mhz', ':', <val>
                        let cur = Some(fields[3].parse::<f32>().unwrap());
                        res.push(CpuFreqs {
                            cur: cur,
                            min: None,
                            max: None,
                        })
                    }
                }
            }
            Err(_) => (),
        }
    }
    res
}

#[cfg(not(target_os = "linux"))]
fn get_cpu_freqs() -> Vec<CpuFreqs> {
    unimplemented!("cpu-freq is not yet supported on this system")
}

#[cfg(test)]
mod tests {
    use super::get;

    #[test]
    fn test_cpu_freq() {
        get().iter().for_each(|x| assert!(x.cur > Some(0.0)));
    }
}

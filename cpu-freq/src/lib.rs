#[derive(Clone, Debug)]
pub struct CpuFreqs {
    min: f32,
    max: f32,
    cur: f32,
}

/// Returns cpu frequency
pub fn cpu_freq() -> Vec<CpuFreqs> {
    get_cpu_freqs()
}

#[cfg(target_os = "linux")]
fn get_cpu_freq() -> Vec<CpuFreqs> {
    let res = CpuFreqs {
        min: 0.0,
        max: 0.0,
        cur: 0.0,
    };
    vec![res]
}

#[cfg(target_os = "linux")]
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

    fn get_measurement(cpu: usize, measurement_file: &str) -> f32 {
        let base_path = get_path(cpu);
        match base_path {
            Some(path) => {
                let val_ = fs::read_to_string(Path::new(&path).join(measurement_file));
                let val = match val_ {
                    Ok(val) => val.trim().parse::<f32>().unwrap() / 1000.0,
                    Err(_) => 0.0,
                };
                val
            }
            None => 0.0,
        }
    }

    let mut res: Vec<CpuFreqs> = vec![];
    let num_cpus = num_cpus::get();
    for cpu in 0..num_cpus {
        let min = get_measurement(cpu, "scaling_min_freq");
        let max = get_measurement(cpu, "scaling_max_freq");
        let curr = get_measurement(cpu, "scaling_cur_freq");
        let r = CpuFreqs {
            min: min,
            max: max,
            cur: curr,
        };
        res.push(r);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::cpu_freq;

    #[test]
    fn test_cpu_freq() {
        println!("CPU frequencies {:?}", cpu_freq());
    }
}

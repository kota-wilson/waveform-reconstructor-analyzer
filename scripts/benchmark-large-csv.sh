#!/usr/bin/env sh
set -eu

samples="${1:-100000}"
iterations="${2:-3}"
work_dir="target/wra-benchmark"
input_path="${work_dir}/large_square_wave_${samples}.csv"
config_path="${work_dir}/large_square_wave_${samples}.toml"

mkdir -p "${work_dir}"

awk -v samples="${samples}" 'BEGIN {
  print "time_s,signal_v"
  for (i = 0; i < samples; i++) {
    time = i / 1000.0
    value = (int(i / 50) % 2 == 0) ? 0.0 : 5.0
    printf "%.6f,%.1f\n", time, value
  }
}' > "${input_path}"

transition_count=$(( (samples - 1) / 50 ))

cat > "${config_path}" <<EOF
[input]
time_column = "time_s"
channels = ["signal_v"]
time_unit = "s"
signal_unit = "V"

[metadata]
test_run_id = "BENCH-LARGE-${samples}"
acquisition_notes = "Generated square-wave benchmark fixture with ${samples} samples."
environment = "local benchmark"
operator = "scripts/benchmark-large-csv.sh"

[[filters]]
type = "moving_average"
window_samples = 4

[[criteria]]
id = "signal_min_voltage"
type = "minimum_voltage"
channel = "signal_v"
threshold_v = 0.0

[[criteria]]
id = "signal_max_voltage"
type = "maximum_voltage"
channel = "signal_v"
threshold_v = 5.5

[[criteria]]
id = "signal_transition_count"
type = "state_transitions"
channel = "signal_v"
threshold_v = 2.5
expected_count = ${transition_count}
EOF

cargo build --quiet --bin wra-bench
target/debug/wra-bench --input "${input_path}" --config "${config_path}" --iterations "${iterations}"

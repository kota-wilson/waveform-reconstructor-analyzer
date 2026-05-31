use serde::Deserialize;

use crate::criteria::{Criterion, EdgeDirection, SignalState, TransientEventKind};
use crate::csv::CsvParseOptions;
use crate::error::{Result, WaveformError};
use crate::filter::{AdcQuantizer, FilterStep, LowPassFilter, MovingAverageFilter};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnalysisConfig {
    pub input: InputConfig,
    #[serde(default)]
    pub filters: Vec<FilterConfig>,
    #[serde(default)]
    pub criteria: Vec<CriterionConfig>,
}

impl AnalysisConfig {
    pub fn csv_options(&self) -> CsvParseOptions {
        CsvParseOptions::new(self.input.time_column.clone(), self.input.channels.clone())
    }

    pub fn criteria(&self) -> Result<Vec<Criterion>> {
        self.criteria
            .iter()
            .map(CriterionConfig::to_criterion)
            .collect()
    }

    pub fn filters(&self) -> Result<Vec<FilterStep>> {
        self.filters
            .iter()
            .map(FilterConfig::to_filter_step)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct InputConfig {
    pub time_column: String,
    pub channels: Vec<String>,
    #[serde(default = "default_time_unit")]
    pub time_unit: String,
    #[serde(default = "default_signal_unit")]
    pub signal_unit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct FilterConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub window_samples: Option<usize>,
    pub cutoff_hz: Option<f64>,
    pub bits: Option<u8>,
    pub min_v: Option<f64>,
    pub max_v: Option<f64>,
}

impl FilterConfig {
    fn to_filter_step(&self) -> Result<FilterStep> {
        match self.kind.as_str() {
            "moving_average" => Ok(FilterStep::MovingAverage(MovingAverageFilter {
                window_samples: self
                    .window_samples
                    .ok_or_else(|| missing_filter_field("window_samples"))?,
            })),
            "low_pass" => Ok(FilterStep::LowPass(LowPassFilter {
                cutoff_hz: self
                    .cutoff_hz
                    .ok_or_else(|| missing_filter_field("cutoff_hz"))?,
            })),
            "adc_quantize" => Ok(FilterStep::AdcQuantize(AdcQuantizer {
                bits: self.bits.ok_or_else(|| missing_filter_field("bits"))?,
                min_v: self.min_v.ok_or_else(|| missing_filter_field("min_v"))?,
                max_v: self.max_v.ok_or_else(|| missing_filter_field("max_v"))?,
            })),
            _ => Err(WaveformError::InvalidParameter {
                name: "filters.type".to_string(),
                reason: format!("unsupported filter type `{}`", self.kind),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionConfig {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub channel: String,
    pub threshold_v: Option<f64>,
    pub expected_count: Option<usize>,
    pub state: Option<String>,
    pub expected_state: Option<String>,
    pub event_kind: Option<String>,
    pub min_width_s: Option<f64>,
    pub max_width_s: Option<f64>,
    pub max_duration_s: Option<f64>,
    pub min_duration_s: Option<f64>,
    pub low_threshold_v: Option<f64>,
    pub high_threshold_v: Option<f64>,
    pub direction: Option<String>,
}

impl CriterionConfig {
    fn to_criterion(&self) -> Result<Criterion> {
        match self.kind.as_str() {
            "minimum_voltage" => Ok(Criterion::minimum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
            )),
            "maximum_voltage" => Ok(Criterion::maximum_voltage(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
            )),
            "state_transitions" => Ok(Criterion::state_transitions(
                self.id.clone(),
                self.channel.clone(),
                self.required_f64("threshold_v")?,
                self.expected_count
                    .ok_or_else(|| missing_field("expected_count"))?,
            )),
            "pulse_width" => Ok(Criterion::pulse_width(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("state")?,
                self.required_f64("threshold_v")?,
                self.min_width_s,
                self.max_width_s,
            )),
            "transient_duration" => Ok(Criterion::transient_duration(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("expected_state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("max_duration_s")?,
            )),
            "transient_event" => Ok(Criterion::transient_event(
                self.id.clone(),
                self.channel.clone(),
                self.transient_event_kind()?,
                self.required_state("expected_state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("max_duration_s")?,
            )),
            "stable_state_duration" => Ok(Criterion::stable_state_duration(
                self.id.clone(),
                self.channel.clone(),
                self.required_state("state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("min_duration_s")?,
            )),
            "rise_fall_time" => Ok(Criterion::rise_fall_time(
                self.id.clone(),
                self.channel.clone(),
                self.required_direction("direction")?,
                self.required_f64("low_threshold_v")?,
                self.required_f64("high_threshold_v")?,
                self.required_f64("max_duration_s")?,
            )),
            _ => Err(WaveformError::InvalidParameter {
                name: "criteria.type".to_string(),
                reason: format!("unsupported criterion type `{}`", self.kind),
            }),
        }
    }

    fn required_f64(&self, field: &str) -> Result<f64> {
        match field {
            "threshold_v" => self.threshold_v,
            "max_duration_s" => self.max_duration_s,
            "min_duration_s" => self.min_duration_s,
            "low_threshold_v" => self.low_threshold_v,
            "high_threshold_v" => self.high_threshold_v,
            _ => None,
        }
        .ok_or_else(|| missing_field(field))
    }

    fn required_state(&self, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            "expected_state" => self.expected_state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| missing_field(field))?;

        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }

    fn required_direction(&self, field: &str) -> Result<EdgeDirection> {
        let value = self
            .direction
            .as_deref()
            .ok_or_else(|| missing_field(field))?;

        EdgeDirection::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{field}"),
            reason: format!("expected `rise` or `fall`, got `{value}`"),
        })
    }

    fn transient_event_kind(&self) -> Result<TransientEventKind> {
        let value = self.event_kind.as_deref().unwrap_or("transient_event");

        TransientEventKind::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: "criteria.event_kind".to_string(),
            reason: format!(
                "expected transient_event, spurious_transition, contact_bounce, dropout, noise_induced_transition, or threshold_crossing_event; got `{value}`"
            ),
        })
    }
}

fn missing_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("criteria.{field}"),
        reason: "field is required for this criterion type".to_string(),
    }
}

fn missing_filter_field(field: &str) -> WaveformError {
    WaveformError::InvalidParameter {
        name: format!("filters.{field}"),
        reason: "field is required for this filter type".to_string(),
    }
}

fn default_time_unit() -> String {
    "s".to_string()
}

fn default_signal_unit() -> String {
    "V".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_config_to_csv_options_and_criteria() {
        let config = AnalysisConfig {
            input: InputConfig {
                time_column: "time".to_string(),
                channels: vec!["input_v".to_string()],
                time_unit: "s".to_string(),
                signal_unit: "V".to_string(),
            },
            filters: vec![FilterConfig {
                kind: "moving_average".to_string(),
                window_samples: Some(2),
                cutoff_hz: None,
                bits: None,
                min_v: None,
                max_v: None,
            }],
            criteria: vec![CriterionConfig {
                id: "max".to_string(),
                kind: "maximum_voltage".to_string(),
                channel: "input_v".to_string(),
                threshold_v: Some(5.5),
                expected_count: None,
                state: None,
                expected_state: None,
                event_kind: None,
                min_width_s: None,
                max_width_s: None,
                max_duration_s: None,
                min_duration_s: None,
                low_threshold_v: None,
                high_threshold_v: None,
                direction: None,
            }],
        };

        let options = config.csv_options();
        let filters = config.filters().expect("filters should convert");
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(options.time_column, "time");
        assert_eq!(options.channel_columns, vec!["input_v"]);
        assert_eq!(
            filters[0],
            FilterStep::MovingAverage(MovingAverageFilter { window_samples: 2 })
        );
        assert_eq!(criteria[0].id, "max");
    }

    #[test]
    fn converts_adc_quantizer_config_to_filter_step() {
        let config = FilterConfig {
            kind: "adc_quantize".to_string(),
            window_samples: None,
            cutoff_hz: None,
            bits: Some(12),
            min_v: Some(0.0),
            max_v: Some(5.0),
        };

        let filter = config.to_filter_step().expect("filter should convert");

        assert_eq!(
            filter,
            FilterStep::AdcQuantize(AdcQuantizer {
                bits: 12,
                min_v: 0.0,
                max_v: 5.0,
            })
        );
    }

    #[test]
    fn rejects_incomplete_adc_quantizer_config() {
        let config = FilterConfig {
            kind: "adc_quantize".to_string(),
            window_samples: None,
            cutoff_hz: None,
            bits: Some(12),
            min_v: Some(0.0),
            max_v: None,
        };

        let result = config.to_filter_step();

        assert_eq!(
            result,
            Err(WaveformError::InvalidParameter {
                name: "filters.max_v".to_string(),
                reason: "field is required for this filter type".to_string(),
            })
        );
    }
}

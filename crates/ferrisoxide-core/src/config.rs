use serde::Deserialize;

pub use crate::criteria::{CriterionMeasurementKind, CriterionOperator};

use crate::criteria::{
    Criterion, EdgeDirection, MeasurementRequirement, MeasurementSpec, ResponseLatencySpec,
    RunSelectionConfig, SignalState, TransientEventKind, TransientEventWindow,
};
use crate::csv::CsvParseOptions;
use crate::error::{Result, WaveformError};
use crate::filter::{AdcQuantizer, FilterStep, LowPassFilter, MovingAverageFilter};
use crate::model::{MetadataContext, TolerancePolicy, Unit};

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AnalysisConfig {
    pub input: InputConfig,
    #[serde(default)]
    pub metadata: MetadataContext,
    #[serde(default)]
    pub tolerances: TolerancePolicy,
    #[serde(default)]
    pub filters: Vec<FilterConfig>,
    #[serde(default)]
    pub criteria: Vec<CriterionConfig>,
}

impl AnalysisConfig {
    pub fn csv_options(&self) -> CsvParseOptions {
        let mut options =
            CsvParseOptions::new(self.input.time_column.clone(), self.input.channels.clone());
        options.time_unit = Unit::new(self.input.time_unit.clone());
        options.signal_unit = Unit::new(self.input.signal_unit.clone());
        options
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

    pub fn validate(&self) -> Result<()> {
        self.tolerances.validate()?;
        for criterion in &self.criteria {
            criterion.validate_schema()?;
        }
        Ok(())
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
    pub kind: Option<String>,
    pub channel: String,
    pub measurement: Option<CriterionMeasurementConfig>,
    pub requirement: Option<CriterionRequirementConfig>,
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
    pub source_channel: Option<String>,
    pub source_threshold_v: Option<f64>,
    pub target_threshold_v: Option<f64>,
    pub source_state: Option<String>,
    pub expected_target_state: Option<String>,
    pub max_latency_s: Option<f64>,
    pub start_time_s: Option<f64>,
    pub end_time_s: Option<f64>,
    pub arm_after_first_expected_state: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionMeasurementConfig {
    #[serde(rename = "type")]
    pub kind: String,
    pub threshold: Option<UnitValueConfig>,
    pub low_threshold: Option<UnitValueConfig>,
    pub high_threshold: Option<UnitValueConfig>,
    pub state: Option<String>,
    pub expected_state: Option<String>,
    pub event_kind: Option<String>,
    pub direction: Option<String>,
    pub selection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CriterionRequirementConfig {
    pub operator: Option<String>,
    pub value: Option<f64>,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct UnitValueConfig {
    pub value: f64,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriterionConfigShape {
    Legacy,
    Dsl,
}

impl CriterionConfig {
    pub fn shape(&self) -> Result<CriterionConfigShape> {
        self.validate_schema()
    }

    fn validate_schema(&self) -> Result<CriterionConfigShape> {
        let has_legacy_type = self.kind.is_some();
        let has_dsl_measurement = self.measurement.is_some();
        let has_dsl_requirement = self.requirement.is_some();
        let has_dsl = has_dsl_measurement || has_dsl_requirement;
        let has_legacy_fields = self.has_legacy_specific_fields();

        if has_legacy_type && has_dsl {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{}", self.id),
                reason:
                    "legacy criteria using `type` cannot include DSL `measurement` or `requirement` sections"
                        .to_string(),
            });
        }

        if has_dsl {
            if has_legacy_fields {
                return Err(WaveformError::InvalidParameter {
                    name: format!("criteria.{}", self.id),
                    reason:
                        "DSL criteria cannot include legacy fields such as `threshold_v`, `state`, `direction`, or duration limits"
                            .to_string(),
                });
            }

            if !(has_dsl_measurement && has_dsl_requirement) {
                return Err(WaveformError::InvalidParameter {
                    name: format!("criteria.{}", self.id),
                    reason: "DSL criteria require both `measurement` and `requirement` sections"
                        .to_string(),
                });
            }

            let measurement = self
                .measurement
                .as_ref()
                .expect("DSL measurement section validated");
            let requirement = self
                .requirement
                .as_ref()
                .expect("DSL requirement section validated");
            let measurement_kind = measurement.validate(&self.id)?;
            requirement.validate(&self.id, measurement_kind.requirement_unit())?;
            let operator = CriterionOperator::from_config(
                requirement
                    .operator
                    .as_deref()
                    .expect("DSL operator validated"),
            )
            .expect("DSL operator validated");
            measurement.to_measurement_spec(&self.id, measurement_kind, operator)?;

            return Ok(CriterionConfigShape::Dsl);
        }

        if has_legacy_type {
            let kind = self.kind.as_deref().expect("legacy type checked");
            self.validate_legacy_kind(kind)?;
            return Ok(CriterionConfigShape::Legacy);
        }

        if has_legacy_fields {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{}", self.id),
                reason: "legacy criteria require a `type` field".to_string(),
            });
        }

        Err(WaveformError::InvalidParameter {
            name: format!("criteria.{}", self.id),
            reason:
                "criterion must include either legacy `type` or DSL `measurement` and `requirement` sections"
                    .to_string(),
        })
    }

    fn to_criterion(&self) -> Result<Criterion> {
        let kind = match self.validate_schema()? {
            CriterionConfigShape::Legacy => self.kind.as_deref().expect("legacy type validated"),
            CriterionConfigShape::Dsl => return self.to_measurement_criterion(),
        };

        match kind {
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
            "transient_event" => Ok(Criterion::transient_event_window(
                self.id.clone(),
                self.channel.clone(),
                self.transient_event_kind()?,
                self.required_state("expected_state")?,
                self.required_f64("threshold_v")?,
                self.required_f64("max_duration_s")?,
                TransientEventWindow {
                    start_time_s: self.start_time_s,
                    end_time_s: self.end_time_s,
                    arm_after_first_expected_state: self
                        .arm_after_first_expected_state
                        .unwrap_or(false),
                },
            )),
            "response_latency" | "state_transition_response" => Ok(Criterion::response_latency(
                self.id.clone(),
                ResponseLatencySpec {
                    source_channel: self.required_string("source_channel")?,
                    target_channel: self.channel.clone(),
                    source_threshold_v: self.required_f64("source_threshold_v")?,
                    target_threshold_v: self.required_f64("target_threshold_v")?,
                    source_state: self.required_state("source_state")?,
                    expected_target_state: self.required_state("expected_target_state")?,
                    max_latency_s: self.required_f64("max_latency_s")?,
                },
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
                reason: format!("unsupported criterion type `{kind}`"),
            }),
        }
    }

    fn validate_legacy_kind(&self, kind: &str) -> Result<()> {
        match kind {
            "minimum_voltage" | "maximum_voltage" => {
                self.validate_required_finite("threshold_v")?;
            }
            "state_transitions" => {
                self.validate_required_finite("threshold_v")?;
                self.expected_count
                    .ok_or_else(|| missing_field("expected_count"))?;
            }
            "pulse_width" => {
                self.required_state("state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_optional_non_negative("min_width_s")?;
                self.validate_optional_non_negative("max_width_s")?;
                if self.min_width_s.is_none() && self.max_width_s.is_none() {
                    return Err(WaveformError::InvalidParameter {
                        name: "criteria.pulse_width".to_string(),
                        reason: "min_width_s or max_width_s is required".to_string(),
                    });
                }
            }
            "transient_duration" => {
                self.required_state("expected_state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("max_duration_s")?;
            }
            "transient_event" => {
                self.transient_event_kind()?;
                self.required_state("expected_state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("max_duration_s")?;
                self.validate_time_window()?;
            }
            "stable_state_duration" => {
                self.required_state("state")?;
                self.validate_required_finite("threshold_v")?;
                self.validate_required_non_negative("min_duration_s")?;
            }
            "rise_fall_time" => {
                self.required_direction("direction")?;
                let low_threshold_v = self.validate_required_finite("low_threshold_v")?;
                let high_threshold_v = self.validate_required_finite("high_threshold_v")?;
                if low_threshold_v >= high_threshold_v {
                    return Err(WaveformError::InvalidParameter {
                        name: "criteria.low_threshold_v".to_string(),
                        reason: "must be lower than high_threshold_v".to_string(),
                    });
                }
                self.validate_required_non_negative("max_duration_s")?;
            }
            "response_latency" | "state_transition_response" => {
                self.required_string("source_channel")?;
                self.required_state("source_state")?;
                self.required_state("expected_target_state")?;
                self.validate_required_finite("source_threshold_v")?;
                self.validate_required_finite("target_threshold_v")?;
                self.validate_required_non_negative("max_latency_s")?;
            }
            _ => {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.type".to_string(),
                    reason: format!("unsupported criterion type `{kind}`"),
                });
            }
        }

        Ok(())
    }

    fn validate_required_finite(&self, field: &str) -> Result<f64> {
        let value = self.required_f64(field)?;
        if value.is_finite() {
            Ok(value)
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be finite".to_string(),
            })
        }
    }

    fn validate_required_non_negative(&self, field: &str) -> Result<f64> {
        let value = self.validate_required_finite(field)?;
        if value >= 0.0 {
            Ok(value)
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be non-negative".to_string(),
            })
        }
    }

    fn validate_optional_non_negative(&self, field: &str) -> Result<()> {
        let value = match field {
            "min_width_s" => self.min_width_s,
            "max_width_s" => self.max_width_s,
            _ => None,
        };
        let Some(value) = value else {
            return Ok(());
        };

        if value.is_finite() && value >= 0.0 {
            Ok(())
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{field}"),
                reason: "must be a finite non-negative value".to_string(),
            })
        }
    }

    fn validate_time_window(&self) -> Result<()> {
        if let Some(start_time_s) = self.start_time_s {
            if !start_time_s.is_finite() || start_time_s < 0.0 {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.start_time_s".to_string(),
                    reason: "must be a finite non-negative value".to_string(),
                });
            }
        }
        if let Some(end_time_s) = self.end_time_s {
            if !end_time_s.is_finite() || end_time_s < 0.0 {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.end_time_s".to_string(),
                    reason: "must be a finite non-negative value".to_string(),
                });
            }
        }
        if let (Some(start_time_s), Some(end_time_s)) = (self.start_time_s, self.end_time_s) {
            if end_time_s < start_time_s {
                return Err(WaveformError::InvalidParameter {
                    name: "criteria.end_time_s".to_string(),
                    reason: "must be greater than or equal to start_time_s".to_string(),
                });
            }
        }

        Ok(())
    }

    fn to_measurement_criterion(&self) -> Result<Criterion> {
        let measurement = self
            .measurement
            .as_ref()
            .expect("DSL measurement section validated");
        let requirement = self
            .requirement
            .as_ref()
            .expect("DSL requirement section validated");
        let measurement_kind = CriterionMeasurementKind::from_config(&measurement.kind)
            .expect("DSL measurement kind validated");
        let operator = CriterionOperator::from_config(
            requirement
                .operator
                .as_deref()
                .expect("DSL operator validated"),
        )
        .expect("DSL operator validated");

        let measurement = measurement.to_measurement_spec(&self.id, measurement_kind, operator)?;
        let requirement = MeasurementRequirement {
            operator,
            value: requirement.value.expect("DSL requirement value validated"),
        };

        Ok(Criterion::measurement(
            self.id.clone(),
            self.channel.clone(),
            measurement,
            requirement,
        ))
    }

    fn has_legacy_specific_fields(&self) -> bool {
        self.threshold_v.is_some()
            || self.expected_count.is_some()
            || self.state.is_some()
            || self.expected_state.is_some()
            || self.event_kind.is_some()
            || self.min_width_s.is_some()
            || self.max_width_s.is_some()
            || self.max_duration_s.is_some()
            || self.min_duration_s.is_some()
            || self.low_threshold_v.is_some()
            || self.high_threshold_v.is_some()
            || self.direction.is_some()
            || self.source_channel.is_some()
            || self.source_threshold_v.is_some()
            || self.target_threshold_v.is_some()
            || self.source_state.is_some()
            || self.expected_target_state.is_some()
            || self.max_latency_s.is_some()
            || self.start_time_s.is_some()
            || self.end_time_s.is_some()
            || self.arm_after_first_expected_state.is_some()
    }

    fn required_f64(&self, field: &str) -> Result<f64> {
        match field {
            "threshold_v" => self.threshold_v,
            "max_duration_s" => self.max_duration_s,
            "min_duration_s" => self.min_duration_s,
            "low_threshold_v" => self.low_threshold_v,
            "high_threshold_v" => self.high_threshold_v,
            "source_threshold_v" => self.source_threshold_v,
            "target_threshold_v" => self.target_threshold_v,
            "max_latency_s" => self.max_latency_s,
            _ => None,
        }
        .ok_or_else(|| missing_field(field))
    }

    fn required_string(&self, field: &str) -> Result<String> {
        match field {
            "source_channel" => self.source_channel.clone(),
            _ => None,
        }
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| missing_field(field))
    }

    fn required_state(&self, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            "expected_state" => self.expected_state.as_deref(),
            "source_state" => self.source_state.as_deref(),
            "expected_target_state" => self.expected_target_state.as_deref(),
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

impl CriterionMeasurementConfig {
    fn validate(&self, criterion_id: &str) -> Result<CriterionMeasurementKind> {
        let measurement_kind =
            CriterionMeasurementKind::from_config(&self.kind).ok_or_else(|| {
                WaveformError::InvalidParameter {
                    name: format!("criteria.{criterion_id}.measurement.type"),
                    reason: format!(
                        "unsupported measurement type `{}`; expected minimum_sample, maximum_sample, state_transition_count, pulse_width, stable_state_duration, transient_event_duration, rise_time, or fall_time",
                        self.kind
                    ),
                }
            })?;

        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.threshold"),
            self.threshold.as_ref(),
            "V",
        )?;
        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.low_threshold"),
            self.low_threshold.as_ref(),
            "V",
        )?;
        validate_optional_unit_value(
            &format!("criteria.{criterion_id}.measurement.high_threshold"),
            self.high_threshold.as_ref(),
            "V",
        )?;

        Ok(measurement_kind)
    }

    fn to_measurement_spec(
        &self,
        criterion_id: &str,
        measurement_kind: CriterionMeasurementKind,
        operator: CriterionOperator,
    ) -> Result<MeasurementSpec> {
        match measurement_kind {
            CriterionMeasurementKind::MinimumSample => Ok(MeasurementSpec::MinimumSample),
            CriterionMeasurementKind::MaximumSample => Ok(MeasurementSpec::MaximumSample),
            CriterionMeasurementKind::StateTransitionCount => {
                Ok(MeasurementSpec::StateTransitionCount {
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::PulseWidth => Ok(MeasurementSpec::PulseWidth {
                state: self.required_state(criterion_id, "state")?,
                threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                selection: self.pulse_width_selection(criterion_id, operator)?,
            }),
            CriterionMeasurementKind::StableStateDuration => {
                self.validate_optional_selection(criterion_id, "longest")?;
                Ok(MeasurementSpec::StableStateDuration {
                    state: self.required_state(criterion_id, "state")?,
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::TransientEventDuration => {
                self.validate_optional_selection(criterion_id, "longest")?;
                Ok(MeasurementSpec::TransientEventDuration {
                    event_kind: self.transient_event_kind(criterion_id)?,
                    expected_state: self.required_state(criterion_id, "expected_state")?,
                    threshold_v: self.required_threshold_value(criterion_id, "threshold")?,
                })
            }
            CriterionMeasurementKind::RiseTime => {
                let (low_threshold_v, high_threshold_v) =
                    self.validated_edge_thresholds(criterion_id)?;
                Ok(MeasurementSpec::RiseTime {
                    low_threshold_v,
                    high_threshold_v,
                })
            }
            CriterionMeasurementKind::FallTime => {
                let (low_threshold_v, high_threshold_v) =
                    self.validated_edge_thresholds(criterion_id)?;
                Ok(MeasurementSpec::FallTime {
                    low_threshold_v,
                    high_threshold_v,
                })
            }
        }
    }

    fn required_threshold_value(&self, criterion_id: &str, field: &str) -> Result<f64> {
        let value = match field {
            "threshold" => self.threshold.as_ref(),
            "low_threshold" => self.low_threshold.as_ref(),
            "high_threshold" => self.high_threshold.as_ref(),
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: "field is required for this measurement type".to_string(),
        })?;

        Ok(value.value)
    }

    fn validated_edge_thresholds(&self, criterion_id: &str) -> Result<(f64, f64)> {
        let low_threshold_v = self.required_threshold_value(criterion_id, "low_threshold")?;
        let high_threshold_v = self.required_threshold_value(criterion_id, "high_threshold")?;
        if low_threshold_v >= high_threshold_v {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.low_threshold"),
                reason: "must be lower than high_threshold".to_string(),
            });
        }

        Ok((low_threshold_v, high_threshold_v))
    }

    fn required_state(&self, criterion_id: &str, field: &str) -> Result<SignalState> {
        let value = match field {
            "state" => self.state.as_deref(),
            "expected_state" => self.expected_state.as_deref(),
            _ => None,
        }
        .ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: "field is required for this measurement type".to_string(),
        })?;

        SignalState::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.{field}"),
            reason: format!("expected `high` or `low`, got `{value}`"),
        })
    }

    fn pulse_width_selection(
        &self,
        criterion_id: &str,
        operator: CriterionOperator,
    ) -> Result<RunSelectionConfig> {
        if let Some(selection) = self.selection.as_deref() {
            return RunSelectionConfig::from_config(selection).ok_or_else(|| {
                WaveformError::InvalidParameter {
                    name: format!("criteria.{criterion_id}.measurement.selection"),
                    reason: format!("expected `shortest` or `longest`, got `{selection}`"),
                }
            });
        }

        match operator {
            CriterionOperator::GreaterThan | CriterionOperator::GreaterThanOrEqual => {
                Ok(RunSelectionConfig::Shortest)
            }
            CriterionOperator::LessThan | CriterionOperator::LessThanOrEqual => {
                Ok(RunSelectionConfig::Longest)
            }
            CriterionOperator::EqualTo => Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.selection"),
                reason:
                    "field is required for equal_to pulse_width criteria; use `shortest` or `longest`"
                        .to_string(),
            }),
        }
    }

    fn validate_optional_selection(&self, criterion_id: &str, expected: &str) -> Result<()> {
        let Some(selection) = self.selection.as_deref() else {
            return Ok(());
        };

        if selection == expected {
            Ok(())
        } else {
            Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.measurement.selection"),
                reason: format!("expected `{expected}`, got `{selection}`"),
            })
        }
    }

    fn transient_event_kind(&self, criterion_id: &str) -> Result<TransientEventKind> {
        let value = self.event_kind.as_deref().unwrap_or("transient_event");

        TransientEventKind::from_config(value).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.measurement.event_kind"),
            reason: format!(
                "expected transient_event, spurious_transition, contact_bounce, dropout, noise_induced_transition, or threshold_crossing_event; got `{value}`"
            ),
        })
    }
}

impl CriterionRequirementConfig {
    fn validate(&self, criterion_id: &str, expected_unit: &str) -> Result<()> {
        let operator = self
            .operator
            .as_deref()
            .ok_or_else(|| WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.requirement.operator"),
                reason: "field is required for DSL requirements".to_string(),
            })?;

        CriterionOperator::from_config(operator).ok_or_else(|| WaveformError::InvalidParameter {
            name: format!("criteria.{criterion_id}.requirement.operator"),
            reason: format!(
                "unsupported operator `{operator}`; expected less_than, less_than_or_equal, greater_than, greater_than_or_equal, or equal_to"
            ),
        })?;

        if self.value.is_none() {
            return Err(WaveformError::InvalidParameter {
                name: format!("criteria.{criterion_id}.requirement.value"),
                reason: "field is required for DSL requirements".to_string(),
            });
        }

        let unit = required_unit(
            &format!("criteria.{criterion_id}.requirement.unit"),
            self.unit.as_deref(),
        )?;
        validate_supported_unit(&format!("criteria.{criterion_id}.requirement.unit"), unit)?;
        validate_expected_unit(
            &format!("criteria.{criterion_id}.requirement.unit"),
            unit,
            expected_unit,
        )
    }
}

fn validate_optional_unit_value(
    field: &str,
    value: Option<&UnitValueConfig>,
    expected_unit: &str,
) -> Result<()> {
    let Some(value) = value else {
        return Ok(());
    };
    let unit = required_unit(&format!("{field}.unit"), value.unit.as_deref())?;
    validate_supported_unit(&format!("{field}.unit"), unit)?;
    validate_expected_unit(&format!("{field}.unit"), unit, expected_unit)
}

fn required_unit<'a>(name: &str, unit: Option<&'a str>) -> Result<&'a str> {
    unit.ok_or_else(|| WaveformError::InvalidParameter {
        name: name.to_string(),
        reason: "explicit unit field is required".to_string(),
    })
}

fn validate_supported_unit(name: &str, unit: &str) -> Result<()> {
    match unit {
        "V" | "s" | "count" => Ok(()),
        _ => Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: format!("unsupported unit `{unit}`; expected `V`, `s`, or `count`"),
        }),
    }
}

fn validate_expected_unit(name: &str, unit: &str, expected_unit: &str) -> Result<()> {
    if unit == expected_unit {
        Ok(())
    } else {
        Err(WaveformError::InvalidParameter {
            name: name.to_string(),
            reason: format!("expected unit `{expected_unit}`, got `{unit}`"),
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
            metadata: MetadataContext::default(),
            tolerances: TolerancePolicy::default(),
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
                kind: Some("maximum_voltage".to_string()),
                channel: "input_v".to_string(),
                measurement: None,
                requirement: None,
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
                source_channel: None,
                source_threshold_v: None,
                target_threshold_v: None,
                source_state: None,
                expected_target_state: None,
                max_latency_s: None,
                start_time_s: None,
                end_time_s: None,
                arm_after_first_expected_state: None,
            }],
        };

        let options = config.csv_options();
        let filters = config.filters().expect("filters should convert");
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(options.time_column, "time");
        assert_eq!(options.channel_columns, vec!["input_v"]);
        assert_eq!(options.time_unit, Unit::seconds());
        assert_eq!(options.signal_unit, Unit::volts());
        assert_eq!(config.tolerances, TolerancePolicy::default());
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
    fn legacy_filter_config_covers_current_transform_types() {
        let config: AnalysisConfig = toml::from_str(
            r#"
[input]
time_column = "time"
channels = ["input_v"]

[[filters]]
type = "moving_average"
window_samples = 2

[[filters]]
type = "low_pass"
cutoff_hz = 25.0

[[filters]]
type = "adc_quantize"
bits = 12
min_v = 0.0
max_v = 5.0

[[criteria]]
id = "input_max"
type = "maximum_voltage"
channel = "input_v"
threshold_v = 5.5
"#,
        )
        .expect("legacy filter config should deserialize");

        let filters = config.filters().expect("filters should convert");

        assert_eq!(
            filters,
            vec![
                FilterStep::MovingAverage(MovingAverageFilter { window_samples: 2 }),
                FilterStep::LowPass(LowPassFilter { cutoff_hz: 25.0 }),
                FilterStep::AdcQuantize(AdcQuantizer {
                    bits: 12,
                    min_v: 0.0,
                    max_v: 5.0,
                }),
            ]
        );
    }

    #[test]
    fn rejects_invalid_tolerance_config() {
        let config = AnalysisConfig {
            input: InputConfig {
                time_column: "time".to_string(),
                channels: vec!["input_v".to_string()],
                time_unit: "s".to_string(),
                signal_unit: "V".to_string(),
            },
            metadata: MetadataContext::default(),
            tolerances: TolerancePolicy {
                voltage_v: 0.0,
                time_s: -0.001,
            },
            filters: Vec::new(),
            criteria: Vec::new(),
        };

        assert!(matches!(
            config.validate(),
            Err(WaveformError::InvalidParameter { .. })
        ));
    }

    #[test]
    fn deserializes_legacy_and_dsl_criteria_side_by_side() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "legacy_max"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.0

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(config.criteria.len(), 2);
        assert_eq!(config.criteria[0].shape(), Ok(CriterionConfigShape::Legacy));
        assert_eq!(config.criteria[1].shape(), Ok(CriterionConfigShape::Dsl));
        assert_eq!(
            config.criteria[1].measurement.as_ref().map(|measurement| {
                (
                    measurement.kind.as_str(),
                    measurement
                        .low_threshold
                        .as_ref()
                        .map(|threshold| (threshold.value, threshold.unit.as_deref())),
                    measurement
                        .high_threshold
                        .as_ref()
                        .map(|threshold| (threshold.value, threshold.unit.as_deref())),
                )
            }),
            Some(("rise_time", Some((0.5, Some("V"))), Some((4.5, Some("V")))))
        );
        assert!(config.validate().is_ok());
    }

    #[test]
    fn validates_supported_dsl_operators_and_requirement_units() {
        for (operator, measurement_section, unit) in [
            ("less_than", "type = \"maximum_sample\"", "V"),
            (
                "less_than_or_equal",
                "type = \"rise_time\"\nlow_threshold = { value = 0.5, unit = \"V\" }\nhigh_threshold = { value = 4.5, unit = \"V\" }",
                "s",
            ),
            (
                "greater_than",
                "type = \"stable_state_duration\"\nthreshold = { value = 2.5, unit = \"V\" }\nstate = \"high\"",
                "s",
            ),
            (
                "greater_than_or_equal",
                "type = \"state_transition_count\"\nthreshold = { value = 2.5, unit = \"V\" }",
                "count",
            ),
            (
                "equal_to",
                "type = \"fall_time\"\nlow_threshold = { value = 0.5, unit = \"V\" }\nhigh_threshold = { value = 4.5, unit = \"V\" }",
                "s",
            ),
        ] {
            let toml = format!(
                r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_check"
channel = "switch_v"

[criteria.measurement]
{measurement_section}

[criteria.requirement]
operator = "{operator}"
value = 1.0
unit = "{unit}"
"#
            );

            let config =
                toml::from_str::<AnalysisConfig>(&toml).expect("config should deserialize");

            assert_eq!(config.validate(), Ok(()), "{operator} should validate");
        }
    }

    #[test]
    fn rejects_unknown_dsl_operator() {
        let config = dsl_config_with_requirement("approximately", 0.005, Some("s"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.operator".to_string(),
                reason:
                    "unsupported operator `approximately`; expected less_than, less_than_or_equal, greater_than, greater_than_or_equal, or equal_to"
                        .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 0.005, None);

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "explicit unit field is required".to_string(),
            })
        );
    }

    #[test]
    fn rejects_unsupported_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 5.0, Some("ms"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "unsupported unit `ms`; expected `V`, `s`, or `count`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_mismatched_dsl_requirement_unit() {
        let config = dsl_config_with_requirement("less_than_or_equal", 0.005, Some("V"));

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.requirement.unit".to_string(),
                reason: "expected unit `s`, got `V`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_threshold_unit() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5 }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.measurement.low_threshold.unit".to_string(),
                reason: "explicit unit field is required".to_string(),
            })
        );
    }

    #[test]
    fn rejects_mismatched_dsl_threshold_unit() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "s" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.dsl_rise.measurement.low_threshold.unit".to_string(),
                reason: "expected unit `V`, got `s`".to_string(),
            })
        );
    }

    #[test]
    fn rejects_unit_shorthand_strings() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = "5ms"
unit = "s"
"#;

        assert!(toml::from_str::<AnalysisConfig>(toml).is_err());
    }

    #[test]
    fn rejects_ambiguous_mixed_legacy_and_dsl_criterion() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "mixed"
type = "maximum_voltage"
channel = "switch_v"
threshold_v = 5.0

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
value = 5.0
unit = "V"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.mixed".to_string(),
                reason:
                    "legacy criteria using `type` cannot include DSL `measurement` or `requirement` sections"
                        .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_measurement_or_requirement_sections() {
        let missing_measurement = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_measurement"
channel = "switch_v"

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let missing_requirement = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_requirement"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            missing_measurement.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_measurement".to_string(),
                reason: "DSL criteria require both `measurement` and `requirement` sections"
                    .to_string(),
            })
        );
        assert_eq!(
            missing_requirement.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_requirement".to_string(),
                reason: "DSL criteria require both `measurement` and `requirement` sections"
                    .to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_requirement_value() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_value"
channel = "switch_v"

[criteria.measurement]
type = "maximum_sample"

[criteria.requirement]
operator = "less_than_or_equal"
unit = "V"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_value.requirement.value".to_string(),
                reason: "field is required for DSL requirements".to_string(),
            })
        );
    }

    #[test]
    fn rejects_missing_dsl_measurement_threshold() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_threshold"
channel = "switch_v"

[criteria.measurement]
type = "state_transition_count"

[criteria.requirement]
operator = "equal_to"
value = 2
unit = "count"
"#;
        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        assert_eq!(
            config.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_threshold.measurement.threshold".to_string(),
                reason: "field is required for this measurement type".to_string(),
            })
        );
    }

    #[test]
    fn rejects_incompatible_dsl_measurement_parameters() {
        let invalid_state = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "bad_state"
channel = "switch_v"

[criteria.measurement]
type = "pulse_width"
threshold = { value = 2.5, unit = "V" }
state = "on"

[criteria.requirement]
operator = "greater_than_or_equal"
value = 0.001
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let missing_selection = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "missing_selection"
channel = "switch_v"

[criteria.measurement]
type = "pulse_width"
threshold = { value = 2.5, unit = "V" }
state = "high"

[criteria.requirement]
operator = "equal_to"
value = 0.001
unit = "s"
"#,
        )
        .expect("config should deserialize");
        let inverted_edge = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "bad_edge"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 4.5, unit = "V" }
high_threshold = { value = 0.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            invalid_state.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.bad_state.measurement.state".to_string(),
                reason: "expected `high` or `low`, got `on`".to_string(),
            })
        );
        assert_eq!(
            missing_selection.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.missing_selection.measurement.selection".to_string(),
                reason:
                    "field is required for equal_to pulse_width criteria; use `shortest` or `longest`"
                        .to_string(),
            })
        );
        assert_eq!(
            inverted_edge.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.bad_edge.measurement.low_threshold".to_string(),
                reason: "must be lower than high_threshold".to_string(),
            })
        );
    }

    #[test]
    fn converts_dsl_criteria_to_measurement_runtime_criteria() {
        let toml = r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = { value = 0.5, unit = "V" }
high_threshold = { value = 4.5, unit = "V" }

[criteria.requirement]
operator = "less_than_or_equal"
value = 0.005
unit = "s"
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");

        let criteria = config.criteria().expect("DSL criteria should convert");

        assert_eq!(criteria.len(), 1);
        assert!(matches!(
            &criteria[0].check,
            crate::criteria::CriterionCheck::Measurement {
                measurement: MeasurementSpec::RiseTime {
                    low_threshold_v: 0.5,
                    high_threshold_v: 4.5,
                },
                requirement: MeasurementRequirement {
                    operator: CriterionOperator::LessThanOrEqual,
                    value: 0.005,
                },
                ..
            }
        ));
    }

    #[test]
    fn converts_response_latency_and_windowed_transient_criteria() {
        let toml = r#"
[input]
time_column = "time_s"
channels = ["command_v", "feedback_v"]

[[criteria]]
id = "response_latency"
type = "response_latency"
channel = "feedback_v"
source_channel = "command_v"
source_threshold_v = 2.5
target_threshold_v = 2.5
source_state = "high"
expected_target_state = "high"
max_latency_s = 0.050

[[criteria]]
id = "windowed_transient"
type = "transient_event"
channel = "feedback_v"
event_kind = "transient_event"
expected_state = "high"
threshold_v = 2.5
max_duration_s = 0.001
start_time_s = 1.020
arm_after_first_expected_state = true
"#;

        let config = toml::from_str::<AnalysisConfig>(toml).expect("config should deserialize");
        let criteria = config.criteria().expect("criteria should convert");

        assert_eq!(criteria.len(), 2);
        assert!(matches!(
            &criteria[0].check,
            crate::criteria::CriterionCheck::ResponseLatency {
                source_channel,
                target_channel,
                max_latency_s,
                ..
            } if source_channel == "command_v"
                && target_channel == "feedback_v"
                && (*max_latency_s - 0.050).abs() < f64::EPSILON
        ));
        assert!(matches!(
            &criteria[1].check,
            crate::criteria::CriterionCheck::TransientEvent {
                start_time_s: Some(1.020),
                end_time_s: None,
                arm_after_first_expected_state: true,
                ..
            }
        ));
    }

    #[test]
    fn rejects_invalid_response_latency_and_transient_window_config() {
        let missing_source = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time_s"
channels = ["command_v", "feedback_v"]

[[criteria]]
id = "missing_source"
type = "response_latency"
channel = "feedback_v"
source_threshold_v = 2.5
target_threshold_v = 2.5
source_state = "high"
expected_target_state = "high"
max_latency_s = 0.050
"#,
        )
        .expect("config should deserialize");
        let inverted_window = toml::from_str::<AnalysisConfig>(
            r#"
[input]
time_column = "time_s"
channels = ["feedback_v"]

[[criteria]]
id = "bad_window"
type = "transient_event"
channel = "feedback_v"
expected_state = "high"
threshold_v = 2.5
max_duration_s = 0.001
start_time_s = 1.020
end_time_s = 1.000
"#,
        )
        .expect("config should deserialize");

        assert_eq!(
            missing_source.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.source_channel".to_string(),
                reason: "field is required for this criterion type".to_string(),
            })
        );
        assert_eq!(
            inverted_window.validate(),
            Err(WaveformError::InvalidParameter {
                name: "criteria.end_time_s".to_string(),
                reason: "must be greater than or equal to start_time_s".to_string(),
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

    fn dsl_config_with_requirement(
        operator: &str,
        value: f64,
        unit: Option<&str>,
    ) -> AnalysisConfig {
        let unit_line = unit
            .map(|unit| format!("unit = \"{unit}\""))
            .unwrap_or_default();
        let toml = format!(
            r#"
[input]
time_column = "time"
channels = ["switch_v"]

[[criteria]]
id = "dsl_rise"
channel = "switch_v"

[criteria.measurement]
type = "rise_time"
low_threshold = {{ value = 0.5, unit = "V" }}
high_threshold = {{ value = 4.5, unit = "V" }}

[criteria.requirement]
operator = "{operator}"
value = {value}
{unit_line}
"#
        );

        toml::from_str::<AnalysisConfig>(&toml).expect("config should deserialize")
    }
}

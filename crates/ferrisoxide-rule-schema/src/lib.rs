//! Versioned portable FerrisOxide Rule Package schema.
//!
//! This crate owns data structures only. It does not evaluate rules, parse CSV,
//! render reports, export deployment packages, compute checksums, or bind any
//! controller, DAQ, RTOS, SDK, or hardware HAL.

use serde::{Deserialize, Serialize};

pub const CURRENT_SCHEMA_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RulePackage {
    pub package: PackageMetadata,
    pub target: TargetProfile,
    pub sample_timing: SampleTimingAssumption,
    pub channels: Vec<ChannelDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub filters: Vec<FilterDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub criteria: Vec<CriterionDefinition>,
}

impl RulePackage {
    pub fn new(
        package: PackageMetadata,
        target: TargetProfile,
        sample_timing: SampleTimingAssumption,
        channels: Vec<ChannelDefinition>,
        criteria: Vec<CriterionDefinition>,
    ) -> Self {
        Self {
            package,
            target,
            sample_timing,
            channels,
            filters: Vec::new(),
            criteria,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMetadata {
    pub name: String,
    pub version: String,
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl PackageMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            schema_version: CURRENT_SCHEMA_VERSION.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetProfile {
    pub kind: TargetProfileKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

impl TargetProfile {
    pub fn new(kind: TargetProfileKind) -> Self {
        Self {
            kind,
            identifier: None,
            notes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetProfileKind {
    DesktopAuthoring,
    EmbeddedRuntime,
    ControllerRuntime,
    TestVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SampleTimingAssumption {
    pub timestamp_unit: EngineeringUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_sample_rate_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate_tolerance_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nominal_sample_interval_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_tolerance_s: Option<f64>,
}

impl SampleTimingAssumption {
    pub const fn seconds_at_hz(nominal_sample_rate_hz: f64) -> Self {
        Self {
            timestamp_unit: EngineeringUnit::Second,
            nominal_sample_rate_hz: Some(nominal_sample_rate_hz),
            sample_rate_tolerance_hz: None,
            nominal_sample_interval_s: None,
            timestamp_tolerance_s: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelDefinition {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    pub unit: EngineeringUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_rate_hz: Option<f64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thresholds: Vec<ThresholdDefinition>,
}

impl ChannelDefinition {
    pub fn new(name: impl Into<String>, unit: EngineeringUnit) -> Self {
        Self {
            name: name.into(),
            source_name: None,
            unit,
            sample_rate_hz: None,
            thresholds: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThresholdDefinition {
    pub name: String,
    pub role: ThresholdRole,
    pub value: UnitValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdRole {
    Low,
    High,
    Decision,
    Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UnitValue {
    pub value: f64,
    pub unit: EngineeringUnit,
}

impl UnitValue {
    pub const fn new(value: f64, unit: EngineeringUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EngineeringUnit {
    #[serde(rename = "V")]
    Volt,
    #[serde(rename = "s")]
    Second,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "sample")]
    Sample,
    #[serde(rename = "Hz")]
    Hertz,
}

impl EngineeringUnit {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Volt => "V",
            Self::Second => "s",
            Self::Count => "count",
            Self::Sample => "sample",
            Self::Hertz => "Hz",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FilterDefinition {
    MovingAverage {
        id: String,
        channel: String,
        window_samples: usize,
    },
    LowPass {
        id: String,
        channel: String,
        cutoff: UnitValue,
    },
    AdcQuantize {
        id: String,
        channel: String,
        bits: u8,
        min: UnitValue,
        max: UnitValue,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CriterionDefinition {
    pub id: String,
    pub channel: String,
    pub measurement: MeasurementDefinition,
    pub requirement: RequirementDefinition,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MeasurementDefinition {
    MinimumSample,
    MaximumSample,
    StateTransitionCount {
        threshold: UnitValue,
    },
    PulseWidth {
        state: SignalState,
        threshold: UnitValue,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        selection: Option<RunSelection>,
    },
    StableStateDuration {
        state: SignalState,
        threshold: UnitValue,
    },
    TransientEventDuration {
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold: UnitValue,
    },
    RiseTime {
        low_threshold: UnitValue,
        high_threshold: UnitValue,
    },
    FallTime {
        low_threshold: UnitValue,
        high_threshold: UnitValue,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequirementDefinition {
    pub operator: ComparisonOperator,
    pub value: UnitValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    EqualTo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalState {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSelection {
    Shortest,
    Longest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransientEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    Dropout,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_portable_rule_package_schema() {
        let package = RulePackage {
            package: PackageMetadata::new("switch-rule", "1.0.0"),
            target: TargetProfile {
                kind: TargetProfileKind::ControllerRuntime,
                identifier: Some("raspberry-pi-5-bare-metal".to_string()),
                notes: Vec::new(),
            },
            sample_timing: SampleTimingAssumption {
                timestamp_unit: EngineeringUnit::Second,
                nominal_sample_rate_hz: Some(10_000.0),
                sample_rate_tolerance_hz: Some(1.0),
                nominal_sample_interval_s: Some(0.0001),
                timestamp_tolerance_s: Some(0.000_001),
            },
            channels: vec![ChannelDefinition {
                name: "switch_v".to_string(),
                source_name: Some("daq_ai0".to_string()),
                unit: EngineeringUnit::Volt,
                sample_rate_hz: Some(10_000.0),
                thresholds: vec![
                    ThresholdDefinition {
                        name: "switch_low".to_string(),
                        role: ThresholdRole::Low,
                        value: UnitValue::new(0.5, EngineeringUnit::Volt),
                    },
                    ThresholdDefinition {
                        name: "switch_high".to_string(),
                        role: ThresholdRole::High,
                        value: UnitValue::new(4.5, EngineeringUnit::Volt),
                    },
                ],
            }],
            filters: vec![
                FilterDefinition::MovingAverage {
                    id: "filter_switch_average".to_string(),
                    channel: "switch_v".to_string(),
                    window_samples: 5,
                },
                FilterDefinition::LowPass {
                    id: "filter_switch_low_pass".to_string(),
                    channel: "switch_v".to_string(),
                    cutoff: UnitValue::new(250.0, EngineeringUnit::Hertz),
                },
                FilterDefinition::AdcQuantize {
                    id: "quantize_switch".to_string(),
                    channel: "switch_v".to_string(),
                    bits: 12,
                    min: UnitValue::new(0.0, EngineeringUnit::Volt),
                    max: UnitValue::new(5.0, EngineeringUnit::Volt),
                },
            ],
            criteria: vec![CriterionDefinition {
                id: "no_dropout_longer_than_1ms".to_string(),
                channel: "switch_v".to_string(),
                measurement: MeasurementDefinition::TransientEventDuration {
                    event_kind: TransientEventKind::Dropout,
                    expected_state: SignalState::High,
                    threshold: UnitValue::new(2.5, EngineeringUnit::Volt),
                },
                requirement: RequirementDefinition {
                    operator: ComparisonOperator::LessThanOrEqual,
                    value: UnitValue::new(0.001, EngineeringUnit::Second),
                },
            }],
        };

        assert_eq!(package.package.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(package.channels[0].unit.symbol(), "V");
        assert_eq!(package.channels[0].thresholds.len(), 2);
        assert_eq!(package.sample_timing.nominal_sample_rate_hz, Some(10_000.0));
        assert_eq!(package.criteria[0].channel, "switch_v");
        assert_eq!(
            package.criteria[0].requirement.value.unit,
            EngineeringUnit::Second
        );
    }

    #[test]
    fn serializes_and_deserializes_explicit_units() {
        let package = RulePackage::new(
            PackageMetadata::new("minimal", "0.1.0"),
            TargetProfile::new(TargetProfileKind::EmbeddedRuntime),
            SampleTimingAssumption::seconds_at_hz(1_000.0),
            vec![ChannelDefinition::new("control_v", EngineeringUnit::Volt)],
            vec![CriterionDefinition {
                id: "max_control".to_string(),
                channel: "control_v".to_string(),
                measurement: MeasurementDefinition::MaximumSample,
                requirement: RequirementDefinition {
                    operator: ComparisonOperator::LessThanOrEqual,
                    value: UnitValue::new(5.0, EngineeringUnit::Volt),
                },
            }],
        );

        let json = serde_json::to_string(&package).expect("schema should serialize");
        assert!(json.contains("\"schema_version\":\"0.1.0\""));
        assert!(json.contains("\"kind\":\"embedded_runtime\""));
        assert!(json.contains("\"unit\":\"V\""));
        assert!(json.contains("\"maximum_sample\""));
        assert!(!json.contains("csv"));
        assert!(!json.contains("plot"));
        assert!(!json.contains("hardware"));

        let round_trip: RulePackage =
            serde_json::from_str(&json).expect("schema should deserialize");
        assert_eq!(round_trip, package);
    }

    #[test]
    fn examples_rules_toml_and_json_describe_same_package() {
        let rules_toml = include_str!("../../../examples/rule-package/rules.toml");
        let rules_json = include_str!("../../../examples/rule-package/rules.json");

        let toml_package: RulePackage =
            toml::from_str(rules_toml).expect("rules.toml should match the schema");
        let json_package: RulePackage =
            serde_json::from_str(rules_json).expect("rules.json should match the schema");

        assert_eq!(toml_package, json_package);
        assert_eq!(toml_package.package.schema_version, CURRENT_SCHEMA_VERSION);
        assert_eq!(toml_package.channels[0].name, "switch_signal");
        assert_eq!(toml_package.channels[0].thresholds.len(), 3);
        assert_eq!(toml_package.filters.len(), 2);
        assert_eq!(toml_package.criteria.len(), 3);
    }
}

pub use ferrisoxide_measurements::{EdgeDirection, SignalState};

#[derive(Debug, Clone, PartialEq)]
pub struct Criterion {
    pub id: String,
    pub check: CriterionCheck,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TransientEventWindow {
    pub start_time_s: Option<f64>,
    pub end_time_s: Option<f64>,
    pub arm_after_first_expected_state: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResponseLatencySpec {
    pub source_channel: String,
    pub target_channel: String,
    pub source_threshold_v: f64,
    pub target_threshold_v: f64,
    pub source_state: SignalState,
    pub expected_target_state: SignalState,
    pub max_latency_s: f64,
}

impl Criterion {
    pub fn minimum_voltage(
        id: impl Into<String>,
        channel: impl Into<String>,
        threshold_v: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::MinimumVoltage {
                channel: channel.into(),
                threshold_v,
            },
        }
    }

    pub fn maximum_voltage(
        id: impl Into<String>,
        channel: impl Into<String>,
        threshold_v: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::MaximumVoltage {
                channel: channel.into(),
                threshold_v,
            },
        }
    }

    pub fn state_transitions(
        id: impl Into<String>,
        channel: impl Into<String>,
        threshold_v: f64,
        expected_count: usize,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::StateTransitions {
                channel: channel.into(),
                threshold_v,
                expected_count,
            },
        }
    }

    pub fn pulse_width(
        id: impl Into<String>,
        channel: impl Into<String>,
        state: SignalState,
        threshold_v: f64,
        min_width_s: Option<f64>,
        max_width_s: Option<f64>,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::PulseWidth {
                channel: channel.into(),
                state,
                threshold_v,
                min_width_s,
                max_width_s,
            },
        }
    }

    pub fn transient_duration(
        id: impl Into<String>,
        channel: impl Into<String>,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::TransientDuration {
                channel: channel.into(),
                expected_state,
                threshold_v,
                max_duration_s,
            },
        }
    }

    pub fn transient_event(
        id: impl Into<String>,
        channel: impl Into<String>,
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::TransientEvent {
                channel: channel.into(),
                event_kind,
                expected_state,
                threshold_v,
                max_duration_s,
                start_time_s: None,
                end_time_s: None,
                arm_after_first_expected_state: false,
            },
        }
    }

    pub fn transient_event_window(
        id: impl Into<String>,
        channel: impl Into<String>,
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
        window: TransientEventWindow,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::TransientEvent {
                channel: channel.into(),
                event_kind,
                expected_state,
                threshold_v,
                max_duration_s,
                start_time_s: window.start_time_s,
                end_time_s: window.end_time_s,
                arm_after_first_expected_state: window.arm_after_first_expected_state,
            },
        }
    }

    pub fn stable_state_duration(
        id: impl Into<String>,
        channel: impl Into<String>,
        state: SignalState,
        threshold_v: f64,
        min_duration_s: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::StableStateDuration {
                channel: channel.into(),
                state,
                threshold_v,
                min_duration_s,
            },
        }
    }

    pub fn rise_fall_time(
        id: impl Into<String>,
        channel: impl Into<String>,
        direction: EdgeDirection,
        low_threshold_v: f64,
        high_threshold_v: f64,
        max_duration_s: f64,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::RiseFallTime {
                channel: channel.into(),
                direction,
                low_threshold_v,
                high_threshold_v,
                max_duration_s,
            },
        }
    }

    pub fn measurement(
        id: impl Into<String>,
        channel: impl Into<String>,
        measurement: MeasurementSpec,
        requirement: MeasurementRequirement,
    ) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::Measurement {
                channel: channel.into(),
                measurement,
                requirement,
            },
        }
    }

    pub fn response_latency(id: impl Into<String>, spec: ResponseLatencySpec) -> Self {
        Self {
            id: id.into(),
            check: CriterionCheck::ResponseLatency {
                source_channel: spec.source_channel,
                target_channel: spec.target_channel,
                source_threshold_v: spec.source_threshold_v,
                target_threshold_v: spec.target_threshold_v,
                source_state: spec.source_state,
                expected_target_state: spec.expected_target_state,
                max_latency_s: spec.max_latency_s,
            },
        }
    }

    pub fn channel(&self) -> &str {
        self.check.channel()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CriterionCheck {
    MinimumVoltage {
        channel: String,
        threshold_v: f64,
    },
    MaximumVoltage {
        channel: String,
        threshold_v: f64,
    },
    StateTransitions {
        channel: String,
        threshold_v: f64,
        expected_count: usize,
    },
    PulseWidth {
        channel: String,
        state: SignalState,
        threshold_v: f64,
        min_width_s: Option<f64>,
        max_width_s: Option<f64>,
    },
    TransientDuration {
        channel: String,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
    },
    TransientEvent {
        channel: String,
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold_v: f64,
        max_duration_s: f64,
        start_time_s: Option<f64>,
        end_time_s: Option<f64>,
        arm_after_first_expected_state: bool,
    },
    StableStateDuration {
        channel: String,
        state: SignalState,
        threshold_v: f64,
        min_duration_s: f64,
    },
    RiseFallTime {
        channel: String,
        direction: EdgeDirection,
        low_threshold_v: f64,
        high_threshold_v: f64,
        max_duration_s: f64,
    },
    Measurement {
        channel: String,
        measurement: MeasurementSpec,
        requirement: MeasurementRequirement,
    },
    ResponseLatency {
        source_channel: String,
        target_channel: String,
        source_threshold_v: f64,
        target_threshold_v: f64,
        source_state: SignalState,
        expected_target_state: SignalState,
        max_latency_s: f64,
    },
}

impl CriterionCheck {
    pub fn channel(&self) -> &str {
        match self {
            Self::MinimumVoltage { channel, .. }
            | Self::MaximumVoltage { channel, .. }
            | Self::StateTransitions { channel, .. }
            | Self::PulseWidth { channel, .. }
            | Self::TransientDuration { channel, .. }
            | Self::TransientEvent { channel, .. }
            | Self::StableStateDuration { channel, .. }
            | Self::RiseFallTime { channel, .. }
            | Self::Measurement { channel, .. } => channel,
            Self::ResponseLatency { target_channel, .. } => target_channel,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriterionOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    EqualTo,
}

impl CriterionOperator {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "less_than" => Some(Self::LessThan),
            "less_than_or_equal" => Some(Self::LessThanOrEqual),
            "greater_than" => Some(Self::GreaterThan),
            "greater_than_or_equal" => Some(Self::GreaterThanOrEqual),
            "equal_to" => Some(Self::EqualTo),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::LessThan => "less_than",
            Self::LessThanOrEqual => "less_than_or_equal",
            Self::GreaterThan => "greater_than",
            Self::GreaterThanOrEqual => "greater_than_or_equal",
            Self::EqualTo => "equal_to",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CriterionMeasurementKind {
    MinimumSample,
    MaximumSample,
    StateTransitionCount,
    PulseWidth,
    StableStateDuration,
    TransientEventDuration,
    RiseTime,
    FallTime,
}

impl CriterionMeasurementKind {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "minimum_sample" => Some(Self::MinimumSample),
            "maximum_sample" => Some(Self::MaximumSample),
            "state_transition_count" => Some(Self::StateTransitionCount),
            "pulse_width" => Some(Self::PulseWidth),
            "stable_state_duration" => Some(Self::StableStateDuration),
            "transient_event_duration" => Some(Self::TransientEventDuration),
            "rise_time" => Some(Self::RiseTime),
            "fall_time" => Some(Self::FallTime),
            _ => None,
        }
    }

    pub fn requirement_unit(self) -> &'static str {
        match self {
            Self::MinimumSample | Self::MaximumSample => "V",
            Self::StateTransitionCount => "count",
            Self::PulseWidth
            | Self::StableStateDuration
            | Self::TransientEventDuration
            | Self::RiseTime
            | Self::FallTime => "s",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementRequirement {
    pub operator: CriterionOperator,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MeasurementSpec {
    MinimumSample,
    MaximumSample,
    StateTransitionCount {
        threshold_v: f64,
    },
    PulseWidth {
        state: SignalState,
        threshold_v: f64,
        selection: RunSelectionConfig,
    },
    StableStateDuration {
        state: SignalState,
        threshold_v: f64,
    },
    TransientEventDuration {
        event_kind: TransientEventKind,
        expected_state: SignalState,
        threshold_v: f64,
    },
    RiseTime {
        low_threshold_v: f64,
        high_threshold_v: f64,
    },
    FallTime {
        low_threshold_v: f64,
        high_threshold_v: f64,
    },
}

impl MeasurementSpec {
    pub fn kind(&self) -> CriterionMeasurementKind {
        match self {
            Self::MinimumSample => CriterionMeasurementKind::MinimumSample,
            Self::MaximumSample => CriterionMeasurementKind::MaximumSample,
            Self::StateTransitionCount { .. } => CriterionMeasurementKind::StateTransitionCount,
            Self::PulseWidth { .. } => CriterionMeasurementKind::PulseWidth,
            Self::StableStateDuration { .. } => CriterionMeasurementKind::StableStateDuration,
            Self::TransientEventDuration { .. } => CriterionMeasurementKind::TransientEventDuration,
            Self::RiseTime { .. } => CriterionMeasurementKind::RiseTime,
            Self::FallTime { .. } => CriterionMeasurementKind::FallTime,
        }
    }

    pub fn is_time_dependent(&self) -> bool {
        !matches!(
            self,
            Self::MinimumSample | Self::MaximumSample | Self::StateTransitionCount { .. }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunSelectionConfig {
    Shortest,
    Longest,
}

impl RunSelectionConfig {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "shortest" => Some(Self::Shortest),
            "longest" => Some(Self::Longest),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Shortest => "shortest",
            Self::Longest => "longest",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransientEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    Dropout,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

impl TransientEventKind {
    pub fn from_config(value: &str) -> Option<Self> {
        match value {
            "transient_event" => Some(Self::TransientEvent),
            "spurious_transition" => Some(Self::SpuriousTransition),
            "contact_bounce" => Some(Self::ContactBounce),
            "dropout" => Some(Self::Dropout),
            "noise_induced_transition" => Some(Self::NoiseInducedTransition),
            "threshold_crossing_event" => Some(Self::ThresholdCrossingEvent),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::TransientEvent => "transient event",
            Self::SpuriousTransition => "spurious transition",
            Self::ContactBounce => "contact bounce",
            Self::Dropout => "dropout",
            Self::NoiseInducedTransition => "noise-induced transition",
            Self::ThresholdCrossingEvent => "threshold crossing event",
        }
    }
}

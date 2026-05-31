pub use wra_measurements::{EdgeDirection, SignalState};

#[derive(Debug, Clone, PartialEq)]
pub struct Criterion {
    pub id: String,
    pub check: CriterionCheck,
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
            | Self::RiseFallTime { channel, .. } => channel,
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

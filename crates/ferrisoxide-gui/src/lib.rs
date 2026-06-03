use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;

use ferrisoxide_workflow::{
    analyze_csv, evaluate_bundle, inspect_source, load_csv_headers, load_csv_plot_series,
    render_source_inspection_output, scaffold_csv_config, AnalyzeCsvRequest, CsvPlotSeriesRequest,
    EvaluateBundleRequest, InspectSourceRequest, ScaffoldConfigRequest, SourceInspection,
    WorkflowBundleOutput, WorkflowPlotPoint, WorkflowPlotSeries, WorkflowSourceMode,
};

pub const TIME_UNIT_OPTIONS: &[&str] = &["s", "ms", "us", "ns", "min"];
pub const CHANNEL_UNIT_OPTIONS: &[&str] = &[
    "V", "mV", "A", "mA", "Ohm", "Hz", "degC", "Pa", "g", "m/s^2", "count", "unitless",
];
pub const PLOT_RESOLUTION_OPTIONS: &[GuiPlotResolution] = &[
    GuiPlotResolution::Fast,
    GuiPlotResolution::Balanced,
    GuiPlotResolution::Detailed,
    GuiPlotResolution::Full,
];
pub const CONFIG_FILTER_OPTIONS: &[GuiConfigFilterKind] = &[
    GuiConfigFilterKind::MovingAverage,
    GuiConfigFilterKind::MovingMedian,
    GuiConfigFilterKind::LowPass,
    GuiConfigFilterKind::HighPass,
    GuiConfigFilterKind::HighPassBaseline,
    GuiConfigFilterKind::Offset,
    GuiConfigFilterKind::Gain,
    GuiConfigFilterKind::Invert,
    GuiConfigFilterKind::Clamp,
    GuiConfigFilterKind::Deadband,
    GuiConfigFilterKind::BaselineSubtract,
    GuiConfigFilterKind::DcRemove,
    GuiConfigFilterKind::AbsoluteValue,
    GuiConfigFilterKind::Square,
    GuiConfigFilterKind::SquareRoot,
    GuiConfigFilterKind::Log,
    GuiConfigFilterKind::Exp,
    GuiConfigFilterKind::Normalize,
    GuiConfigFilterKind::Tanh,
    GuiConfigFilterKind::Sigmoid,
    GuiConfigFilterKind::SoftLimit,
];
pub const CONFIG_CRITERION_OPTIONS: &[GuiConfigCriterionKind] = &[
    GuiConfigCriterionKind::MinimumVoltage,
    GuiConfigCriterionKind::MaximumVoltage,
    GuiConfigCriterionKind::StateTransitions,
    GuiConfigCriterionKind::PulseWidth,
    GuiConfigCriterionKind::TransientDuration,
    GuiConfigCriterionKind::TransientEvent,
    GuiConfigCriterionKind::StableStateDuration,
    GuiConfigCriterionKind::RiseFallTime,
    GuiConfigCriterionKind::ResponseLatency,
];
pub const CONFIG_STATE_OPTIONS: &[GuiConfigState] = &[GuiConfigState::High, GuiConfigState::Low];
pub const CONFIG_DIRECTION_OPTIONS: &[GuiConfigDirection] =
    &[GuiConfigDirection::Rise, GuiConfigDirection::Fall];
pub const CONFIG_EVENT_KIND_OPTIONS: &[GuiConfigEventKind] = &[
    GuiConfigEventKind::TransientEvent,
    GuiConfigEventKind::SpuriousTransition,
    GuiConfigEventKind::ContactBounce,
    GuiConfigEventKind::Dropout,
    GuiConfigEventKind::NoiseInducedTransition,
    GuiConfigEventKind::ThresholdCrossingEvent,
];
pub const CONFIG_NORMALIZE_MODE_OPTIONS: &[GuiConfigNormalizeMode] = &[
    GuiConfigNormalizeMode::ZeroToOne,
    GuiConfigNormalizeMode::MinusOneToOne,
    GuiConfigNormalizeMode::ZScore,
    GuiConfigNormalizeMode::Range,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiSourceMode {
    Csv,
    Simulation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiTab {
    Source,
    Config,
    Run,
    Results,
    Plot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkflowStatus {
    Idle,
    Succeeded(String),
    Failed(String),
}

impl WorkflowStatus {
    pub fn message(&self) -> &str {
        match self {
            Self::Idle => "",
            Self::Succeeded(message) | Self::Failed(message) => message,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiChannelSelection {
    pub header: String,
    pub enabled: bool,
    pub unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiPlotChannelSelection {
    pub channel: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigFilterKind {
    MovingAverage,
    MovingMedian,
    LowPass,
    HighPass,
    HighPassBaseline,
    Offset,
    Gain,
    Invert,
    Clamp,
    Deadband,
    BaselineSubtract,
    DcRemove,
    AbsoluteValue,
    Square,
    SquareRoot,
    Log,
    Exp,
    Normalize,
    Tanh,
    Sigmoid,
    SoftLimit,
    Resample,
    Downsample,
    Decimate,
}

impl GuiConfigFilterKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::MovingAverage => "Moving average",
            Self::MovingMedian => "Moving median",
            Self::LowPass => "Low pass",
            Self::HighPass => "High pass",
            Self::HighPassBaseline => "High-pass baseline",
            Self::Offset => "Offset",
            Self::Gain => "Gain",
            Self::Invert => "Invert",
            Self::Clamp => "Clamp",
            Self::Deadband => "Deadband",
            Self::BaselineSubtract => "Baseline subtract",
            Self::DcRemove => "DC remove",
            Self::AbsoluteValue => "Absolute value",
            Self::Square => "Square",
            Self::SquareRoot => "Square root",
            Self::Log => "Log",
            Self::Exp => "Exp",
            Self::Normalize => "Normalize",
            Self::Tanh => "Tanh",
            Self::Sigmoid => "Sigmoid",
            Self::SoftLimit => "Soft limit",
            Self::Resample => "Resample",
            Self::Downsample => "Downsample",
            Self::Decimate => "Decimate",
        }
    }

    fn toml_type(self) -> &'static str {
        match self {
            Self::MovingAverage => "moving_average",
            Self::MovingMedian => "moving_median",
            Self::LowPass => "low_pass",
            Self::HighPass => "high_pass",
            Self::HighPassBaseline => "high_pass_baseline",
            Self::Offset => "offset",
            Self::Gain => "gain",
            Self::Invert => "invert",
            Self::Clamp => "clamp",
            Self::Deadband => "deadband",
            Self::BaselineSubtract => "baseline_subtract",
            Self::DcRemove => "dc_remove",
            Self::AbsoluteValue => "absolute_value",
            Self::Square => "square",
            Self::SquareRoot => "square_root",
            Self::Log => "log",
            Self::Exp => "exp",
            Self::Normalize => "normalize",
            Self::Tanh => "tanh",
            Self::Sigmoid => "sigmoid",
            Self::SoftLimit => "soft_limit",
            Self::Resample => "resample",
            Self::Downsample => "downsample",
            Self::Decimate => "decimate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigCriterionKind {
    MinimumVoltage,
    MaximumVoltage,
    StateTransitions,
    PulseWidth,
    TransientDuration,
    TransientEvent,
    StableStateDuration,
    RiseFallTime,
    ResponseLatency,
}

impl GuiConfigCriterionKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::MinimumVoltage => "Minimum voltage",
            Self::MaximumVoltage => "Maximum voltage",
            Self::StateTransitions => "State transitions",
            Self::PulseWidth => "Pulse width",
            Self::TransientDuration => "Transient duration",
            Self::TransientEvent => "Transient event",
            Self::StableStateDuration => "Stable state duration",
            Self::RiseFallTime => "Rise/fall time",
            Self::ResponseLatency => "Response latency",
        }
    }

    fn toml_type(self) -> &'static str {
        match self {
            Self::MinimumVoltage => "minimum_voltage",
            Self::MaximumVoltage => "maximum_voltage",
            Self::StateTransitions => "state_transitions",
            Self::PulseWidth => "pulse_width",
            Self::TransientDuration => "transient_duration",
            Self::TransientEvent => "transient_event",
            Self::StableStateDuration => "stable_state_duration",
            Self::RiseFallTime => "rise_fall_time",
            Self::ResponseLatency => "response_latency",
        }
    }

    fn id_suffix(self) -> &'static str {
        match self {
            Self::MinimumVoltage => "min_voltage",
            Self::MaximumVoltage => "max_voltage",
            Self::StateTransitions => "state_transitions",
            Self::PulseWidth => "pulse_width",
            Self::TransientDuration => "transient_duration",
            Self::TransientEvent => "transient_event",
            Self::StableStateDuration => "stable_state",
            Self::RiseFallTime => "rise_fall_time",
            Self::ResponseLatency => "response_latency",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigState {
    High,
    Low,
}

impl GuiConfigState {
    pub fn label(self) -> &'static str {
        match self {
            Self::High => "High",
            Self::Low => "Low",
        }
    }

    fn toml_value(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Low => "low",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigDirection {
    Rise,
    Fall,
}

impl GuiConfigDirection {
    pub fn label(self) -> &'static str {
        match self {
            Self::Rise => "Rise",
            Self::Fall => "Fall",
        }
    }

    fn toml_value(self) -> &'static str {
        match self {
            Self::Rise => "rise",
            Self::Fall => "fall",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigEventKind {
    TransientEvent,
    SpuriousTransition,
    ContactBounce,
    Dropout,
    NoiseInducedTransition,
    ThresholdCrossingEvent,
}

impl GuiConfigEventKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::TransientEvent => "Transient event",
            Self::SpuriousTransition => "Spurious transition",
            Self::ContactBounce => "Contact bounce",
            Self::Dropout => "Dropout",
            Self::NoiseInducedTransition => "Noise-induced transition",
            Self::ThresholdCrossingEvent => "Threshold crossing event",
        }
    }

    fn toml_value(self) -> &'static str {
        match self {
            Self::TransientEvent => "transient_event",
            Self::SpuriousTransition => "spurious_transition",
            Self::ContactBounce => "contact_bounce",
            Self::Dropout => "dropout",
            Self::NoiseInducedTransition => "noise_induced_transition",
            Self::ThresholdCrossingEvent => "threshold_crossing_event",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiConfigNormalizeMode {
    ZeroToOne,
    MinusOneToOne,
    ZScore,
    Range,
}

impl GuiConfigNormalizeMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::ZeroToOne => "0 to 1",
            Self::MinusOneToOne => "-1 to 1",
            Self::ZScore => "Z score",
            Self::Range => "Range",
        }
    }

    fn toml_value(self) -> &'static str {
        match self {
            Self::ZeroToOne => "zero_to_one",
            Self::MinusOneToOne => "minus_one_to_one",
            Self::ZScore => "z_score",
            Self::Range => "range",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiConfigFilterDraft {
    pub kind: GuiConfigFilterKind,
    pub window_samples: usize,
    pub cutoff_hz: f64,
    pub offset_v: f64,
    pub gain: f64,
    pub threshold_v: f64,
    pub baseline_v: f64,
    pub min_v: f64,
    pub max_v: f64,
    pub base: f64,
    pub limit_v: f64,
    pub sample_interval_s: f64,
    pub factor: usize,
    pub normalize_mode: GuiConfigNormalizeMode,
    pub input_min_v: f64,
    pub input_max_v: f64,
    pub output_min: f64,
    pub output_max: f64,
}

impl Default for GuiConfigFilterDraft {
    fn default() -> Self {
        Self {
            kind: GuiConfigFilterKind::MovingAverage,
            window_samples: 3,
            cutoff_hz: 10.0,
            offset_v: 0.0,
            gain: 1.0,
            threshold_v: 0.05,
            baseline_v: 0.0,
            min_v: 0.0,
            max_v: 5.0,
            base: 10.0,
            limit_v: 5.0,
            sample_interval_s: 0.001,
            factor: 2,
            normalize_mode: GuiConfigNormalizeMode::ZeroToOne,
            input_min_v: 0.0,
            input_max_v: 5.0,
            output_min: 0.0,
            output_max: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiConfigCriterionDraft {
    pub kind: GuiConfigCriterionKind,
    pub threshold_v: f64,
    pub expected_count: usize,
    pub state: GuiConfigState,
    pub expected_state: GuiConfigState,
    pub event_kind: GuiConfigEventKind,
    pub min_width_s: f64,
    pub max_width_s: f64,
    pub max_duration_s: f64,
    pub min_duration_s: f64,
    pub low_threshold_v: f64,
    pub high_threshold_v: f64,
    pub direction: GuiConfigDirection,
    pub source_channel: String,
    pub source_threshold_v: f64,
    pub target_threshold_v: f64,
    pub source_state: GuiConfigState,
    pub expected_target_state: GuiConfigState,
    pub max_latency_s: f64,
    pub start_time_s: f64,
    pub end_time_s: f64,
}

impl Default for GuiConfigCriterionDraft {
    fn default() -> Self {
        Self {
            kind: GuiConfigCriterionKind::MaximumVoltage,
            threshold_v: 5.5,
            expected_count: 1,
            state: GuiConfigState::High,
            expected_state: GuiConfigState::High,
            event_kind: GuiConfigEventKind::TransientEvent,
            min_width_s: 0.001,
            max_width_s: 1.0,
            max_duration_s: 1.0,
            min_duration_s: 0.001,
            low_threshold_v: 0.5,
            high_threshold_v: 4.5,
            direction: GuiConfigDirection::Rise,
            source_channel: String::new(),
            source_threshold_v: 2.5,
            target_threshold_v: 2.5,
            source_state: GuiConfigState::High,
            expected_target_state: GuiConfigState::High,
            max_latency_s: 1.0,
            start_time_s: 0.0,
            end_time_s: 1.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiChannelConfigDraft {
    pub channel: String,
    pub unit: String,
    pub filters: Vec<GuiConfigFilterDraft>,
    pub criteria: Vec<GuiConfigCriterionDraft>,
}

impl GuiChannelConfigDraft {
    fn new(channel: String, unit: String) -> Self {
        Self {
            channel,
            unit,
            filters: Vec::new(),
            criteria: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GuiConfigDraft {
    pub selected_channel: Option<String>,
    pub channels: Vec<GuiChannelConfigDraft>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiPlotResolution {
    Fast,
    Balanced,
    Detailed,
    Full,
}

impl GuiPlotResolution {
    pub fn label(self) -> &'static str {
        match self {
            Self::Fast => "Fast",
            Self::Balanced => "Balanced",
            Self::Detailed => "Detailed",
            Self::Full => "Full",
        }
    }

    pub fn render_point_budget(self, plot_width_px: f64) -> Option<usize> {
        let width = plot_width_px.clamp(128.0, 8192.0);
        let budget = match self {
            Self::Fast => (width * 2.0).ceil().clamp(512.0, 25_000.0),
            Self::Balanced => (width * 4.0).ceil().clamp(1_024.0, 50_000.0),
            Self::Detailed => (width * 6.0).ceil().clamp(2_048.0, 100_000.0),
            Self::Full => return None,
        };
        Some(budget as usize)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiRenderedPlotSeries {
    pub name: String,
    pub unit: String,
    pub points: Vec<[f64; 2]>,
    pub raw_point_count: usize,
    pub visible_point_count: usize,
    pub rendered_point_count: usize,
    pub pyramid_stride: usize,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GuiPlotPyramid {
    pub data_revision: u64,
    pub series: Vec<GuiPlotPyramidSeries>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiPlotPyramidSeries {
    pub name: String,
    pub unit: String,
    pub raw_point_count: usize,
    pub levels: Vec<GuiPlotPyramidLevel>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiPlotPyramidLevel {
    pub stride: usize,
    pub points: Vec<WorkflowPlotPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GuiPlotRenderSignature {
    pub data_revision: u64,
    pub resolution: GuiPlotResolution,
    pub width_px_bucket: u32,
    pub x_min_bucket: i64,
    pub x_max_bucket: i64,
    pub selected_channels: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GuiPlotRenderCache {
    pub signature: GuiPlotRenderSignature,
    pub series: Vec<GuiRenderedPlotSeries>,
}

#[derive(Debug, Clone)]
pub struct GuiSession {
    pub active_tab: GuiTab,
    pub source_mode: GuiSourceMode,
    pub input_path: String,
    pub config_path: String,
    pub output_dir: String,
    pub time_column: String,
    pub channel_csv: String,
    pub csv_headers: Vec<String>,
    pub channel_selections: Vec<GuiChannelSelection>,
    pub plot_channel_selections: Vec<GuiPlotChannelSelection>,
    pub time_unit: String,
    pub signal_unit: String,
    pub control_config_path: String,
    pub verification_config_path: String,
    pub channel_map_path: String,
    pub simulation_mode: String,
    pub overwrite_outputs: bool,
    pub include_plot_artifact: bool,
    pub status: WorkflowStatus,
    pub inspection: Option<SourceInspection>,
    pub inspection_text: String,
    pub config_text: String,
    pub config_draft: GuiConfigDraft,
    pub report_preview: String,
    pub bundle: Option<WorkflowBundleOutput>,
    pub plot_series: Vec<WorkflowPlotSeries>,
    pub plot_resolution: GuiPlotResolution,
    pub plot_pyramid: GuiPlotPyramid,
    pub plot_render_cache: Option<GuiPlotRenderCache>,
    pub plot_data_revision: u64,
    pub plot_render_cache_hits: usize,
    pub plot_render_cache_misses: usize,
    pub plot_render_summary: String,
}

impl Default for GuiSession {
    fn default() -> Self {
        Self {
            active_tab: GuiTab::Source,
            source_mode: GuiSourceMode::Csv,
            input_path: String::new(),
            config_path: String::new(),
            output_dir: String::new(),
            time_column: "time".to_string(),
            channel_csv: String::new(),
            csv_headers: Vec::new(),
            channel_selections: Vec::new(),
            plot_channel_selections: Vec::new(),
            time_unit: "s".to_string(),
            signal_unit: "V".to_string(),
            control_config_path: String::new(),
            verification_config_path: String::new(),
            channel_map_path: String::new(),
            simulation_mode: String::new(),
            overwrite_outputs: false,
            include_plot_artifact: true,
            status: WorkflowStatus::Idle,
            inspection: None,
            inspection_text: String::new(),
            config_text: String::new(),
            config_draft: GuiConfigDraft::default(),
            report_preview: String::new(),
            bundle: None,
            plot_series: Vec::new(),
            plot_resolution: GuiPlotResolution::Balanced,
            plot_pyramid: GuiPlotPyramid::default(),
            plot_render_cache: None,
            plot_data_revision: 0,
            plot_render_cache_hits: 0,
            plot_render_cache_misses: 0,
            plot_render_summary: String::new(),
        }
    }
}

impl GuiSession {
    pub fn set_input_path(&mut self, input_path: impl Into<String>) {
        self.input_path = input_path.into();
        self.csv_headers.clear();
        self.channel_selections.clear();
        self.plot_channel_selections.clear();
        self.channel_csv.clear();
        self.inspection = None;
        self.inspection_text.clear();
        self.config_draft = GuiConfigDraft::default();
        self.config_text.clear();
        self.replace_plot_series(Vec::new());
    }

    pub fn load_channels_from_csv(&mut self) {
        if self.source_mode != GuiSourceMode::Csv {
            self.fail("Channel loading is currently limited to CSV sources".to_string());
            return;
        }
        if self.input_path.trim().is_empty() {
            self.fail("CSV input path is required before loading channels".to_string());
            return;
        }
        match load_csv_headers(&self.input_path) {
            Ok(headers) => {
                let header_count = headers.len();
                self.csv_headers = headers;
                self.sync_time_column_with_headers();
                self.sync_channel_selections_with_headers();
                self.status =
                    WorkflowStatus::Succeeded(format!("Loaded {header_count} CSV headers"));
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn set_time_column(&mut self, time_column: impl Into<String>) {
        self.time_column = time_column.into();
        self.sync_channel_selections_with_headers();
    }

    pub fn refresh_channel_selection_summary(&mut self) {
        self.channel_csv = self.selected_channel_names().join(",");
        self.signal_unit = self.selected_signal_unit();
        self.sync_plot_channel_selections();
        self.sync_config_draft_with_source_channels();
    }

    pub fn refresh_plot_channel_choices(&mut self) {
        self.sync_plot_channel_selections();
    }

    pub fn selected_channel_names(&self) -> Vec<String> {
        if self.channel_selections.is_empty() {
            parse_channel_csv(&self.channel_csv)
        } else {
            self.channel_selections
                .iter()
                .filter(|selection| selection.enabled)
                .map(|selection| selection.header.clone())
                .collect()
        }
    }

    pub fn selected_plot_channel_names(&self) -> Vec<String> {
        if self.plot_channel_selections.is_empty() {
            self.selected_channel_names()
        } else {
            self.plot_channel_selections
                .iter()
                .filter(|selection| selection.enabled)
                .map(|selection| selection.channel.clone())
                .collect()
        }
    }

    pub fn selected_config_channel_index(&self) -> Option<usize> {
        self.config_draft
            .selected_channel
            .as_ref()
            .and_then(|selected| {
                self.config_draft
                    .channels
                    .iter()
                    .position(|channel| &channel.channel == selected)
            })
            .or_else(|| (!self.config_draft.channels.is_empty()).then_some(0))
    }

    pub fn select_config_channel(&mut self, channel: impl Into<String>) {
        let channel = channel.into();
        if self
            .config_draft
            .channels
            .iter()
            .any(|draft| draft.channel == channel)
        {
            self.config_draft.selected_channel = Some(channel);
        }
    }

    pub fn config_channel_mut(&mut self, channel: &str) -> Option<&mut GuiChannelConfigDraft> {
        self.config_draft
            .channels
            .iter_mut()
            .find(|draft| draft.channel == channel)
    }

    pub fn refresh_config_channel_choices(&mut self) {
        self.sync_config_draft_with_source_channels();
    }

    pub fn set_plot_resolution(&mut self, resolution: GuiPlotResolution) {
        if self.plot_resolution != resolution {
            self.plot_resolution = resolution;
            self.invalidate_plot_render_cache();
        }
    }

    pub fn plot_x_range(&self) -> Option<(f64, f64)> {
        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        for series in &self.plot_series {
            if let Some(first) = series.points.first() {
                x_min = x_min.min(first.time);
            }
            if let Some(last) = series.points.last() {
                x_max = x_max.max(last.time);
            }
        }
        if x_min.is_finite() && x_max.is_finite() && x_max > x_min {
            Some((x_min, x_max))
        } else {
            None
        }
    }

    pub fn rendered_plot_series_for_viewport(
        &mut self,
        x_min: f64,
        x_max: f64,
        plot_width_px: f64,
    ) -> Vec<GuiRenderedPlotSeries> {
        let (x_min, x_max) = self
            .normalize_plot_x_range(x_min, x_max)
            .unwrap_or((0.0, 1.0));
        let signature = self.plot_render_signature(x_min, x_max, plot_width_px);
        if let Some(cache) = &self.plot_render_cache {
            if cache.signature == signature {
                self.plot_render_cache_hits += 1;
                return cache.series.clone();
            }
        }

        let rendered = self.build_rendered_plot_series(x_min, x_max, plot_width_px);
        self.plot_render_cache_misses += 1;
        self.plot_render_summary = render_summary(&rendered, self.plot_resolution);
        self.plot_render_cache = Some(GuiPlotRenderCache {
            signature,
            series: rendered.clone(),
        });
        rendered
    }

    pub fn plot_pyramid_strides_for_channel(&self, channel: &str) -> Vec<usize> {
        self.plot_pyramid
            .series
            .iter()
            .find(|series| series.name == channel)
            .map(|series| series.levels.iter().map(|level| level.stride).collect())
            .unwrap_or_default()
    }

    pub fn set_plot_series_for_testing(&mut self, series: Vec<WorkflowPlotSeries>) {
        self.replace_plot_series(series);
    }

    pub fn inspect_current_source(&mut self) {
        let result = self.inspect_request().and_then(|request| {
            let mut inspection = inspect_source(&request)?;
            self.apply_channel_units_to_inspection(&mut inspection);
            let text = render_source_inspection_output(&inspection, "text")?;
            Ok((inspection, text))
        });
        match result {
            Ok((inspection, text)) => {
                self.inspection = Some(inspection);
                self.inspection_text = text;
                self.status = WorkflowStatus::Succeeded("Source inspection complete".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn scaffold_config(&mut self) {
        if self.source_mode != GuiSourceMode::Csv {
            self.fail("Config scaffolding is currently limited to CSV sources".to_string());
            return;
        }
        let request = ScaffoldConfigRequest {
            input_path: self.input_path.clone(),
            time_column: self.time_column.clone(),
            channels: self.selected_channel_names(),
            time_unit: self.time_unit.clone(),
            signal_unit: self.selected_signal_unit(),
        };
        match scaffold_csv_config(&request) {
            Ok(config) => {
                self.config_text = config;
                self.status = WorkflowStatus::Succeeded("Config scaffold generated".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn load_config_builder_from_source(&mut self) {
        self.sync_config_draft_with_source_channels();
        if self.config_draft.channels.is_empty() {
            self.fail("Load and enable at least one Source channel first".to_string());
            return;
        }
        self.refresh_config_text_from_builder();
        self.status = WorkflowStatus::Succeeded(format!(
            "Loaded {} channel config section(s)",
            self.config_draft.channels.len()
        ));
    }

    pub fn generate_config_text_from_builder(&mut self) {
        match self.build_config_text_from_builder() {
            Ok(config_text) => {
                self.config_text = config_text;
                self.status = WorkflowStatus::Succeeded("Config generated".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn refresh_config_text_from_builder(&mut self) {
        if let Ok(config_text) = self.build_config_text_from_builder() {
            self.config_text = config_text;
        }
    }

    pub fn load_config_text_from_path(&mut self, config_path: impl Into<String>) {
        let config_path = config_path.into();
        if config_path.trim().is_empty() {
            self.fail("Config file is required before loading".to_string());
            return;
        }
        match fs::read_to_string(&config_path) {
            Ok(config_text) => {
                self.config_path = config_path;
                self.config_text = config_text;
                self.status = WorkflowStatus::Succeeded("Config loaded".to_string());
            }
            Err(error) => self.fail(format!("failed to read `{config_path}`: {error}")),
        }
    }

    pub fn save_config_text(&mut self) {
        if self.config_path.trim().is_empty() {
            self.fail("Config path is required before saving".to_string());
            return;
        }
        match fs::write(&self.config_path, &self.config_text) {
            Ok(()) => {
                self.status = WorkflowStatus::Succeeded("Config saved".to_string());
            }
            Err(error) => self.fail(format!("failed to write `{}`: {error}", self.config_path)),
        }
    }

    pub fn analyze_current_config(&mut self) {
        let result = self
            .require_csv_config()
            .and_then(|(input_path, config_path)| {
                analyze_csv(&AnalyzeCsvRequest::json(input_path, config_path))
            });
        match result {
            Ok(report) => {
                self.report_preview = report;
                self.status = WorkflowStatus::Succeeded("Analysis complete".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn evaluate_current_bundle(&mut self) {
        let result = self
            .evaluate_bundle_request()
            .and_then(|request| evaluate_bundle(&request));
        match result {
            Ok(bundle) => {
                self.report_preview = bundle.summary_json.clone();
                self.bundle = Some(bundle);
                self.status = WorkflowStatus::Succeeded("Evaluation bundle complete".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn load_plot_series(&mut self) {
        if self.source_mode != GuiSourceMode::Csv {
            self.fail("Interactive plots are currently limited to CSV sources".to_string());
            return;
        }
        self.sync_plot_channel_selections();
        let selected_plot_channels = self.selected_plot_channel_names();
        if selected_plot_channels.is_empty() {
            self.fail("Select at least one channel to plot".to_string());
            return;
        }
        let request = if self.config_path.trim().is_empty() {
            CsvPlotSeriesRequest {
                input_path: self.input_path.clone(),
                config_path: None,
                time_column: self.time_column.clone(),
                channels: selected_plot_channels.clone(),
                time_unit: self.time_unit.clone(),
                signal_unit: self.selected_signal_unit(),
            }
        } else {
            CsvPlotSeriesRequest::from_config(self.input_path.clone(), self.config_path.clone())
        };
        match load_csv_plot_series(&request) {
            Ok(mut series) => {
                self.apply_channel_units_to_plot_series(&mut series);
                series.retain(|series| {
                    selected_plot_channels
                        .iter()
                        .any(|channel| channel == &series.name)
                });
                if series.is_empty() {
                    self.fail("No plot series matched the selected channels".to_string());
                    return;
                }
                self.replace_plot_series(series);
                self.status = WorkflowStatus::Succeeded("Plot series loaded".to_string());
            }
            Err(error) => self.fail(error),
        }
    }

    pub fn artifact_rows(&self) -> Vec<String> {
        self.bundle
            .as_ref()
            .map(|bundle| bundle.artifacts.clone())
            .unwrap_or_default()
    }

    fn replace_plot_series(&mut self, series: Vec<WorkflowPlotSeries>) {
        self.plot_series = series;
        self.plot_data_revision = self.plot_data_revision.wrapping_add(1);
        self.plot_pyramid = GuiPlotPyramid::from_series(self.plot_data_revision, &self.plot_series);
        self.invalidate_plot_render_cache();
    }

    fn invalidate_plot_render_cache(&mut self) {
        self.plot_render_cache = None;
        self.plot_render_summary.clear();
    }

    fn normalize_plot_x_range(&self, x_min: f64, x_max: f64) -> Option<(f64, f64)> {
        if x_min.is_finite() && x_max.is_finite() && x_max > x_min {
            Some((x_min, x_max))
        } else {
            self.plot_x_range()
        }
    }

    fn plot_render_signature(
        &self,
        x_min: f64,
        x_max: f64,
        plot_width_px: f64,
    ) -> GuiPlotRenderSignature {
        let width_px_bucket = ((plot_width_px.max(1.0) / 8.0).round().max(1.0) * 8.0) as u32;
        let signature_span = self
            .plot_x_range()
            .map(|(full_min, full_max)| full_max - full_min)
            .filter(|span| span.is_finite() && *span > 0.0)
            .unwrap_or_else(|| (x_max - x_min).abs());
        let x_per_pixel = (signature_span / plot_width_px.max(1.0)).max(f64::EPSILON);
        let quantum = (x_per_pixel / 4.0).max(f64::EPSILON);
        GuiPlotRenderSignature {
            data_revision: self.plot_data_revision,
            resolution: self.plot_resolution,
            width_px_bucket,
            x_min_bucket: quantize_f64(x_min, quantum),
            x_max_bucket: quantize_f64(x_max, quantum),
            selected_channels: self.selected_plot_channel_names().join(","),
        }
    }

    fn build_rendered_plot_series(
        &self,
        x_min: f64,
        x_max: f64,
        plot_width_px: f64,
    ) -> Vec<GuiRenderedPlotSeries> {
        let selected_channels = self.selected_plot_channel_names();
        let point_budget = self.plot_resolution.render_point_budget(plot_width_px);
        self.plot_series
            .iter()
            .filter(|series| {
                selected_channels.is_empty()
                    || selected_channels
                        .iter()
                        .any(|channel| channel == &series.name)
            })
            .map(|series| self.render_plot_series(series, x_min, x_max, point_budget))
            .collect()
    }

    fn render_plot_series(
        &self,
        series: &WorkflowPlotSeries,
        x_min: f64,
        x_max: f64,
        point_budget: Option<usize>,
    ) -> GuiRenderedPlotSeries {
        let raw_visible_count = count_points_in_x_range(&series.points, x_min, x_max);
        let desired_stride = point_budget
            .map(|budget| desired_pyramid_stride(raw_visible_count, budget))
            .unwrap_or(1);
        let (candidate_points, pyramid_stride) =
            self.pyramid_candidate_points(series, desired_stride);
        let visible_points = points_in_x_range(candidate_points, x_min, x_max);
        let rendered_points = match point_budget {
            Some(budget) if visible_points.len() > budget => {
                min_max_decimate_for_viewport(visible_points, x_min, x_max, budget)
            }
            _ => visible_points.to_vec(),
        };
        let points = rendered_points
            .iter()
            .map(|point| [point.time, point.value])
            .collect::<Vec<_>>();
        GuiRenderedPlotSeries {
            name: series.name.clone(),
            unit: series.unit.clone(),
            raw_point_count: series.points.len(),
            visible_point_count: raw_visible_count,
            rendered_point_count: points.len(),
            pyramid_stride,
            points,
        }
    }

    fn pyramid_candidate_points<'a>(
        &'a self,
        series: &'a WorkflowPlotSeries,
        desired_stride: usize,
    ) -> (&'a [WorkflowPlotPoint], usize) {
        if desired_stride <= 1 {
            return (&series.points, 1);
        }
        let level = self
            .plot_pyramid
            .series
            .iter()
            .find(|pyramid_series| pyramid_series.name == series.name)
            .and_then(|pyramid_series| {
                pyramid_series
                    .levels
                    .iter()
                    .filter(|level| level.stride <= desired_stride)
                    .max_by_key(|level| level.stride)
            });
        if let Some(level) = level {
            (&level.points, level.stride)
        } else {
            (&series.points, 1)
        }
    }

    fn inspect_request(&self) -> Result<InspectSourceRequest, String> {
        match self.source_mode {
            GuiSourceMode::Csv => Ok(InspectSourceRequest {
                source_mode: WorkflowSourceMode::Csv,
                input_path: self.input_path.clone(),
                time_column: self.time_column.clone(),
                channels: self.selected_channel_names(),
                time_unit: self.time_unit.clone(),
                signal_unit: self.selected_signal_unit(),
                channel_map_path: None,
            }),
            GuiSourceMode::Simulation => {
                if self.channel_map_path.trim().is_empty() {
                    return Err(
                        "Channel map path is required for simulation inspection".to_string()
                    );
                }
                Ok(InspectSourceRequest {
                    source_mode: WorkflowSourceMode::Simulation,
                    input_path: self.input_path.clone(),
                    time_column: self.time_column.clone(),
                    channels: self.selected_channel_names(),
                    time_unit: self.time_unit.clone(),
                    signal_unit: self.selected_signal_unit(),
                    channel_map_path: Some(self.channel_map_path.clone()),
                })
            }
        }
    }

    fn require_csv_config(&self) -> Result<(String, String), String> {
        if self.source_mode != GuiSourceMode::Csv {
            return Err("Analysis preview is currently limited to CSV sources".to_string());
        }
        if self.input_path.trim().is_empty() {
            return Err("Input path is required".to_string());
        }
        if self.config_path.trim().is_empty() {
            return Err("Config path is required".to_string());
        }
        Ok((self.input_path.clone(), self.config_path.clone()))
    }

    fn evaluate_bundle_request(&self) -> Result<EvaluateBundleRequest, String> {
        if self.output_dir.trim().is_empty() {
            return Err("Output directory is required".to_string());
        }
        match self.source_mode {
            GuiSourceMode::Csv => {
                let (input_path, config_path) = self.require_csv_config()?;
                let mut request =
                    EvaluateBundleRequest::csv(input_path, config_path, self.output_dir.clone());
                request.overwrite = self.overwrite_outputs;
                request.include_plot = self.include_plot_artifact;
                Ok(request)
            }
            GuiSourceMode::Simulation => {
                if self.input_path.trim().is_empty() {
                    return Err("Input path is required".to_string());
                }
                if self.control_config_path.trim().is_empty() {
                    return Err("Production control config path is required".to_string());
                }
                if self.verification_config_path.trim().is_empty() {
                    return Err("Test verification config path is required".to_string());
                }
                if self.channel_map_path.trim().is_empty() {
                    return Err("Channel map path is required".to_string());
                }
                let mut request = EvaluateBundleRequest::simulation(
                    self.input_path.clone(),
                    self.control_config_path.clone(),
                    self.verification_config_path.clone(),
                    self.channel_map_path.clone(),
                    self.output_dir.clone(),
                );
                request.overwrite = self.overwrite_outputs;
                if !self.simulation_mode.trim().is_empty() {
                    request.mode = Some(self.simulation_mode.clone());
                }
                Ok(request)
            }
        }
    }

    fn fail(&mut self, error: String) {
        self.status = WorkflowStatus::Failed(error);
    }

    fn sync_time_column_with_headers(&mut self) {
        if self.csv_headers.is_empty()
            || self
                .csv_headers
                .iter()
                .any(|header| header == &self.time_column)
        {
            return;
        }
        if let Some(time_header) = self
            .csv_headers
            .iter()
            .find(|header| header.eq_ignore_ascii_case("time"))
        {
            self.time_column = time_header.clone();
        } else if let Some(first_header) = self.csv_headers.first() {
            self.time_column = first_header.clone();
        }
    }

    fn sync_channel_selections_with_headers(&mut self) {
        if self.csv_headers.is_empty() {
            self.refresh_channel_selection_summary();
            return;
        }
        let previous = self
            .channel_selections
            .iter()
            .map(|selection| {
                (
                    selection.header.clone(),
                    (selection.enabled, selection.unit.clone()),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let default_unit = self.signal_unit.clone();
        self.channel_selections = self
            .csv_headers
            .iter()
            .filter(|header| header.as_str() != self.time_column)
            .map(|header| {
                let (enabled, unit) = previous
                    .get(header)
                    .cloned()
                    .unwrap_or((true, default_unit.clone()));
                GuiChannelSelection {
                    header: header.clone(),
                    enabled,
                    unit,
                }
            })
            .collect();
        self.refresh_channel_selection_summary();
    }

    fn sync_config_draft_with_source_channels(&mut self) {
        let options = self.config_channel_options();
        if options.is_empty() {
            self.config_draft.channels.clear();
            self.config_draft.selected_channel = None;
            return;
        }

        let previous = self
            .config_draft
            .channels
            .iter()
            .map(|draft| (draft.channel.clone(), draft.clone()))
            .collect::<BTreeMap<_, _>>();
        self.config_draft.channels = options
            .into_iter()
            .map(|(channel, unit)| {
                if let Some(mut draft) = previous.get(&channel).cloned() {
                    draft.unit = unit;
                    draft
                } else {
                    GuiChannelConfigDraft::new(channel, unit)
                }
            })
            .collect();

        let selected_is_valid =
            self.config_draft
                .selected_channel
                .as_ref()
                .is_some_and(|selected| {
                    self.config_draft
                        .channels
                        .iter()
                        .any(|draft| &draft.channel == selected)
                });
        if !selected_is_valid {
            self.config_draft.selected_channel = self
                .config_draft
                .channels
                .first()
                .map(|draft| draft.channel.clone());
        }
    }

    fn config_channel_options(&self) -> Vec<(String, String)> {
        if !self.channel_selections.is_empty() {
            return self
                .channel_selections
                .iter()
                .filter(|selection| selection.enabled)
                .map(|selection| (selection.header.clone(), selection.unit.clone()))
                .collect();
        }
        parse_channel_csv(&self.channel_csv)
            .into_iter()
            .map(|channel| (channel, self.signal_unit.clone()))
            .collect()
    }

    fn build_config_text_from_builder(&self) -> Result<String, String> {
        if self.source_mode != GuiSourceMode::Csv {
            return Err("Config generation is currently limited to CSV sources".to_string());
        }
        if self.input_path.trim().is_empty() {
            return Err("CSV input path is required before generating config".to_string());
        }
        if self.time_column.trim().is_empty() {
            return Err("Time column is required before generating config".to_string());
        }
        if self.config_draft.channels.is_empty() {
            return Err("Load and enable at least one Source channel first".to_string());
        }

        let channel_names = self
            .config_draft
            .channels
            .iter()
            .map(|draft| draft.channel.clone())
            .collect::<Vec<_>>();
        let signal_unit = self
            .config_draft
            .channels
            .first()
            .map(|draft| draft.unit.clone())
            .unwrap_or_else(|| self.selected_signal_unit());
        let mut output = String::new();
        writeln!(&mut output, "[input]").expect("writing to String should succeed");
        writeln!(
            &mut output,
            "time_column = {}",
            toml_string(&self.time_column)
        )
        .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "channels = {}",
            toml_string_array(&channel_names)
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "time_unit = {}", toml_string(&self.time_unit))
            .expect("writing to String should succeed");
        writeln!(&mut output, "signal_unit = {}", toml_string(&signal_unit))
            .expect("writing to String should succeed");

        for channel in &self.config_draft.channels {
            for filter in &channel.filters {
                output.push('\n');
                write_filter_config(&mut output, &channel.channel, filter);
            }
        }

        for channel in &self.config_draft.channels {
            for (index, criterion) in channel.criteria.iter().enumerate() {
                output.push('\n');
                write_criterion_config(&mut output, &channel.channel, index, criterion);
            }
        }

        Ok(output)
    }

    fn selected_signal_unit(&self) -> String {
        self.channel_selections
            .iter()
            .find(|selection| selection.enabled)
            .map(|selection| selection.unit.clone())
            .unwrap_or_else(|| self.signal_unit.clone())
    }

    fn apply_channel_units_to_inspection(&self, inspection: &mut SourceInspection) {
        for channel in &mut inspection.channels {
            if let Some(unit) = self.unit_for_channel(&channel.id, &channel.source_column) {
                channel.unit = unit;
            }
        }
    }

    fn apply_channel_units_to_plot_series(&self, series: &mut [WorkflowPlotSeries]) {
        for series in series {
            if let Some(unit) = self.unit_for_channel(&series.name, &series.name) {
                series.unit = unit;
            }
        }
    }

    fn unit_for_channel(&self, channel_id: &str, source_column: &str) -> Option<String> {
        self.channel_selections
            .iter()
            .find(|selection| selection.header == channel_id || selection.header == source_column)
            .map(|selection| selection.unit.clone())
    }

    fn sync_plot_channel_selections(&mut self) {
        let options = self.plot_channel_options();
        if options.is_empty() {
            self.plot_channel_selections.clear();
            return;
        }
        let previous = self
            .plot_channel_selections
            .iter()
            .map(|selection| (selection.channel.clone(), selection.enabled))
            .collect::<BTreeMap<_, _>>();
        self.plot_channel_selections = options
            .into_iter()
            .map(|(channel, default_enabled)| GuiPlotChannelSelection {
                enabled: previous.get(&channel).copied().unwrap_or(default_enabled),
                channel,
            })
            .collect();
    }

    fn plot_channel_options(&self) -> Vec<(String, bool)> {
        if !self.channel_selections.is_empty() {
            return self
                .channel_selections
                .iter()
                .map(|selection| (selection.header.clone(), selection.enabled))
                .collect();
        }
        let parsed_channels = parse_channel_csv(&self.channel_csv);
        if !parsed_channels.is_empty() {
            return parsed_channels
                .into_iter()
                .map(|channel| (channel, true))
                .collect();
        }
        self.inspection
            .as_ref()
            .map(|inspection| {
                inspection
                    .channels
                    .iter()
                    .map(|channel| (channel.id.clone(), true))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl GuiPlotPyramid {
    fn from_series(data_revision: u64, series: &[WorkflowPlotSeries]) -> Self {
        Self {
            data_revision,
            series: series
                .iter()
                .map(GuiPlotPyramidSeries::from_series)
                .collect(),
        }
    }
}

impl GuiPlotPyramidSeries {
    fn from_series(series: &WorkflowPlotSeries) -> Self {
        let mut levels = Vec::new();
        let mut stride = 2usize;
        let mut last_point_count = series.points.len();
        while stride < series.points.len() {
            let points = min_max_decimate_by_stride(&series.points, stride);
            if !points.is_empty() && points.len() < last_point_count {
                last_point_count = points.len();
                levels.push(GuiPlotPyramidLevel { stride, points });
            }
            if stride > usize::MAX / 2 {
                break;
            }
            stride *= 2;
        }
        Self {
            name: series.name.clone(),
            unit: series.unit.clone(),
            raw_point_count: series.points.len(),
            levels,
        }
    }
}

pub fn parse_channel_csv(channels: &str) -> Vec<String> {
    channels
        .split(',')
        .map(str::trim)
        .filter(|channel| !channel.is_empty())
        .map(str::to_string)
        .collect()
}

fn quantize_f64(value: f64, quantum: f64) -> i64 {
    if !value.is_finite() || !quantum.is_finite() || quantum <= 0.0 {
        return 0;
    }
    let scaled = (value / quantum).round();
    if scaled > i64::MAX as f64 {
        i64::MAX
    } else if scaled < i64::MIN as f64 {
        i64::MIN
    } else {
        scaled as i64
    }
}

fn desired_pyramid_stride(visible_point_count: usize, point_budget: usize) -> usize {
    if point_budget == 0 || visible_point_count <= point_budget {
        1
    } else {
        (visible_point_count / point_budget).max(1)
    }
}

fn count_points_in_x_range(points: &[WorkflowPlotPoint], x_min: f64, x_max: f64) -> usize {
    points_in_x_range(points, x_min, x_max).len()
}

fn points_in_x_range(points: &[WorkflowPlotPoint], x_min: f64, x_max: f64) -> &[WorkflowPlotPoint] {
    if points.is_empty() || !x_min.is_finite() || !x_max.is_finite() || x_max < x_min {
        return &[];
    }
    let start = points.partition_point(|point| point.time < x_min);
    let end = points.partition_point(|point| point.time <= x_max);
    &points[start..end]
}

fn min_max_decimate_by_stride(
    points: &[WorkflowPlotPoint],
    stride: usize,
) -> Vec<WorkflowPlotPoint> {
    if points.len() <= 2 || stride <= 1 {
        return points.to_vec();
    }
    let mut output = Vec::with_capacity((points.len() / stride + 1) * 2);
    for chunk_start in (0..points.len()).step_by(stride) {
        let chunk_end = (chunk_start + stride).min(points.len());
        push_min_max_for_index_range(points, chunk_start, chunk_end, &mut output);
    }
    output
}

fn min_max_decimate_for_viewport(
    points: &[WorkflowPlotPoint],
    x_min: f64,
    x_max: f64,
    max_points: usize,
) -> Vec<WorkflowPlotPoint> {
    if points.len() <= max_points || points.len() <= 2 || max_points < 4 || x_max <= x_min {
        return points.to_vec();
    }

    let bucket_count = ((max_points.saturating_sub(2)) / 2).max(1);
    let bucket_width = ((x_max - x_min) / bucket_count as f64).max(f64::EPSILON);
    let mut buckets = vec![DecimationBucket::default(); bucket_count];
    for (index, point) in points.iter().copied().enumerate() {
        let bucket_index = (((point.time - x_min) / bucket_width).floor() as isize)
            .clamp(0, bucket_count as isize - 1) as usize;
        buckets[bucket_index].include(index, point);
    }

    let mut indexed_points = Vec::with_capacity(max_points);
    indexed_points.push((0, points[0]));
    for bucket in buckets {
        bucket.push_indexed_points(&mut indexed_points);
    }
    indexed_points.push((points.len() - 1, points[points.len() - 1]));
    indexed_points.sort_by_key(|(index, _)| *index);
    indexed_points.dedup_by_key(|(index, _)| *index);
    indexed_points
        .into_iter()
        .take(max_points)
        .map(|(_, point)| point)
        .collect()
}

fn push_min_max_for_index_range(
    points: &[WorkflowPlotPoint],
    start: usize,
    end: usize,
    output: &mut Vec<WorkflowPlotPoint>,
) {
    if start >= end {
        return;
    }
    let mut min_index = start;
    let mut max_index = start;
    for index in start + 1..end {
        if points[index].value < points[min_index].value {
            min_index = index;
        }
        if points[index].value > points[max_index].value {
            max_index = index;
        }
    }
    if min_index <= max_index {
        output.push(points[min_index]);
        if max_index != min_index {
            output.push(points[max_index]);
        }
    } else {
        output.push(points[max_index]);
        output.push(points[min_index]);
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct DecimationBucket {
    min: Option<(usize, WorkflowPlotPoint)>,
    max: Option<(usize, WorkflowPlotPoint)>,
}

impl DecimationBucket {
    fn include(&mut self, index: usize, point: WorkflowPlotPoint) {
        if self
            .min
            .map(|(_, min_point)| point.value < min_point.value)
            .unwrap_or(true)
        {
            self.min = Some((index, point));
        }
        if self
            .max
            .map(|(_, max_point)| point.value > max_point.value)
            .unwrap_or(true)
        {
            self.max = Some((index, point));
        }
    }

    fn push_indexed_points(self, output: &mut Vec<(usize, WorkflowPlotPoint)>) {
        match (self.min, self.max) {
            (Some(min), Some(max)) if min.0 <= max.0 => {
                output.push(min);
                if max.0 != min.0 {
                    output.push(max);
                }
            }
            (Some(min), Some(max)) => {
                output.push(max);
                output.push(min);
            }
            (Some(point), None) | (None, Some(point)) => output.push(point),
            (None, None) => {}
        }
    }
}

fn render_summary(series: &[GuiRenderedPlotSeries], resolution: GuiPlotResolution) -> String {
    if series.is_empty() {
        return "No plot series loaded".to_string();
    }
    let rendered_points = series
        .iter()
        .map(|series| series.rendered_point_count)
        .sum::<usize>();
    let visible_points = series
        .iter()
        .map(|series| series.visible_point_count)
        .sum::<usize>();
    let raw_points = series
        .iter()
        .map(|series| series.raw_point_count)
        .sum::<usize>();
    let max_stride = series
        .iter()
        .map(|series| series.pyramid_stride)
        .max()
        .unwrap_or(1);
    format!(
        "{} rendered points from {} visible / {} loaded points across {} channel(s); {} mode; pyramid stride up to {}x",
        rendered_points,
        visible_points,
        raw_points,
        series.len(),
        resolution.label(),
        max_stride
    )
}

fn write_filter_config(output: &mut String, channel: &str, filter: &GuiConfigFilterDraft) {
    writeln!(output, "[[filters]]").expect("writing to String should succeed");
    writeln!(output, "type = {}", toml_string(filter.kind.toml_type()))
        .expect("writing to String should succeed");
    writeln!(output, "channel = {}", toml_string(channel))
        .expect("writing to String should succeed");
    match filter.kind {
        GuiConfigFilterKind::MovingAverage | GuiConfigFilterKind::MovingMedian => {
            writeln!(output, "window_samples = {}", filter.window_samples.max(1))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::LowPass
        | GuiConfigFilterKind::HighPass
        | GuiConfigFilterKind::HighPassBaseline => {
            writeln!(
                output,
                "cutoff_hz = {}",
                positive_f64(filter.cutoff_hz, 10.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Offset => {
            writeln!(output, "offset_v = {}", finite_f64(filter.offset_v, 0.0))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Gain => {
            writeln!(output, "gain = {}", finite_f64(filter.gain, 1.0))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Clamp => {
            let min_v = finite_f64(filter.min_v, 0.0);
            let max_v = finite_f64(filter.max_v, min_v + 1.0);
            writeln!(output, "min_v = {min_v}").expect("writing to String should succeed");
            writeln!(output, "max_v = {}", max_v.max(min_v + f64::EPSILON))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Deadband => {
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(filter.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::BaselineSubtract => {
            writeln!(
                output,
                "baseline_v = {}",
                finite_f64(filter.baseline_v, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Log | GuiConfigFilterKind::Exp => {
            writeln!(output, "base = {}", positive_f64(filter.base, 10.0))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Normalize => {
            writeln!(
                output,
                "mode = {}",
                toml_string(filter.normalize_mode.toml_value())
            )
            .expect("writing to String should succeed");
            if filter.normalize_mode == GuiConfigNormalizeMode::Range {
                let input_min_v = finite_f64(filter.input_min_v, 0.0);
                let input_max_v = finite_f64(filter.input_max_v, input_min_v + 1.0);
                let output_min = finite_f64(filter.output_min, 0.0);
                let output_max = finite_f64(filter.output_max, output_min + 1.0);
                writeln!(output, "input_min_v = {input_min_v}")
                    .expect("writing to String should succeed");
                writeln!(
                    output,
                    "input_max_v = {}",
                    input_max_v.max(input_min_v + f64::EPSILON)
                )
                .expect("writing to String should succeed");
                writeln!(output, "output_min = {output_min}")
                    .expect("writing to String should succeed");
                writeln!(
                    output,
                    "output_max = {}",
                    output_max.max(output_min + f64::EPSILON)
                )
                .expect("writing to String should succeed");
            }
        }
        GuiConfigFilterKind::SoftLimit => {
            writeln!(output, "limit_v = {}", positive_f64(filter.limit_v, 1.0))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Resample => {
            writeln!(
                output,
                "sample_interval_s = {}",
                positive_f64(filter.sample_interval_s, 0.001)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Downsample => {
            writeln!(output, "factor = {}", filter.factor.max(1))
                .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Decimate => {
            writeln!(output, "factor = {}", filter.factor.max(1))
                .expect("writing to String should succeed");
            writeln!(
                output,
                "cutoff_hz = {}",
                positive_f64(filter.cutoff_hz, 10.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigFilterKind::Invert
        | GuiConfigFilterKind::DcRemove
        | GuiConfigFilterKind::AbsoluteValue
        | GuiConfigFilterKind::Square
        | GuiConfigFilterKind::SquareRoot
        | GuiConfigFilterKind::Tanh
        | GuiConfigFilterKind::Sigmoid => {}
    }
}

fn write_criterion_config(
    output: &mut String,
    channel: &str,
    index: usize,
    criterion: &GuiConfigCriterionDraft,
) {
    writeln!(output, "[[criteria]]").expect("writing to String should succeed");
    writeln!(
        output,
        "id = {}",
        toml_string(&format!(
            "{}_{}_{}",
            sanitize_identifier(channel),
            criterion.kind.id_suffix(),
            index + 1
        ))
    )
    .expect("writing to String should succeed");
    writeln!(output, "type = {}", toml_string(criterion.kind.toml_type()))
        .expect("writing to String should succeed");
    writeln!(output, "channel = {}", toml_string(channel))
        .expect("writing to String should succeed");
    match criterion.kind {
        GuiConfigCriterionKind::MinimumVoltage | GuiConfigCriterionKind::MaximumVoltage => {
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::StateTransitions => {
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "expected_count = {}",
                criterion.expected_count.max(1)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::PulseWidth => {
            writeln!(
                output,
                "state = {}",
                toml_string(criterion.state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "min_width_s = {}",
                non_negative_f64(criterion.min_width_s, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "max_width_s = {}",
                non_negative_f64(criterion.max_width_s, criterion.min_width_s.max(0.0))
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::TransientDuration => {
            writeln!(
                output,
                "expected_state = {}",
                toml_string(criterion.expected_state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "max_duration_s = {}",
                non_negative_f64(criterion.max_duration_s, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::TransientEvent => {
            writeln!(
                output,
                "event_kind = {}",
                toml_string(criterion.event_kind.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "expected_state = {}",
                toml_string(criterion.expected_state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "max_duration_s = {}",
                non_negative_f64(criterion.max_duration_s, 0.0)
            )
            .expect("writing to String should succeed");
            let start_time_s = non_negative_f64(criterion.start_time_s, 0.0);
            let end_time_s = non_negative_f64(criterion.end_time_s, start_time_s).max(start_time_s);
            writeln!(output, "start_time_s = {start_time_s}")
                .expect("writing to String should succeed");
            writeln!(output, "end_time_s = {end_time_s}")
                .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::StableStateDuration => {
            writeln!(
                output,
                "state = {}",
                toml_string(criterion.state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "threshold_v = {}",
                finite_f64(criterion.threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "min_duration_s = {}",
                non_negative_f64(criterion.min_duration_s, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::RiseFallTime => {
            let low_threshold_v = finite_f64(criterion.low_threshold_v, 0.5);
            let high_threshold_v = finite_f64(criterion.high_threshold_v, 4.5);
            writeln!(
                output,
                "direction = {}",
                toml_string(criterion.direction.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(output, "low_threshold_v = {low_threshold_v}")
                .expect("writing to String should succeed");
            writeln!(
                output,
                "high_threshold_v = {}",
                high_threshold_v.max(low_threshold_v + f64::EPSILON)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "max_duration_s = {}",
                non_negative_f64(criterion.max_duration_s, 0.0)
            )
            .expect("writing to String should succeed");
        }
        GuiConfigCriterionKind::ResponseLatency => {
            let source_channel = if criterion.source_channel.trim().is_empty() {
                channel
            } else {
                &criterion.source_channel
            };
            writeln!(output, "source_channel = {}", toml_string(source_channel))
                .expect("writing to String should succeed");
            writeln!(
                output,
                "source_threshold_v = {}",
                finite_f64(criterion.source_threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "target_threshold_v = {}",
                finite_f64(criterion.target_threshold_v, 0.0)
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "source_state = {}",
                toml_string(criterion.source_state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "expected_target_state = {}",
                toml_string(criterion.expected_target_state.toml_value())
            )
            .expect("writing to String should succeed");
            writeln!(
                output,
                "max_latency_s = {}",
                non_negative_f64(criterion.max_latency_s, 0.0)
            )
            .expect("writing to String should succeed");
        }
    }
}

fn toml_string(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len() + 2);
    escaped.push('"');
    for character in value.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            _ => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

fn toml_string_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| toml_string(value))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

fn sanitize_identifier(value: &str) -> String {
    let identifier = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string();
    if identifier.is_empty() {
        "channel".to_string()
    } else {
        identifier
    }
}

fn finite_f64(value: f64, fallback: f64) -> f64 {
    if value.is_finite() {
        value
    } else {
        fallback
    }
}

fn positive_f64(value: f64, fallback: f64) -> f64 {
    if value.is_finite() && value > 0.0 {
        value
    } else {
        fallback
    }
}

fn non_negative_f64(value: f64, fallback: f64) -> f64 {
    if value.is_finite() && value >= 0.0 {
        value
    } else {
        fallback
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::{Path, PathBuf};

    fn unique_gui_dir(name: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time should be available")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "ferrisoxide-gui-{name}-{}-{nonce}",
            std::process::id()
        ))
    }

    fn synthetic_plot_series(
        name: &str,
        point_count: usize,
        spike_index: Option<usize>,
    ) -> WorkflowPlotSeries {
        let points = (0..point_count)
            .map(|index| WorkflowPlotPoint {
                time: index as f64,
                value: if Some(index) == spike_index {
                    1_000.0
                } else {
                    (index % 17) as f64
                },
            })
            .collect();
        WorkflowPlotSeries {
            name: name.to_string(),
            unit: "V".to_string(),
            points,
        }
    }

    fn basic_waveform_csv_path() -> String {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string()
    }

    #[test]
    fn parses_channel_csv_for_request_building() {
        assert_eq!(
            parse_channel_csv(" input_v, output_v ,, "),
            vec!["input_v".to_string(), "output_v".to_string()]
        );
    }

    #[test]
    fn session_inspects_and_scaffolds_csv_source() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let mut session = GuiSession {
            input_path,
            ..GuiSession::default()
        };

        session.inspect_current_source();
        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert!(session
            .inspection_text
            .contains("FerrisOxide Source Inspection"));

        session.scaffold_config();
        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert!(session.config_text.contains("[[criteria]]"));
    }

    #[test]
    fn session_loads_headers_and_assigns_channels() {
        let mut session = GuiSession {
            input_path: basic_waveform_csv_path(),
            ..GuiSession::default()
        };

        session.load_channels_from_csv();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert_eq!(
            session.csv_headers,
            vec![
                "time".to_string(),
                "input_v".to_string(),
                "output_v".to_string()
            ]
        );
        assert_eq!(session.time_column, "time");
        assert_eq!(
            session.selected_channel_names(),
            vec!["input_v".to_string(), "output_v".to_string()]
        );
        assert_eq!(session.channel_csv, "input_v,output_v");
    }

    #[test]
    fn session_syncs_config_builder_from_loaded_channels() {
        let mut session = GuiSession {
            input_path: basic_waveform_csv_path(),
            ..GuiSession::default()
        };
        session.load_channels_from_csv();

        session.load_config_builder_from_source();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert_eq!(
            session
                .config_draft
                .channels
                .iter()
                .map(|channel| channel.channel.as_str())
                .collect::<Vec<_>>(),
            vec!["input_v", "output_v"]
        );
        assert_eq!(
            session.config_draft.selected_channel.as_deref(),
            Some("input_v")
        );
    }

    #[test]
    fn session_generates_config_text_from_channel_builder() {
        let mut session = GuiSession {
            input_path: basic_waveform_csv_path(),
            ..GuiSession::default()
        };
        session.load_channels_from_csv();
        session.load_config_builder_from_source();
        let channel = session
            .config_channel_mut("input_v")
            .expect("input channel config should exist");
        channel.filters.push(GuiConfigFilterDraft {
            kind: GuiConfigFilterKind::Offset,
            offset_v: -0.1,
            ..GuiConfigFilterDraft::default()
        });
        channel.criteria.push(GuiConfigCriterionDraft {
            kind: GuiConfigCriterionKind::StableStateDuration,
            threshold_v: 2.5,
            state: GuiConfigState::High,
            min_duration_s: 0.5,
            ..GuiConfigCriterionDraft::default()
        });

        session.generate_config_text_from_builder();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert!(session
            .config_text
            .contains("channels = [\"input_v\", \"output_v\"]"));
        assert!(session.config_text.contains("type = \"offset\""));
        assert!(session.config_text.contains("channel = \"input_v\""));
        assert!(session.config_text.contains("offset_v = -0.1"));
        assert!(session
            .config_text
            .contains("type = \"stable_state_duration\""));
        assert!(session.config_text.contains("state = \"high\""));
        assert!(session.config_text.contains("min_duration_s = 0.5"));
    }

    #[test]
    fn session_loads_existing_config_text_from_path() {
        let temp_root = unique_gui_dir("load-config");
        fs::create_dir_all(&temp_root).expect("temp root should be created");
        let config_path = temp_root.join("existing-config.toml");
        let config_text = "[input]\ntime_column = \"time\"\nchannels = [\"input_v\"]\n";
        fs::write(&config_path, config_text).expect("existing config should be written");
        let mut session = GuiSession::default();

        session.load_config_text_from_path(config_path.display().to_string());

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert_eq!(session.config_path, config_path.display().to_string());
        assert_eq!(session.config_text, config_text);

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn session_preserves_config_builder_rows_when_source_resyncs() {
        let mut session = GuiSession {
            input_path: basic_waveform_csv_path(),
            ..GuiSession::default()
        };
        session.load_channels_from_csv();
        session.load_config_builder_from_source();
        session
            .config_channel_mut("input_v")
            .expect("input channel config should exist")
            .criteria
            .push(GuiConfigCriterionDraft {
                kind: GuiConfigCriterionKind::MaximumVoltage,
                threshold_v: 4.2,
                ..GuiConfigCriterionDraft::default()
            });
        session
            .channel_selections
            .iter_mut()
            .find(|selection| selection.header == "input_v")
            .expect("input channel should be assigned")
            .unit = "mV".to_string();

        session.refresh_channel_selection_summary();

        let channel = session
            .config_draft
            .channels
            .iter()
            .find(|channel| channel.channel == "input_v")
            .expect("input channel config should remain");
        assert_eq!(channel.unit, "mV");
        assert_eq!(channel.criteria.len(), 1);
        assert_eq!(channel.criteria[0].threshold_v, 4.2);
    }

    #[test]
    fn session_preserves_units_when_time_column_changes() {
        let mut session = GuiSession {
            input_path: basic_waveform_csv_path(),
            ..GuiSession::default()
        };
        session.load_channels_from_csv();
        session
            .channel_selections
            .iter_mut()
            .find(|selection| selection.header == "input_v")
            .expect("input channel should be assigned")
            .unit = "mV".to_string();
        session.refresh_channel_selection_summary();

        session.inspect_current_source();

        assert!(session.inspection_text.contains("unit=mV"));
        session.set_time_column("input_v");
        assert_eq!(
            session.selected_channel_names(),
            vec!["time".to_string(), "output_v".to_string()]
        );
    }

    #[test]
    fn session_runs_csv_bundle_and_records_artifacts() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let temp_root = unique_gui_dir("bundle");
        let output_dir = temp_root.join("out");
        let mut session = GuiSession {
            input_path,
            config_path,
            output_dir: output_dir.display().to_string(),
            ..GuiSession::default()
        };

        session.evaluate_current_bundle();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert!(session.artifact_rows().contains(&"report.json".to_string()));
        assert!(output_dir.join("bundle-summary.json").exists());

        fs::remove_dir_all(&temp_root).expect("temp root should be removable");
    }

    #[test]
    fn session_loads_plot_series_from_csv_config() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let mut session = GuiSession {
            input_path,
            config_path,
            ..GuiSession::default()
        };
        session.load_channels_from_csv();

        session.load_plot_series();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert_eq!(session.plot_series.len(), 2);
        assert_eq!(session.plot_series[0].points.len(), 5);
    }

    #[test]
    fn session_filters_plot_series_without_mutating_source_channels() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let config_path = Path::new(manifest_dir)
            .join("../../examples/basic-config.toml")
            .display()
            .to_string();
        let mut session = GuiSession {
            input_path,
            config_path,
            ..GuiSession::default()
        };
        session.load_channels_from_csv();
        session
            .plot_channel_selections
            .iter_mut()
            .find(|selection| selection.channel == "input_v")
            .expect("input plot channel should be assigned")
            .enabled = false;

        session.load_plot_series();

        assert!(matches!(session.status, WorkflowStatus::Succeeded(_)));
        assert_eq!(session.selected_channel_names().len(), 2);
        assert_eq!(session.plot_series.len(), 1);
        assert_eq!(session.plot_series[0].name, "output_v");
    }

    #[test]
    fn session_rejects_plot_load_without_selected_channels() {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(manifest_dir)
            .join("../../examples/basic-waveform.csv")
            .display()
            .to_string();
        let mut session = GuiSession {
            input_path,
            ..GuiSession::default()
        };
        session.load_channels_from_csv();
        for selection in &mut session.plot_channel_selections {
            selection.enabled = false;
        }

        session.load_plot_series();

        assert!(matches!(
            session.status,
            WorkflowStatus::Failed(ref message)
                if message == "Select at least one channel to plot"
        ));
        assert!(session.plot_series.is_empty());
    }

    #[test]
    fn session_downsamples_rendered_plot_points_to_resolution_budget() {
        let mut session = GuiSession::default();
        session.set_plot_series_for_testing(vec![synthetic_plot_series("input_v", 10_000, None)]);
        session.set_plot_resolution(GuiPlotResolution::Fast);
        let budget = GuiPlotResolution::Fast
            .render_point_budget(100.0)
            .expect("fast mode should have a render budget");

        let rendered = session.rendered_plot_series_for_viewport(0.0, 9_999.0, 100.0);

        assert_eq!(session.plot_series[0].points.len(), 10_000);
        assert_eq!(rendered.len(), 1);
        assert!(rendered[0].rendered_point_count <= budget);
        assert!(rendered[0].rendered_point_count < rendered[0].visible_point_count);
    }

    #[test]
    fn min_max_decimation_preserves_narrow_spikes() {
        let mut session = GuiSession::default();
        session.set_plot_series_for_testing(vec![synthetic_plot_series(
            "input_v",
            20_000,
            Some(12_345),
        )]);
        session.set_plot_resolution(GuiPlotResolution::Fast);

        let rendered = session.rendered_plot_series_for_viewport(0.0, 19_999.0, 160.0);

        assert!(rendered[0]
            .points
            .iter()
            .any(|point| point[0] == 12_345.0 && point[1] == 1_000.0));
    }

    #[test]
    fn rendered_plot_cache_reuses_and_invalidates_by_viewport_and_resolution() {
        let mut session = GuiSession::default();
        session.set_plot_series_for_testing(vec![synthetic_plot_series("input_v", 5_000, None)]);

        let first = session.rendered_plot_series_for_viewport(0.0, 4_999.0, 200.0);
        let misses_after_first = session.plot_render_cache_misses;
        let second = session.rendered_plot_series_for_viewport(0.0, 4_999.0, 200.0);

        assert_eq!(first, second);
        assert_eq!(misses_after_first, 1);
        assert_eq!(session.plot_render_cache_hits, 1);

        session.set_plot_resolution(GuiPlotResolution::Detailed);
        let _third = session.rendered_plot_series_for_viewport(0.0, 4_999.0, 200.0);

        assert_eq!(session.plot_render_cache_misses, 2);
    }

    #[test]
    fn plot_pyramid_supplies_coarser_levels_for_large_viewports() {
        let mut session = GuiSession::default();
        session.set_plot_series_for_testing(vec![synthetic_plot_series("input_v", 65_536, None)]);
        session.set_plot_resolution(GuiPlotResolution::Fast);

        let strides = session.plot_pyramid_strides_for_channel("input_v");
        assert!(strides.iter().any(|stride| *stride > 1));

        let full_view = session.rendered_plot_series_for_viewport(0.0, 65_535.0, 120.0);
        assert!(full_view[0].pyramid_stride > 1);

        let zoomed_view = session.rendered_plot_series_for_viewport(0.0, 50.0, 120.0);
        assert_eq!(zoomed_view[0].pyramid_stride, 1);
        assert_eq!(zoomed_view[0].rendered_point_count, 51);
    }
}

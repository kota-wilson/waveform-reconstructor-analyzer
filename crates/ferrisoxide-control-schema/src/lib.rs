//! Versioned production control config schema for FerrisOxide controller workflows.
//!
//! This crate owns schema data structures and validation helpers only. It does
//! not simulate controllers, parse waveform CSV, evaluate test verification
//! criteria, render reports, talk to DAQ hardware, bind HALs, or integrate RTOS
//! SDKs.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub const CURRENT_CONTROL_SCHEMA_VERSION: &str = "0.1.0";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProductionControlConfig {
    pub package: ControlPackageMetadata,
    pub target: ControlTargetProfile,
    pub approval: ApprovalMetadata,
    pub timing: ControlTiming,
    pub inputs: Vec<ControlInput>,
    pub outputs: Vec<ControlOutput>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub thresholds: Vec<ControlThreshold>,
    pub modes: Vec<ControlMode>,
    pub state_machines: Vec<StateMachine>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timing_rules: Vec<TimingRule>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ControlAction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fault_responses: Vec<FaultResponse>,
}

impl ProductionControlConfig {
    pub fn validate(&self) -> Result<(), ControlConfigValidationReport> {
        let mut report = ControlConfigValidationReport::new();

        validate_non_empty("package.name", &self.package.name, &mut report);
        validate_non_empty("package.version", &self.package.version, &mut report);
        if self.package.schema_version != CURRENT_CONTROL_SCHEMA_VERSION {
            report.push(ControlConfigValidationError::new(
                "package.schema_version",
                ControlConfigValidationErrorKind::SchemaVersionMismatch,
                format!(
                    "expected schema version `{CURRENT_CONTROL_SCHEMA_VERSION}`, got `{}`",
                    self.package.schema_version
                ),
            ));
        }

        validate_non_empty("target.identifier", &self.target.identifier, &mut report);
        validate_approval(&self.approval, &mut report);
        validate_timing(&self.timing, &mut report);

        let input_ids = collect_ids("inputs", &self.inputs, ControlInput::id, &mut report);
        let output_ids = collect_ids("outputs", &self.outputs, ControlOutput::id, &mut report);
        let mode_ids = collect_ids("modes", &self.modes, ControlMode::id, &mut report);
        let machine_ids = collect_ids(
            "state_machines",
            &self.state_machines,
            StateMachine::id,
            &mut report,
        );
        let action_ids = collect_ids("actions", &self.actions, ControlAction::id, &mut report);
        let fault_ids = collect_ids(
            "fault_responses",
            &self.fault_responses,
            FaultResponse::id,
            &mut report,
        );
        let threshold_ids = collect_ids(
            "thresholds",
            &self.thresholds,
            ControlThreshold::id,
            &mut report,
        );
        let _timing_rule_ids = collect_ids(
            "timing_rules",
            &self.timing_rules,
            TimingRule::id,
            &mut report,
        );

        if self.inputs.is_empty() {
            report.push(ControlConfigValidationError::new(
                "inputs",
                ControlConfigValidationErrorKind::MissingInput,
                "at least one control input is required",
            ));
        }
        if self.outputs.is_empty() {
            report.push(ControlConfigValidationError::new(
                "outputs",
                ControlConfigValidationErrorKind::MissingOutput,
                "at least one control output is required",
            ));
        }
        if self.modes.is_empty() {
            report.push(ControlConfigValidationError::new(
                "modes",
                ControlConfigValidationErrorKind::InvalidMode,
                "at least one control mode is required",
            ));
        }
        if self.state_machines.is_empty() {
            report.push(ControlConfigValidationError::new(
                "state_machines",
                ControlConfigValidationErrorKind::InvalidStateMachine,
                "at least one state machine is required",
            ));
        }

        for (index, input) in self.inputs.iter().enumerate() {
            validate_input(index, input, &mut report);
        }
        for (index, output) in self.outputs.iter().enumerate() {
            validate_output(index, output, &mut report);
        }
        for (index, threshold) in self.thresholds.iter().enumerate() {
            validate_threshold(index, threshold, &input_ids, self, &mut report);
        }
        for (index, mode) in self.modes.iter().enumerate() {
            validate_mode(index, mode, &machine_ids, &action_ids, &mut report);
        }
        for (index, action) in self.actions.iter().enumerate() {
            validate_action(
                index,
                action,
                &output_ids,
                &mode_ids,
                &fault_ids,
                &mut report,
            );
        }
        for (index, fault) in self.fault_responses.iter().enumerate() {
            validate_fault_response(index, fault, &action_ids, &mode_ids, &mut report);
        }
        for (index, timing_rule) in self.timing_rules.iter().enumerate() {
            validate_timing_rule(index, timing_rule, &machine_ids, &action_ids, &mut report);
        }
        let references = ControlReferenceSets {
            input_ids: &input_ids,
            threshold_ids: &threshold_ids,
            mode_ids: &mode_ids,
            action_ids: &action_ids,
            fault_ids: &fault_ids,
        };
        for (index, machine) in self.state_machines.iter().enumerate() {
            validate_state_machine(index, machine, &references, &mut report);
        }

        report.into_result()
    }
}

pub fn parse_control_config_json(
    input: &str,
) -> Result<ProductionControlConfig, ControlConfigValidationError> {
    serde_json::from_str(input).map_err(|error| {
        ControlConfigValidationError::new(
            "production-control-config.json",
            ControlConfigValidationErrorKind::ParseError,
            error.to_string(),
        )
    })
}

pub fn parse_control_config_toml(
    input: &str,
) -> Result<ProductionControlConfig, ControlConfigValidationError> {
    toml::from_str(input).map_err(|error| {
        ControlConfigValidationError::new(
            "production-control-config.toml",
            ControlConfigValidationErrorKind::ParseError,
            error.to_string(),
        )
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlPackageMetadata {
    pub name: String,
    pub version: String,
    pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ControlPackageMetadata {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            schema_version: CURRENT_CONTROL_SCHEMA_VERSION.to_string(),
            description: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalMetadata {
    pub status: ApprovalStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approved_at: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStatus {
    Draft,
    Reviewed,
    Approved,
    Retired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlTargetProfile {
    pub kind: ControlTargetKind,
    pub identifier: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlTargetKind {
    DesktopSimulation,
    ControllerRuntime,
    EmbeddedRuntime,
    TestStand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlTiming {
    pub clock: ClockSource,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_loop_period_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_loop_jitter_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub startup_timeout_s: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClockSource {
    Simulated,
    Monotonic,
    HardwareTimer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlInput {
    pub id: String,
    pub source: String,
    pub signal: SignalKind,
    pub unit: ControlUnit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sample_period_s: Option<f64>,
}

impl ControlInput {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlOutput {
    pub id: String,
    pub sink: String,
    pub signal: SignalKind,
    pub unit: ControlUnit,
    pub safe_state: OutputValue,
}

impl ControlOutput {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalKind {
    AnalogVoltage,
    Digital,
    Gpio,
    Pwm,
    Virtual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlThreshold {
    pub id: String,
    pub input: String,
    pub role: ThresholdRole,
    pub value: UnitValue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hysteresis: Option<UnitValue>,
}

impl ControlThreshold {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThresholdRole {
    Low,
    High,
    Decision,
    Fault,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlMode {
    pub id: String,
    pub initial_state_machine: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_state_machines: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exit_actions: Vec<String>,
}

impl ControlMode {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateMachine {
    pub id: String,
    pub initial_state: String,
    pub states: Vec<StateDefinition>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<StateTransition>,
}

impl StateMachine {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateDefinition {
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entry_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exit_actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateTransition {
    pub id: String,
    pub from: String,
    pub to: String,
    pub condition: TransitionCondition,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fault_response: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransitionCondition {
    InputAbove { input: String, threshold: String },
    InputBelow { input: String, threshold: String },
    InputState { input: String, state: DigitalState },
    TimerElapsed { duration_s: f64 },
    ModeIs { mode: String },
    Always,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimingRule {
    pub id: String,
    pub scope: TimingRuleScope,
    pub target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_latency_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_interval_s: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_s: Option<f64>,
}

impl TimingRule {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimingRuleScope {
    ControlLoop,
    StateMachine,
    Transition,
    Action,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ControlAction {
    SetOutput {
        id: String,
        output: String,
        value: OutputValue,
    },
    EnterMode {
        id: String,
        mode: String,
    },
    RaiseFault {
        id: String,
        fault_response: String,
    },
    NoOp {
        id: String,
    },
}

impl ControlAction {
    fn id(&self) -> &str {
        match self {
            Self::SetOutput { id, .. }
            | Self::EnterMode { id, .. }
            | Self::RaiseFault { id, .. }
            | Self::NoOp { id } => id,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaultResponse {
    pub id: String,
    pub severity: FaultSeverity,
    pub latch: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safe_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<String>,
}

impl FaultResponse {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultSeverity {
    Warning,
    Recoverable,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalState {
    Low,
    High,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputValue {
    Analog { value: UnitValue },
    Digital { state: DigitalState },
    PwmDuty { duty_cycle: f64 },
    Named { state: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct UnitValue {
    pub value: f64,
    pub unit: ControlUnit,
}

impl UnitValue {
    pub const fn new(value: f64, unit: ControlUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlUnit {
    #[serde(rename = "V")]
    Volt,
    #[serde(rename = "s")]
    Second,
    #[serde(rename = "Hz")]
    Hertz,
    #[serde(rename = "percent")]
    Percent,
    #[serde(rename = "bool")]
    Boolean,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "unitless")]
    Unitless,
}

impl ControlUnit {
    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Volt => "V",
            Self::Second => "s",
            Self::Hertz => "Hz",
            Self::Percent => "percent",
            Self::Boolean => "bool",
            Self::Count => "count",
            Self::Unitless => "unitless",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlConfigValidationReport {
    pub errors: Vec<ControlConfigValidationError>,
}

impl ControlConfigValidationReport {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    fn push(&mut self, error: ControlConfigValidationError) {
        self.errors.push(error);
    }

    fn into_result(self) -> Result<(), Self> {
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

impl Default for ControlConfigValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ControlConfigValidationReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.errors.is_empty() {
            return write!(formatter, "control config validation passed");
        }

        writeln!(
            formatter,
            "control config validation failed with {} error(s):",
            self.errors.len()
        )?;
        for error in &self.errors {
            writeln!(formatter, "- {error}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlConfigValidationError {
    pub field: String,
    pub kind: ControlConfigValidationErrorKind,
    pub message: String,
}

impl ControlConfigValidationError {
    pub fn new(
        field: impl Into<String>,
        kind: ControlConfigValidationErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            field: field.into(),
            kind,
            message: message.into(),
        }
    }
}

impl fmt::Display for ControlConfigValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {} ({})",
            self.field,
            self.message,
            self.kind.as_str()
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlConfigValidationErrorKind {
    ParseError,
    SchemaVersionMismatch,
    InvalidMetadata,
    InvalidApproval,
    InvalidTiming,
    DuplicateIdentifier,
    MissingInput,
    MissingOutput,
    MissingThreshold,
    MissingMode,
    MissingStateMachine,
    MissingState,
    MissingAction,
    MissingFaultResponse,
    InvalidInput,
    InvalidOutput,
    InvalidThreshold,
    InvalidMode,
    InvalidStateMachine,
    InvalidTransition,
    InvalidTimingRule,
    InvalidAction,
    InvalidFaultResponse,
    InvalidUnit,
}

impl ControlConfigValidationErrorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::SchemaVersionMismatch => "schema_version_mismatch",
            Self::InvalidMetadata => "invalid_metadata",
            Self::InvalidApproval => "invalid_approval",
            Self::InvalidTiming => "invalid_timing",
            Self::DuplicateIdentifier => "duplicate_identifier",
            Self::MissingInput => "missing_input",
            Self::MissingOutput => "missing_output",
            Self::MissingThreshold => "missing_threshold",
            Self::MissingMode => "missing_mode",
            Self::MissingStateMachine => "missing_state_machine",
            Self::MissingState => "missing_state",
            Self::MissingAction => "missing_action",
            Self::MissingFaultResponse => "missing_fault_response",
            Self::InvalidInput => "invalid_input",
            Self::InvalidOutput => "invalid_output",
            Self::InvalidThreshold => "invalid_threshold",
            Self::InvalidMode => "invalid_mode",
            Self::InvalidStateMachine => "invalid_state_machine",
            Self::InvalidTransition => "invalid_transition",
            Self::InvalidTimingRule => "invalid_timing_rule",
            Self::InvalidAction => "invalid_action",
            Self::InvalidFaultResponse => "invalid_fault_response",
            Self::InvalidUnit => "invalid_unit",
        }
    }
}

fn validate_non_empty(field: &str, value: &str, report: &mut ControlConfigValidationReport) {
    if value.trim().is_empty() {
        report.push(ControlConfigValidationError::new(
            field,
            ControlConfigValidationErrorKind::InvalidMetadata,
            "value must not be empty",
        ));
    }
}

fn collect_ids<T>(
    field: &str,
    items: &[T],
    id: impl Fn(&T) -> &str,
    report: &mut ControlConfigValidationReport,
) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        let item_id = id(item);
        validate_non_empty(&format!("{field}[{index}].id"), item_id, report);
        if !item_id.is_empty() && !ids.insert(item_id.to_string()) {
            report.push(ControlConfigValidationError::new(
                format!("{field}[{index}].id"),
                ControlConfigValidationErrorKind::DuplicateIdentifier,
                format!("duplicate identifier `{item_id}`"),
            ));
        }
    }
    ids
}

fn validate_reference(
    field: &str,
    value: &str,
    valid_ids: &BTreeSet<String>,
    missing_kind: ControlConfigValidationErrorKind,
    report: &mut ControlConfigValidationReport,
) {
    validate_non_empty(field, value, report);
    if !value.is_empty() && !valid_ids.contains(value) {
        report.push(ControlConfigValidationError::new(
            field,
            missing_kind,
            format!("reference `{value}` is not defined"),
        ));
    }
}

fn validate_approval(approval: &ApprovalMetadata, report: &mut ControlConfigValidationReport) {
    if approval.status == ApprovalStatus::Approved {
        if approval
            .approved_by
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            report.push(ControlConfigValidationError::new(
                "approval.approved_by",
                ControlConfigValidationErrorKind::InvalidApproval,
                "approved configs must identify the approver",
            ));
        }
        if approval
            .approved_at
            .as_deref()
            .unwrap_or_default()
            .is_empty()
        {
            report.push(ControlConfigValidationError::new(
                "approval.approved_at",
                ControlConfigValidationErrorKind::InvalidApproval,
                "approved configs must record approval time",
            ));
        }
    }
}

fn validate_timing(timing: &ControlTiming, report: &mut ControlConfigValidationReport) {
    validate_optional_positive_finite(
        "timing.control_loop_period_s",
        timing.control_loop_period_s,
        ControlConfigValidationErrorKind::InvalidTiming,
        report,
    );
    validate_optional_non_negative_finite(
        "timing.max_loop_jitter_s",
        timing.max_loop_jitter_s,
        ControlConfigValidationErrorKind::InvalidTiming,
        report,
    );
    validate_optional_non_negative_finite(
        "timing.startup_timeout_s",
        timing.startup_timeout_s,
        ControlConfigValidationErrorKind::InvalidTiming,
        report,
    );
}

fn validate_input(index: usize, input: &ControlInput, report: &mut ControlConfigValidationReport) {
    let field = format!("inputs[{index}]");
    validate_non_empty(&format!("{field}.source"), &input.source, report);
    validate_optional_positive_finite(
        &format!("{field}.sample_period_s"),
        input.sample_period_s,
        ControlConfigValidationErrorKind::InvalidInput,
        report,
    );
}

fn validate_output(
    index: usize,
    output: &ControlOutput,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("outputs[{index}]");
    validate_non_empty(&format!("{field}.sink"), &output.sink, report);
    validate_output_value(
        &format!("{field}.safe_state"),
        &output.safe_state,
        output.unit,
        report,
    );
}

fn validate_threshold(
    index: usize,
    threshold: &ControlThreshold,
    input_ids: &BTreeSet<String>,
    config: &ProductionControlConfig,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("thresholds[{index}]");
    validate_reference(
        &format!("{field}.input"),
        &threshold.input,
        input_ids,
        ControlConfigValidationErrorKind::MissingInput,
        report,
    );

    let expected_unit = config
        .inputs
        .iter()
        .find(|input| input.id == threshold.input)
        .map(|input| input.unit);
    validate_unit_value(
        &format!("{field}.value"),
        threshold.value,
        expected_unit,
        ControlConfigValidationErrorKind::InvalidThreshold,
        report,
    );
    if let Some(hysteresis) = threshold.hysteresis {
        validate_unit_value(
            &format!("{field}.hysteresis"),
            hysteresis,
            expected_unit,
            ControlConfigValidationErrorKind::InvalidThreshold,
            report,
        );
        if hysteresis.value < 0.0 {
            report.push(ControlConfigValidationError::new(
                format!("{field}.hysteresis"),
                ControlConfigValidationErrorKind::InvalidThreshold,
                "hysteresis must be non-negative",
            ));
        }
    }
}

fn validate_mode(
    index: usize,
    mode: &ControlMode,
    machine_ids: &BTreeSet<String>,
    action_ids: &BTreeSet<String>,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("modes[{index}]");
    validate_reference(
        &format!("{field}.initial_state_machine"),
        &mode.initial_state_machine,
        machine_ids,
        ControlConfigValidationErrorKind::MissingStateMachine,
        report,
    );
    for (machine_index, machine) in mode.enabled_state_machines.iter().enumerate() {
        validate_reference(
            &format!("{field}.enabled_state_machines[{machine_index}]"),
            machine,
            machine_ids,
            ControlConfigValidationErrorKind::MissingStateMachine,
            report,
        );
    }
    for (action_index, action) in mode.entry_actions.iter().enumerate() {
        validate_reference(
            &format!("{field}.entry_actions[{action_index}]"),
            action,
            action_ids,
            ControlConfigValidationErrorKind::MissingAction,
            report,
        );
    }
    for (action_index, action) in mode.exit_actions.iter().enumerate() {
        validate_reference(
            &format!("{field}.exit_actions[{action_index}]"),
            action,
            action_ids,
            ControlConfigValidationErrorKind::MissingAction,
            report,
        );
    }
}

fn validate_action(
    index: usize,
    action: &ControlAction,
    output_ids: &BTreeSet<String>,
    mode_ids: &BTreeSet<String>,
    fault_ids: &BTreeSet<String>,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("actions[{index}]");
    match action {
        ControlAction::SetOutput { output, value, .. } => {
            validate_reference(
                &format!("{field}.output"),
                output,
                output_ids,
                ControlConfigValidationErrorKind::MissingOutput,
                report,
            );
            validate_output_value(
                &format!("{field}.value"),
                value,
                ControlUnit::Unitless,
                report,
            );
        }
        ControlAction::EnterMode { mode, .. } => validate_reference(
            &format!("{field}.mode"),
            mode,
            mode_ids,
            ControlConfigValidationErrorKind::MissingMode,
            report,
        ),
        ControlAction::RaiseFault { fault_response, .. } => validate_reference(
            &format!("{field}.fault_response"),
            fault_response,
            fault_ids,
            ControlConfigValidationErrorKind::MissingFaultResponse,
            report,
        ),
        ControlAction::NoOp { .. } => {}
    }
}

fn validate_fault_response(
    index: usize,
    fault: &FaultResponse,
    action_ids: &BTreeSet<String>,
    mode_ids: &BTreeSet<String>,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("fault_responses[{index}]");
    if let Some(safe_mode) = &fault.safe_mode {
        validate_reference(
            &format!("{field}.safe_mode"),
            safe_mode,
            mode_ids,
            ControlConfigValidationErrorKind::MissingMode,
            report,
        );
    }
    for (action_index, action) in fault.actions.iter().enumerate() {
        validate_reference(
            &format!("{field}.actions[{action_index}]"),
            action,
            action_ids,
            ControlConfigValidationErrorKind::MissingAction,
            report,
        );
    }
}

fn validate_timing_rule(
    index: usize,
    timing_rule: &TimingRule,
    machine_ids: &BTreeSet<String>,
    action_ids: &BTreeSet<String>,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("timing_rules[{index}]");
    match timing_rule.scope {
        TimingRuleScope::StateMachine => validate_reference(
            &format!("{field}.target"),
            &timing_rule.target,
            machine_ids,
            ControlConfigValidationErrorKind::MissingStateMachine,
            report,
        ),
        TimingRuleScope::Action => validate_reference(
            &format!("{field}.target"),
            &timing_rule.target,
            action_ids,
            ControlConfigValidationErrorKind::MissingAction,
            report,
        ),
        TimingRuleScope::ControlLoop | TimingRuleScope::Transition => {
            validate_non_empty(&format!("{field}.target"), &timing_rule.target, report);
        }
    }
    validate_optional_non_negative_finite(
        &format!("{field}.max_latency_s"),
        timing_rule.max_latency_s,
        ControlConfigValidationErrorKind::InvalidTimingRule,
        report,
    );
    validate_optional_non_negative_finite(
        &format!("{field}.min_interval_s"),
        timing_rule.min_interval_s,
        ControlConfigValidationErrorKind::InvalidTimingRule,
        report,
    );
    validate_optional_non_negative_finite(
        &format!("{field}.timeout_s"),
        timing_rule.timeout_s,
        ControlConfigValidationErrorKind::InvalidTimingRule,
        report,
    );
}

struct ControlReferenceSets<'a> {
    input_ids: &'a BTreeSet<String>,
    threshold_ids: &'a BTreeSet<String>,
    mode_ids: &'a BTreeSet<String>,
    action_ids: &'a BTreeSet<String>,
    fault_ids: &'a BTreeSet<String>,
}

fn validate_state_machine(
    index: usize,
    machine: &StateMachine,
    references: &ControlReferenceSets<'_>,
    report: &mut ControlConfigValidationReport,
) {
    let field = format!("state_machines[{index}]");
    if machine.states.is_empty() {
        report.push(ControlConfigValidationError::new(
            format!("{field}.states"),
            ControlConfigValidationErrorKind::InvalidStateMachine,
            "state machine must define at least one state",
        ));
    }

    let state_ids = collect_ids(
        &format!("{field}.states"),
        &machine.states,
        |state| state.id.as_str(),
        report,
    );
    validate_reference(
        &format!("{field}.initial_state"),
        &machine.initial_state,
        &state_ids,
        ControlConfigValidationErrorKind::MissingState,
        report,
    );

    let mut transition_ids = BTreeSet::new();
    for (state_index, state) in machine.states.iter().enumerate() {
        let state_field = format!("{field}.states[{state_index}]");
        validate_optional_non_negative_finite(
            &format!("{state_field}.timeout_s"),
            state.timeout_s,
            ControlConfigValidationErrorKind::InvalidStateMachine,
            report,
        );
        for (action_index, action) in state.entry_actions.iter().enumerate() {
            validate_reference(
                &format!("{state_field}.entry_actions[{action_index}]"),
                action,
                references.action_ids,
                ControlConfigValidationErrorKind::MissingAction,
                report,
            );
        }
        for (action_index, action) in state.exit_actions.iter().enumerate() {
            validate_reference(
                &format!("{state_field}.exit_actions[{action_index}]"),
                action,
                references.action_ids,
                ControlConfigValidationErrorKind::MissingAction,
                report,
            );
        }
    }

    for (transition_index, transition) in machine.transitions.iter().enumerate() {
        let transition_field = format!("{field}.transitions[{transition_index}]");
        validate_non_empty(&format!("{transition_field}.id"), &transition.id, report);
        if !transition.id.is_empty() && !transition_ids.insert(transition.id.clone()) {
            report.push(ControlConfigValidationError::new(
                format!("{transition_field}.id"),
                ControlConfigValidationErrorKind::DuplicateIdentifier,
                format!("duplicate transition `{}`", transition.id),
            ));
        }
        validate_reference(
            &format!("{transition_field}.from"),
            &transition.from,
            &state_ids,
            ControlConfigValidationErrorKind::MissingState,
            report,
        );
        validate_reference(
            &format!("{transition_field}.to"),
            &transition.to,
            &state_ids,
            ControlConfigValidationErrorKind::MissingState,
            report,
        );
        validate_transition_condition(&transition_field, &transition.condition, references, report);
        for (action_index, action) in transition.actions.iter().enumerate() {
            validate_reference(
                &format!("{transition_field}.actions[{action_index}]"),
                action,
                references.action_ids,
                ControlConfigValidationErrorKind::MissingAction,
                report,
            );
        }
        if let Some(fault_response) = &transition.fault_response {
            validate_reference(
                &format!("{transition_field}.fault_response"),
                fault_response,
                references.fault_ids,
                ControlConfigValidationErrorKind::MissingFaultResponse,
                report,
            );
        }
    }
}

fn validate_transition_condition(
    field: &str,
    condition: &TransitionCondition,
    references: &ControlReferenceSets<'_>,
    report: &mut ControlConfigValidationReport,
) {
    match condition {
        TransitionCondition::InputAbove { input, threshold }
        | TransitionCondition::InputBelow { input, threshold } => {
            validate_reference(
                &format!("{field}.condition.input"),
                input,
                references.input_ids,
                ControlConfigValidationErrorKind::MissingInput,
                report,
            );
            validate_reference(
                &format!("{field}.condition.threshold"),
                threshold,
                references.threshold_ids,
                ControlConfigValidationErrorKind::MissingThreshold,
                report,
            );
        }
        TransitionCondition::InputState { input, .. } => validate_reference(
            &format!("{field}.condition.input"),
            input,
            references.input_ids,
            ControlConfigValidationErrorKind::MissingInput,
            report,
        ),
        TransitionCondition::TimerElapsed { duration_s } => {
            if !duration_s.is_finite() || *duration_s < 0.0 {
                report.push(ControlConfigValidationError::new(
                    format!("{field}.condition.duration_s"),
                    ControlConfigValidationErrorKind::InvalidTransition,
                    "duration_s must be finite and non-negative",
                ));
            }
        }
        TransitionCondition::ModeIs { mode } => validate_reference(
            &format!("{field}.condition.mode"),
            mode,
            references.mode_ids,
            ControlConfigValidationErrorKind::MissingMode,
            report,
        ),
        TransitionCondition::Always => {}
    }
}

fn validate_output_value(
    field: &str,
    value: &OutputValue,
    expected_unit: ControlUnit,
    report: &mut ControlConfigValidationReport,
) {
    match value {
        OutputValue::Analog { value } => {
            validate_unit_value(
                &format!("{field}.value"),
                *value,
                if expected_unit == ControlUnit::Unitless {
                    None
                } else {
                    Some(expected_unit)
                },
                ControlConfigValidationErrorKind::InvalidAction,
                report,
            );
        }
        OutputValue::Digital { .. } => {}
        OutputValue::PwmDuty { duty_cycle } => {
            if !duty_cycle.is_finite() || !(0.0..=1.0).contains(duty_cycle) {
                report.push(ControlConfigValidationError::new(
                    format!("{field}.duty_cycle"),
                    ControlConfigValidationErrorKind::InvalidAction,
                    "duty_cycle must be finite and between 0.0 and 1.0",
                ));
            }
        }
        OutputValue::Named { state } => {
            validate_non_empty(&format!("{field}.state"), state, report)
        }
    }
}

fn validate_unit_value(
    field: &str,
    value: UnitValue,
    expected_unit: Option<ControlUnit>,
    kind: ControlConfigValidationErrorKind,
    report: &mut ControlConfigValidationReport,
) {
    if !value.value.is_finite() {
        report.push(ControlConfigValidationError::new(
            field,
            kind,
            "value must be finite",
        ));
    }
    if let Some(expected_unit) = expected_unit {
        if value.unit != expected_unit {
            report.push(ControlConfigValidationError::new(
                format!("{field}.unit"),
                ControlConfigValidationErrorKind::InvalidUnit,
                format!(
                    "expected unit `{}`, got `{}`",
                    expected_unit.symbol(),
                    value.unit.symbol()
                ),
            ));
        }
    }
}

fn validate_optional_positive_finite(
    field: &str,
    value: Option<f64>,
    kind: ControlConfigValidationErrorKind,
    report: &mut ControlConfigValidationReport,
) {
    let Some(value) = value else {
        return;
    };
    if !value.is_finite() || value <= 0.0 {
        report.push(ControlConfigValidationError::new(
            field,
            kind,
            "value must be finite and greater than zero",
        ));
    }
}

fn validate_optional_non_negative_finite(
    field: &str,
    value: Option<f64>,
    kind: ControlConfigValidationErrorKind,
    report: &mut ControlConfigValidationReport,
) {
    let Some(value) = value else {
        return;
    };
    if !value.is_finite() || value < 0.0 {
        report.push(ControlConfigValidationError::new(
            field,
            kind,
            "value must be finite and non-negative",
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_validates_production_control_config_schema() {
        let config = example_config();

        assert_eq!(
            config.package.schema_version,
            CURRENT_CONTROL_SCHEMA_VERSION
        );
        assert_eq!(config.target.kind, ControlTargetKind::ControllerRuntime);
        assert_eq!(config.inputs.len(), 2);
        assert_eq!(config.outputs.len(), 2);
        assert_eq!(config.thresholds.len(), 2);
        assert_eq!(config.modes[0].id, "normal");
        assert_eq!(config.state_machines[0].initial_state, "idle");
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn parses_example_toml_and_json_round_trip() {
        let toml_config = parse_control_config_toml(include_str!(
            "../../../examples/control-config/production-control-config.toml"
        ))
        .expect("example TOML should parse");

        toml_config.validate().expect("example should validate");

        let json = serde_json::to_string_pretty(&toml_config).expect("config should serialize");
        assert!(json.contains("\"schema_version\": \"0.1.0\""));
        assert!(json.contains("\"controller_runtime\""));
        assert!(!json.contains("daq"));
        assert!(!json.contains("plot"));
        assert!(!json.contains("hal"));

        let json_config = parse_control_config_json(&json).expect("JSON should parse");
        assert_eq!(json_config, toml_config);
    }

    #[test]
    fn rejects_missing_references_and_invalid_values() {
        let mut config = example_config();
        config.thresholds[0].input = "missing_input".to_string();
        if let ControlAction::SetOutput { output, value, .. } = &mut config.actions[0] {
            *output = "missing_output".to_string();
            *value = OutputValue::PwmDuty { duty_cycle: 1.5 };
        }
        config.timing.control_loop_period_s = Some(0.0);
        config.state_machines[0].initial_state = "missing_state".to_string();
        config.state_machines[0].transitions[0].actions = vec!["missing_action".to_string()];

        let report = config
            .validate()
            .expect_err("invalid config should produce structured errors");

        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::MissingInput
                && error.field == "thresholds[0].input"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::MissingOutput
                && error.field == "actions[0].output"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::InvalidAction
                && error.field == "actions[0].value.duty_cycle"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::InvalidTiming
                && error.field == "timing.control_loop_period_s"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::MissingState
                && error.field == "state_machines[0].initial_state"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::MissingAction
                && error.field == "state_machines[0].transitions[0].actions[0]"
        }));
    }

    #[test]
    fn requires_approval_metadata_for_approved_configs() {
        let mut config = example_config();
        config.approval.status = ApprovalStatus::Approved;
        config.approval.approved_by = None;
        config.approval.approved_at = None;

        let report = config
            .validate()
            .expect_err("approved config without approval metadata should fail");

        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::InvalidApproval
                && error.field == "approval.approved_by"
        }));
        assert!(report.errors.iter().any(|error| {
            error.kind == ControlConfigValidationErrorKind::InvalidApproval
                && error.field == "approval.approved_at"
        }));
    }

    fn example_config() -> ProductionControlConfig {
        parse_control_config_toml(include_str!(
            "../../../examples/control-config/production-control-config.toml"
        ))
        .expect("example TOML should parse")
    }
}

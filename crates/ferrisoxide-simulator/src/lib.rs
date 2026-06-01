//! Deterministic virtual controller simulation over production control configs.
//!
//! The simulator consumes `ferrisoxide-control-schema` data structures and
//! caller-provided sample frames. It does not parse CSV, acquire DAQ data,
//! command hardware, bind HALs, integrate RTOS SDKs, render plots, or claim
//! real-time behavior.

use std::collections::BTreeMap;
use std::fmt;

use ferrisoxide_control_schema::{
    ControlAction, ControlConfigValidationReport, ControlMode, ControlThreshold, DigitalState,
    FaultResponse, OutputValue, ProductionControlConfig, StateDefinition, StateMachine,
    StateTransition, TransitionCondition,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationInputFrame {
    pub time_s: f64,
    pub inputs: BTreeMap<String, SimulatedInputValue>,
}

impl SimulationInputFrame {
    pub fn new(time_s: f64) -> Self {
        Self {
            time_s,
            inputs: BTreeMap::new(),
        }
    }

    pub fn with_input(mut self, input: impl Into<String>, value: SimulatedInputValue) -> Self {
        self.inputs.insert(input.into(), value);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SimulatedInputValue {
    Analog { value: f64 },
    Digital { state: DigitalState },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationReport {
    pub package_name: String,
    pub package_version: String,
    pub initial_mode: String,
    pub frames: Vec<ControlStateTrace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ControlStateTrace {
    pub sample_index: usize,
    pub time_s: f64,
    pub mode: String,
    pub machines: Vec<StateMachineTrace>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transitions: Vec<TransitionTrace>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionTrace>,
    pub outputs: Vec<OutputTrace>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub faults: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateMachineTrace {
    pub machine: String,
    pub state: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransitionTrace {
    pub machine: String,
    pub transition: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionTrace {
    pub action: String,
    pub kind: ActionTraceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionTraceKind {
    SetOutput,
    EnterMode,
    RaiseFault,
    NoOp,
    FaultResponse,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputTrace {
    pub output: String,
    pub value: OutputValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimulationError {
    InvalidConfig(ControlConfigValidationReport),
    EmptyInput,
    NonMonotonicTime {
        previous_time_s: f64,
        current_time_s: f64,
    },
    UnknownMode(String),
    UnknownStateMachine(String),
    UnknownState {
        machine: String,
        state: String,
    },
    MissingInput {
        input: String,
        sample_index: usize,
    },
    MissingThreshold(String),
    MissingAction(String),
    MissingOutput(String),
    MissingFaultResponse(String),
    InputTypeMismatch {
        input: String,
        expected: &'static str,
    },
    InvalidInputValue {
        input: String,
    },
    InvalidTimerCondition {
        transition: String,
    },
}

impl fmt::Display for SimulationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfig(report) => write!(formatter, "invalid control config: {report}"),
            Self::EmptyInput => write!(formatter, "simulation requires at least one input frame"),
            Self::NonMonotonicTime {
                previous_time_s,
                current_time_s,
            } => write!(
                formatter,
                "simulation time must increase monotonically, previous={previous_time_s}, current={current_time_s}"
            ),
            Self::UnknownMode(mode) => write!(formatter, "unknown mode `{mode}`"),
            Self::UnknownStateMachine(machine) => {
                write!(formatter, "unknown state machine `{machine}`")
            }
            Self::UnknownState { machine, state } => {
                write!(formatter, "unknown state `{state}` in state machine `{machine}`")
            }
            Self::MissingInput {
                input,
                sample_index,
            } => write!(
                formatter,
                "sample {sample_index} is missing required control input `{input}`"
            ),
            Self::MissingThreshold(threshold) => write!(formatter, "missing threshold `{threshold}`"),
            Self::MissingAction(action) => write!(formatter, "missing action `{action}`"),
            Self::MissingOutput(output) => write!(formatter, "missing output `{output}`"),
            Self::MissingFaultResponse(fault) => {
                write!(formatter, "missing fault response `{fault}`")
            }
            Self::InputTypeMismatch { input, expected } => {
                write!(formatter, "input `{input}` does not provide expected {expected} value")
            }
            Self::InvalidInputValue { input } => {
                write!(formatter, "input `{input}` contains a non-finite value")
            }
            Self::InvalidTimerCondition { transition } => {
                write!(formatter, "transition `{transition}` has invalid timer condition")
            }
        }
    }
}

impl std::error::Error for SimulationError {}

pub fn simulate_controller(
    config: &ProductionControlConfig,
    mode_id: &str,
    frames: &[SimulationInputFrame],
) -> Result<SimulationReport, SimulationError> {
    config.validate().map_err(SimulationError::InvalidConfig)?;

    if frames.is_empty() {
        return Err(SimulationError::EmptyInput);
    }

    let mode = find_mode(config, mode_id)?;
    let mut runtime = RuntimeState::new(config, mode, frames[0].time_s)?;
    let mut traces = Vec::with_capacity(frames.len());

    for (sample_index, frame) in frames.iter().enumerate() {
        if sample_index > 0 {
            let previous_time_s = frames[sample_index - 1].time_s;
            if frame.time_s <= previous_time_s {
                return Err(SimulationError::NonMonotonicTime {
                    previous_time_s,
                    current_time_s: frame.time_s,
                });
            }
        }

        validate_frame_inputs(config, sample_index, frame)?;
        let mut trace = ControlStateTrace {
            sample_index,
            time_s: frame.time_s,
            mode: runtime.current_mode.clone(),
            machines: Vec::new(),
            transitions: Vec::new(),
            actions: Vec::new(),
            outputs: Vec::new(),
            faults: Vec::new(),
        };

        if sample_index == 0 {
            runtime.apply_initial_actions(config, &mut trace)?;
        }

        runtime.advance(config, frame, &mut trace)?;
        trace.mode = runtime.current_mode.clone();
        trace.machines = runtime.machine_traces();
        trace.outputs = runtime.output_traces();
        traces.push(trace);
    }

    Ok(SimulationReport {
        package_name: config.package.name.clone(),
        package_version: config.package.version.clone(),
        initial_mode: mode_id.to_string(),
        frames: traces,
    })
}

struct RuntimeState {
    current_mode: String,
    machines: BTreeMap<String, MachineRuntimeState>,
    outputs: BTreeMap<String, OutputValue>,
}

impl RuntimeState {
    fn new(
        config: &ProductionControlConfig,
        mode: &ControlMode,
        initial_time_s: f64,
    ) -> Result<Self, SimulationError> {
        let mut machines = BTreeMap::new();
        for machine_id in enabled_state_machines(mode) {
            let machine = find_state_machine(config, machine_id)?;
            machines.insert(
                machine.id.clone(),
                MachineRuntimeState {
                    state: machine.initial_state.clone(),
                    entered_time_s: initial_time_s,
                },
            );
        }

        let outputs = config
            .outputs
            .iter()
            .map(|output| (output.id.clone(), output.safe_state.clone()))
            .collect();

        Ok(Self {
            current_mode: mode.id.clone(),
            machines,
            outputs,
        })
    }

    fn apply_initial_actions(
        &mut self,
        config: &ProductionControlConfig,
        trace: &mut ControlStateTrace,
    ) -> Result<(), SimulationError> {
        let mode = find_mode(config, &self.current_mode)?;
        for action_id in &mode.entry_actions {
            self.apply_action(config, action_id, trace, 0)?;
        }

        let machine_ids: Vec<String> = self.machines.keys().cloned().collect();
        for machine_id in machine_ids {
            let state_id = self
                .machines
                .get(&machine_id)
                .expect("machine id collected from map")
                .state
                .clone();
            let machine = find_state_machine(config, &machine_id)?;
            let state = find_state(machine, &state_id)?;
            for action_id in &state.entry_actions {
                self.apply_action(config, action_id, trace, 0)?;
            }
        }

        Ok(())
    }

    fn advance(
        &mut self,
        config: &ProductionControlConfig,
        frame: &SimulationInputFrame,
        trace: &mut ControlStateTrace,
    ) -> Result<(), SimulationError> {
        let machine_ids: Vec<String> = self.machines.keys().cloned().collect();
        for machine_id in machine_ids {
            let machine = find_state_machine(config, &machine_id)?;
            let current = self
                .machines
                .get(&machine_id)
                .expect("machine id collected from map")
                .clone();
            let current_state = find_state(machine, &current.state)?;

            if let Some(transition) =
                first_matching_transition(config, machine, &current, frame, &self.current_mode)?
            {
                for action_id in &current_state.exit_actions {
                    self.apply_action(config, action_id, trace, 0)?;
                }
                for action_id in &transition.actions {
                    self.apply_action(config, action_id, trace, 0)?;
                }
                if let Some(fault_response) = &transition.fault_response {
                    self.apply_fault_response(config, fault_response, trace, 0)?;
                }

                self.machines.insert(
                    machine_id.clone(),
                    MachineRuntimeState {
                        state: transition.to.clone(),
                        entered_time_s: frame.time_s,
                    },
                );

                trace.transitions.push(TransitionTrace {
                    machine: machine_id,
                    transition: transition.id.clone(),
                    from: transition.from.clone(),
                    to: transition.to.clone(),
                });

                let next_state = find_state(machine, &transition.to)?;
                for action_id in &next_state.entry_actions {
                    self.apply_action(config, action_id, trace, 0)?;
                }
            }
        }

        Ok(())
    }

    fn apply_action(
        &mut self,
        config: &ProductionControlConfig,
        action_id: &str,
        trace: &mut ControlStateTrace,
        depth: usize,
    ) -> Result<(), SimulationError> {
        let action = find_action(config, action_id)?;
        match action {
            ControlAction::SetOutput { id, output, value } => {
                if !self.outputs.contains_key(output) {
                    return Err(SimulationError::MissingOutput(output.clone()));
                }
                self.outputs.insert(output.clone(), value.clone());
                trace.actions.push(ActionTrace {
                    action: id.clone(),
                    kind: ActionTraceKind::SetOutput,
                });
            }
            ControlAction::EnterMode { id, mode } => {
                find_mode(config, mode)?;
                self.current_mode = mode.clone();
                trace.actions.push(ActionTrace {
                    action: id.clone(),
                    kind: ActionTraceKind::EnterMode,
                });
            }
            ControlAction::RaiseFault { id, fault_response } => {
                trace.actions.push(ActionTrace {
                    action: id.clone(),
                    kind: ActionTraceKind::RaiseFault,
                });
                self.apply_fault_response(config, fault_response, trace, depth + 1)?;
            }
            ControlAction::NoOp { id } => {
                trace.actions.push(ActionTrace {
                    action: id.clone(),
                    kind: ActionTraceKind::NoOp,
                });
            }
        }

        Ok(())
    }

    fn apply_fault_response(
        &mut self,
        config: &ProductionControlConfig,
        fault_response_id: &str,
        trace: &mut ControlStateTrace,
        depth: usize,
    ) -> Result<(), SimulationError> {
        if depth > 4 {
            return Ok(());
        }
        if trace.faults.iter().any(|fault| fault == fault_response_id) {
            return Ok(());
        }

        let fault = find_fault_response(config, fault_response_id)?;
        trace.faults.push(fault.id.clone());
        trace.actions.push(ActionTrace {
            action: fault.id.clone(),
            kind: ActionTraceKind::FaultResponse,
        });
        if let Some(safe_mode) = &fault.safe_mode {
            find_mode(config, safe_mode)?;
            self.current_mode = safe_mode.clone();
        }
        for action_id in &fault.actions {
            self.apply_action(config, action_id, trace, depth + 1)?;
        }

        Ok(())
    }

    fn machine_traces(&self) -> Vec<StateMachineTrace> {
        self.machines
            .iter()
            .map(|(machine, state)| StateMachineTrace {
                machine: machine.clone(),
                state: state.state.clone(),
            })
            .collect()
    }

    fn output_traces(&self) -> Vec<OutputTrace> {
        self.outputs
            .iter()
            .map(|(output, value)| OutputTrace {
                output: output.clone(),
                value: value.clone(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct MachineRuntimeState {
    state: String,
    entered_time_s: f64,
}

fn validate_frame_inputs(
    config: &ProductionControlConfig,
    sample_index: usize,
    frame: &SimulationInputFrame,
) -> Result<(), SimulationError> {
    for input in &config.inputs {
        let Some(value) = frame.inputs.get(&input.id) else {
            return Err(SimulationError::MissingInput {
                input: input.id.clone(),
                sample_index,
            });
        };
        if let SimulatedInputValue::Analog { value } = value {
            if !value.is_finite() {
                return Err(SimulationError::InvalidInputValue {
                    input: input.id.clone(),
                });
            }
        }
    }
    Ok(())
}

fn first_matching_transition<'a>(
    config: &'a ProductionControlConfig,
    machine: &'a StateMachine,
    current: &MachineRuntimeState,
    frame: &SimulationInputFrame,
    current_mode: &str,
) -> Result<Option<&'a StateTransition>, SimulationError> {
    for transition in machine
        .transitions
        .iter()
        .filter(|transition| transition.from == current.state)
    {
        if condition_matches(config, transition, current, frame, current_mode)? {
            return Ok(Some(transition));
        }
    }
    Ok(None)
}

fn condition_matches(
    config: &ProductionControlConfig,
    transition: &StateTransition,
    current: &MachineRuntimeState,
    frame: &SimulationInputFrame,
    current_mode: &str,
) -> Result<bool, SimulationError> {
    match &transition.condition {
        TransitionCondition::InputAbove { input, threshold } => {
            let value = analog_input(frame, input)?;
            let threshold = find_threshold(config, threshold)?;
            Ok(value > threshold.value.value)
        }
        TransitionCondition::InputBelow { input, threshold } => {
            let value = analog_input(frame, input)?;
            let threshold = find_threshold(config, threshold)?;
            Ok(value < threshold.value.value)
        }
        TransitionCondition::InputState { input, state } => {
            let value = digital_input(frame, input)?;
            Ok(value == *state)
        }
        TransitionCondition::TimerElapsed { duration_s } => {
            if !duration_s.is_finite() || *duration_s < 0.0 {
                return Err(SimulationError::InvalidTimerCondition {
                    transition: transition.id.clone(),
                });
            }
            Ok(frame.time_s - current.entered_time_s >= *duration_s)
        }
        TransitionCondition::ModeIs { mode } => Ok(current_mode == mode),
        TransitionCondition::Always => Ok(true),
    }
}

fn analog_input(frame: &SimulationInputFrame, input: &str) -> Result<f64, SimulationError> {
    match frame.inputs.get(input) {
        Some(SimulatedInputValue::Analog { value }) => Ok(*value),
        Some(SimulatedInputValue::Digital { .. }) => Err(SimulationError::InputTypeMismatch {
            input: input.to_string(),
            expected: "analog",
        }),
        None => Err(SimulationError::MissingInput {
            input: input.to_string(),
            sample_index: 0,
        }),
    }
}

fn digital_input(
    frame: &SimulationInputFrame,
    input: &str,
) -> Result<DigitalState, SimulationError> {
    match frame.inputs.get(input) {
        Some(SimulatedInputValue::Digital { state }) => Ok(*state),
        Some(SimulatedInputValue::Analog { .. }) => Err(SimulationError::InputTypeMismatch {
            input: input.to_string(),
            expected: "digital",
        }),
        None => Err(SimulationError::MissingInput {
            input: input.to_string(),
            sample_index: 0,
        }),
    }
}

fn find_mode<'a>(
    config: &'a ProductionControlConfig,
    mode_id: &str,
) -> Result<&'a ControlMode, SimulationError> {
    config
        .modes
        .iter()
        .find(|mode| mode.id == mode_id)
        .ok_or_else(|| SimulationError::UnknownMode(mode_id.to_string()))
}

fn find_state_machine<'a>(
    config: &'a ProductionControlConfig,
    machine_id: &str,
) -> Result<&'a StateMachine, SimulationError> {
    config
        .state_machines
        .iter()
        .find(|machine| machine.id == machine_id)
        .ok_or_else(|| SimulationError::UnknownStateMachine(machine_id.to_string()))
}

fn find_state<'a>(
    machine: &'a StateMachine,
    state_id: &str,
) -> Result<&'a StateDefinition, SimulationError> {
    machine
        .states
        .iter()
        .find(|state| state.id == state_id)
        .ok_or_else(|| SimulationError::UnknownState {
            machine: machine.id.clone(),
            state: state_id.to_string(),
        })
}

fn find_threshold<'a>(
    config: &'a ProductionControlConfig,
    threshold_id: &str,
) -> Result<&'a ControlThreshold, SimulationError> {
    config
        .thresholds
        .iter()
        .find(|threshold| threshold.id == threshold_id)
        .ok_or_else(|| SimulationError::MissingThreshold(threshold_id.to_string()))
}

fn find_action<'a>(
    config: &'a ProductionControlConfig,
    action_id: &str,
) -> Result<&'a ControlAction, SimulationError> {
    config
        .actions
        .iter()
        .find(|action| action_id_of(action) == action_id)
        .ok_or_else(|| SimulationError::MissingAction(action_id.to_string()))
}

fn find_fault_response<'a>(
    config: &'a ProductionControlConfig,
    fault_response_id: &str,
) -> Result<&'a FaultResponse, SimulationError> {
    config
        .fault_responses
        .iter()
        .find(|fault| fault.id == fault_response_id)
        .ok_or_else(|| SimulationError::MissingFaultResponse(fault_response_id.to_string()))
}

fn action_id_of(action: &ControlAction) -> &str {
    match action {
        ControlAction::SetOutput { id, .. }
        | ControlAction::EnterMode { id, .. }
        | ControlAction::RaiseFault { id, .. }
        | ControlAction::NoOp { id } => id,
    }
}

fn enabled_state_machines(mode: &ControlMode) -> Vec<&str> {
    if mode.enabled_state_machines.is_empty() {
        vec![mode.initial_state_machine.as_str()]
    } else {
        mode.enabled_state_machines
            .iter()
            .map(String::as_str)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ferrisoxide_control_schema::parse_control_config_toml;

    #[test]
    fn simulates_heated_actuator_state_trace_from_abstract_samples() {
        let config = example_config();
        let frames = vec![
            frame(0.000, 0.0, 0.0),
            frame(1.000, 5.0, 0.0),
            frame(1.020, 5.0, 5.0),
        ];

        let report = simulate_controller(&config, "normal", &frames).expect("simulation passes");

        assert_eq!(report.package_name, "heated-actuator-production-control");
        assert_eq!(state_at(&report, 0), "idle");
        assert_eq!(state_at(&report, 1), "heating");
        assert_eq!(state_at(&report, 2), "idle");
        assert_eq!(
            report.frames[1].transitions[0].transition,
            "command_to_heating"
        );
        assert_eq!(
            report.frames[2].transitions[0].transition,
            "feedback_reached"
        );
        assert!(report.frames[1]
            .actions
            .iter()
            .any(|action| action.action == "heater_on"));
    }

    #[test]
    fn records_timeout_fault_without_controller_hardware() {
        let config = example_config();
        let frames = vec![
            frame(0.000, 0.0, 0.0),
            frame(1.000, 5.0, 0.0),
            frame(1.052, 5.0, 0.0),
        ];

        let report = simulate_controller(&config, "normal", &frames).expect("simulation passes");
        let timeout_trace = &report.frames[2];

        assert_eq!(state_at(&report, 2), "faulted");
        assert_eq!(timeout_trace.mode, "safe");
        assert_eq!(timeout_trace.transitions[0].transition, "response_timeout");
        assert!(timeout_trace
            .faults
            .iter()
            .any(|fault| fault == "actuator_response_fault"));
        assert_output_digital(timeout_trace, "heater_enable", DigitalState::Low);
        assert_output_pwm(timeout_trace, "actuator_pwm", 0.0);
    }

    #[test]
    fn rejects_missing_inputs_and_non_monotonic_time() {
        let config = example_config();
        let missing = vec![SimulationInputFrame::new(0.0)
            .with_input("command", SimulatedInputValue::Analog { value: 0.0 })];

        let error = simulate_controller(&config, "normal", &missing).expect_err("missing feedback");
        assert!(matches!(error, SimulationError::MissingInput { .. }));

        let non_monotonic = vec![frame(0.0, 0.0, 0.0), frame(0.0, 5.0, 0.0)];
        let error =
            simulate_controller(&config, "normal", &non_monotonic).expect_err("bad time axis");
        assert!(matches!(error, SimulationError::NonMonotonicTime { .. }));
    }

    fn example_config() -> ProductionControlConfig {
        let input = include_str!("../../../examples/control-config/production-control-config.toml");
        parse_control_config_toml(input).expect("example config parses")
    }

    fn frame(time_s: f64, command_v: f64, feedback_v: f64) -> SimulationInputFrame {
        SimulationInputFrame::new(time_s)
            .with_input("command", SimulatedInputValue::Analog { value: command_v })
            .with_input(
                "feedback",
                SimulatedInputValue::Analog { value: feedback_v },
            )
    }

    fn state_at(report: &SimulationReport, index: usize) -> &str {
        report.frames[index].machines[0].state.as_str()
    }

    fn assert_output_digital(trace: &ControlStateTrace, output_id: &str, expected: DigitalState) {
        let output = trace
            .outputs
            .iter()
            .find(|output| output.output == output_id)
            .expect("output trace exists");
        assert_eq!(output.value, OutputValue::Digital { state: expected });
    }

    fn assert_output_pwm(trace: &ControlStateTrace, output_id: &str, expected: f64) {
        let output = trace
            .outputs
            .iter()
            .find(|output| output.output == output_id)
            .expect("output trace exists");
        assert_eq!(
            output.value,
            OutputValue::PwmDuty {
                duty_cycle: expected
            }
        );
    }
}

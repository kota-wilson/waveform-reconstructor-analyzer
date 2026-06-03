use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use ferrisoxide_gui::{
    GuiChannelConfigDraft, GuiConfigCriterionDraft, GuiConfigCriterionKind, GuiConfigDirection,
    GuiConfigEventKind, GuiConfigFilterDraft, GuiConfigFilterKind, GuiConfigNormalizeMode,
    GuiConfigState, GuiSession, GuiSourceMode, GuiTab, WorkflowStatus, CHANNEL_UNIT_OPTIONS,
    CONFIG_CRITERION_OPTIONS, CONFIG_DIRECTION_OPTIONS, CONFIG_EVENT_KIND_OPTIONS,
    CONFIG_FILTER_OPTIONS, CONFIG_NORMALIZE_MODE_OPTIONS, CONFIG_STATE_OPTIONS,
    PLOT_RESOLUTION_OPTIONS, TIME_UNIT_OPTIONS,
};
use std::path::{Path, PathBuf};

pub fn run() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "FerrisOxide Workflow",
        options,
        Box::new(|_cc| Ok(Box::new(FerrisOxideGuiApp::default()))),
    )
}

#[derive(Default)]
struct FerrisOxideGuiApp {
    session: GuiSession,
}

impl eframe::App for FerrisOxideGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("workflow_nav")
            .resizable(false)
            .default_width(156.0)
            .show(ctx, |ui| {
                ui.heading("FerrisOxide");
                ui.separator();
                nav_button(ui, &mut self.session.active_tab, GuiTab::Source, "Source");
                nav_button(ui, &mut self.session.active_tab, GuiTab::Config, "Config");
                nav_button(ui, &mut self.session.active_tab, GuiTab::Run, "Run");
                nav_button(ui, &mut self.session.active_tab, GuiTab::Results, "Results");
                nav_button(ui, &mut self.session.active_tab, GuiTab::Plot, "Plot");
                ui.separator();
                match &self.session.status {
                    WorkflowStatus::Idle => {
                        ui.label("Idle");
                    }
                    WorkflowStatus::Succeeded(message) => {
                        ui.colored_label(egui::Color32::from_rgb(18, 112, 58), message);
                    }
                    WorkflowStatus::Failed(message) => {
                        ui.colored_label(egui::Color32::from_rgb(174, 42, 42), message);
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| match self.session.active_tab {
            GuiTab::Source => self.source_panel(ui),
            GuiTab::Config => self.config_panel(ui),
            GuiTab::Run => self.run_panel(ui),
            GuiTab::Results => self.results_panel(ui),
            GuiTab::Plot => self.plot_panel(ui),
        });
    }
}

impl FerrisOxideGuiApp {
    fn source_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Source");
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.session.source_mode, GuiSourceMode::Csv, "CSV");
            ui.selectable_value(
                &mut self.session.source_mode,
                GuiSourceMode::Simulation,
                "Simulation",
            );
        });
        match self.session.source_mode {
            GuiSourceMode::Csv => self.csv_source_controls(ui),
            GuiSourceMode::Simulation => self.simulation_source_controls(ui),
        }
        if ui.button("Inspect").clicked() {
            self.session.inspect_current_source();
        }
        ui.separator();
        text_area(ui, "Inspection", &mut self.session.inspection_text, 18);
    }

    fn csv_source_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("CSV File");
            let button_width = (ui.available_width() - 128.0).max(220.0);
            let file_label = file_selector_label(&self.session.input_path);
            let response = ui
                .add_sized([button_width, 24.0], egui::Button::new(file_label))
                .on_hover_text(file_selector_hover(&self.session.input_path));
            if response.clicked() {
                pick_csv_file(&mut self.session);
            }
            if ui.button("Load Channels").clicked() {
                self.session.load_channels_from_csv();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Time Column");
            let headers = self.session.csv_headers.clone();
            ui.add_enabled_ui(!headers.is_empty(), |ui| {
                let mut selected_time_column = self.session.time_column.clone();
                egui::ComboBox::from_id_source("source_time_column")
                    .selected_text(if selected_time_column.is_empty() {
                        "Load channels first"
                    } else {
                        selected_time_column.as_str()
                    })
                    .show_ui(ui, |ui| {
                        for header in &headers {
                            ui.selectable_value(
                                &mut selected_time_column,
                                header.clone(),
                                header.as_str(),
                            );
                        }
                    });
                if selected_time_column != self.session.time_column {
                    self.session.set_time_column(selected_time_column);
                }
            });
        });

        unit_combo(
            ui,
            "Time Unit",
            "source_time_unit",
            &mut self.session.time_unit,
        );

        ui.label("Channels");
        if self.session.channel_selections.is_empty() {
            ui.label("Load channels from the selected CSV.");
        } else {
            egui::ScrollArea::vertical()
                .max_height(180.0)
                .show(ui, |ui| {
                    egui::Grid::new("source_channel_units")
                        .num_columns(3)
                        .striped(true)
                        .spacing([12.0, 6.0])
                        .show(ui, |ui| {
                            ui.strong("Use");
                            ui.strong("Channel");
                            ui.strong("Unit");
                            ui.end_row();
                            for selection in &mut self.session.channel_selections {
                                ui.checkbox(&mut selection.enabled, "");
                                ui.label(selection.header.as_str());
                                egui::ComboBox::from_id_source(format!(
                                    "source_channel_unit_{}",
                                    selection.header
                                ))
                                .selected_text(selection.unit.as_str())
                                .show_ui(ui, |ui| {
                                    for unit in CHANNEL_UNIT_OPTIONS {
                                        ui.selectable_value(
                                            &mut selection.unit,
                                            (*unit).to_string(),
                                            *unit,
                                        );
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });
            self.session.refresh_channel_selection_summary();
        }
    }

    fn simulation_source_controls(&mut self, ui: &mut egui::Ui) {
        labeled_text(ui, "Input", &mut self.session.input_path);
        labeled_text(ui, "Time Column", &mut self.session.time_column);
        labeled_text(ui, "Channels", &mut self.session.channel_csv);
        unit_combo(
            ui,
            "Time Unit",
            "simulation_time_unit",
            &mut self.session.time_unit,
        );
        labeled_text(ui, "Signal Unit", &mut self.session.signal_unit);
        labeled_text(ui, "Channel Map", &mut self.session.channel_map_path);
    }

    fn config_panel(&mut self, ui: &mut egui::Ui) {
        self.session.refresh_config_channel_choices();
        ui.heading("Config");
        ui.horizontal(|ui| {
            ui.label("Config File");
            ui.monospace(config_file_label(&self.session.config_path))
                .on_hover_text(config_file_hover(&self.session.config_path));
            if ui.button("Open TOML").clicked() {
                pick_config_open_file(&mut self.session);
            }
            if ui.button("Save As").clicked() && pick_config_save_file(&mut self.session) {
                self.session.save_config_text();
            }
        });
        ui.horizontal(|ui| {
            if ui.button("Load From Source").clicked() {
                self.session.load_config_builder_from_source();
            }
            if ui.button("Generate").clicked() {
                self.session.generate_config_text_from_builder();
            }
            if ui.button("Save").clicked() {
                if self.session.config_path.trim().is_empty() {
                    if pick_config_save_file(&mut self.session) {
                        self.session.save_config_text();
                    }
                } else {
                    self.session.save_config_text();
                }
            }
        });
        ui.separator();

        if self.session.config_draft.channels.is_empty() {
            ui.label("Load channels on Source first.");
        } else {
            let selected_index = self.session.selected_config_channel_index();
            let channel_names = self
                .session
                .config_draft
                .channels
                .iter()
                .map(|channel| channel.channel.clone())
                .collect::<Vec<_>>();
            let mut selected_channel = None;
            let mut changed = false;

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_width(220.0);
                    ui.strong("Channels");
                    egui::ScrollArea::vertical()
                        .max_height(360.0)
                        .show(ui, |ui| {
                            for (index, channel) in
                                self.session.config_draft.channels.iter().enumerate()
                            {
                                let selected = Some(index) == selected_index;
                                let label = format!("{} ({})", channel.channel, channel.unit);
                                if ui.selectable_label(selected, label).clicked() {
                                    selected_channel = Some(channel.channel.clone());
                                }
                            }
                        });
                });
                ui.separator();
                ui.vertical(|ui| {
                    if let Some(index) = selected_index {
                        let channel_config = &mut self.session.config_draft.channels[index];
                        changed |= config_channel_builder_ui(ui, channel_config, &channel_names);
                    }
                });
            });

            if let Some(channel) = selected_channel {
                self.session.select_config_channel(channel);
            }
            if changed {
                self.session.refresh_config_text_from_builder();
            }
        }

        ui.separator();
        read_only_text_area(ui, "Config TOML", &self.session.config_text, 16);
    }

    fn run_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Run");
        ui.horizontal(|ui| {
            ui.label("Output Dir");
            let button_width = (ui.available_width() - 16.0).max(220.0);
            let response = ui
                .add_sized(
                    [button_width, 24.0],
                    egui::Button::new(output_directory_label(&self.session.output_dir)),
                )
                .on_hover_text(output_directory_hover(&self.session.output_dir));
            if response.clicked() {
                pick_output_directory(&mut self.session);
            }
        });
        if self.session.source_mode == GuiSourceMode::Simulation {
            labeled_text(ui, "Control Config", &mut self.session.control_config_path);
            labeled_text(
                ui,
                "Verification Config",
                &mut self.session.verification_config_path,
            );
            labeled_text(ui, "Channel Map", &mut self.session.channel_map_path);
            labeled_text(ui, "Mode", &mut self.session.simulation_mode);
        }
        ui.checkbox(&mut self.session.overwrite_outputs, "Overwrite");
        ui.checkbox(&mut self.session.include_plot_artifact, "SVG Plot Artifact");
        ui.horizontal(|ui| {
            if ui.button("Analyze").clicked() {
                self.session.analyze_current_config();
            }
            if ui.button("Evaluate Bundle").clicked() {
                self.session.evaluate_current_bundle();
            }
        });
    }

    fn results_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Results");
        if let Some(bundle) = &self.session.bundle {
            ui.label(format!("Outcome: {}", bundle.overall_outcome));
            ui.label(format!("Output: {}", bundle.output_dir));
            ui.separator();
            for artifact in &bundle.artifacts {
                ui.label(artifact);
            }
        }
        ui.separator();
        text_area(ui, "Report", &mut self.session.report_preview, 24);
    }

    fn plot_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("Plot");
        self.session.refresh_plot_channel_choices();
        ui.horizontal_wrapped(|ui| {
            if ui.button("Load Series").clicked() {
                self.session.load_plot_series();
            }
            ui.label("Resolution");
            let mut selected_resolution = self.session.plot_resolution;
            egui::ComboBox::from_id_source("plot_resolution")
                .selected_text(selected_resolution.label())
                .show_ui(ui, |ui| {
                    for resolution in PLOT_RESOLUTION_OPTIONS {
                        ui.selectable_value(
                            &mut selected_resolution,
                            *resolution,
                            resolution.label(),
                        );
                    }
                });
            self.session.set_plot_resolution(selected_resolution);
            if self.session.plot_channel_selections.is_empty() {
                ui.label("Load channels on Source first.");
            } else {
                for selection in &mut self.session.plot_channel_selections {
                    ui.checkbox(&mut selection.enabled, selection.channel.as_str());
                }
            }
        });
        if !self.session.plot_render_summary.is_empty() {
            ui.label(self.session.plot_render_summary.as_str());
        }
        Plot::new("workflow_plot")
            .height(ui.available_height().max(240.0))
            .show(ui, |plot_ui| {
                let bounds = plot_ui.plot_bounds();
                let x_range = if plot_ui.auto_bounds().x {
                    self.session.plot_x_range()
                } else {
                    let min = bounds.min();
                    let max = bounds.max();
                    Some((min[0], max[0]))
                };
                if let Some((x_min, x_max)) = x_range {
                    let plot_width_px = plot_ui.transform().frame().width() as f64;
                    for series in
                        self.session
                            .rendered_plot_series_for_viewport(x_min, x_max, plot_width_px)
                    {
                        let points = PlotPoints::from(series.points);
                        plot_ui.line(
                            Line::new(points).name(format!("{} ({})", series.name, series.unit)),
                        );
                    }
                }
            });
    }
}

fn config_channel_builder_ui(
    ui: &mut egui::Ui,
    channel_config: &mut GuiChannelConfigDraft,
    channel_names: &[String],
) -> bool {
    let mut changed = false;
    ui.heading(format!(
        "{} ({})",
        channel_config.channel, channel_config.unit
    ));
    ui.horizontal(|ui| {
        if ui.button("Add Filter").clicked() {
            channel_config.filters.push(GuiConfigFilterDraft::default());
            changed = true;
        }
        if ui.button("Add Criterion").clicked() {
            let criterion = GuiConfigCriterionDraft {
                source_channel: default_source_channel(&channel_config.channel, channel_names),
                ..GuiConfigCriterionDraft::default()
            };
            channel_config.criteria.push(criterion);
            changed = true;
        }
    });

    ui.separator();
    ui.strong("Filters");
    if channel_config.filters.is_empty() {
        ui.label("None");
    }
    let mut filter_to_remove = None;
    for (index, filter) in channel_config.filters.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            changed |= filter_kind_combo(
                ui,
                format!("config_filter_kind_{}_{}", channel_config.channel, index),
                &mut filter.kind,
            );
            if ui.button("Remove").clicked() {
                filter_to_remove = Some(index);
            }
        });
        changed |= filter_parameter_controls(
            ui,
            format!("config_filter_params_{}_{}", channel_config.channel, index),
            filter,
        );
    }
    if let Some(index) = filter_to_remove {
        channel_config.filters.remove(index);
        changed = true;
    }

    ui.separator();
    ui.strong("Criteria");
    if channel_config.criteria.is_empty() {
        ui.label("None");
    }
    let mut criterion_to_remove = None;
    for (index, criterion) in channel_config.criteria.iter_mut().enumerate() {
        if criterion.source_channel.trim().is_empty() {
            criterion.source_channel =
                default_source_channel(&channel_config.channel, channel_names);
            changed = true;
        }
        ui.horizontal(|ui| {
            changed |= criterion_kind_combo(
                ui,
                format!("config_criterion_kind_{}_{}", channel_config.channel, index),
                &mut criterion.kind,
            );
            if ui.button("Remove").clicked() {
                criterion_to_remove = Some(index);
            }
        });
        changed |= criterion_parameter_controls(
            ui,
            format!(
                "config_criterion_params_{}_{}",
                channel_config.channel, index
            ),
            &channel_config.channel,
            channel_names,
            criterion,
        );
    }
    if let Some(index) = criterion_to_remove {
        channel_config.criteria.remove(index);
        changed = true;
    }

    changed
}

fn filter_kind_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigFilterKind,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_FILTER_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn criterion_kind_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigCriterionKind,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_CRITERION_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn filter_parameter_controls(
    ui: &mut egui::Ui,
    id_prefix: String,
    filter: &mut GuiConfigFilterDraft,
) -> bool {
    let mut changed = false;
    ui.horizontal_wrapped(|ui| match filter.kind {
        GuiConfigFilterKind::MovingAverage | GuiConfigFilterKind::MovingMedian => {
            changed |= usize_drag(ui, "Window", &mut filter.window_samples, 1, 1_000_000);
        }
        GuiConfigFilterKind::LowPass
        | GuiConfigFilterKind::HighPass
        | GuiConfigFilterKind::HighPassBaseline => {
            changed |= f64_drag(ui, "Cutoff Hz", &mut filter.cutoff_hz, 0.001, 1_000_000.0);
        }
        GuiConfigFilterKind::Offset => {
            changed |= signed_f64_drag(ui, "Offset V", &mut filter.offset_v);
        }
        GuiConfigFilterKind::Gain => {
            changed |= signed_f64_drag(ui, "Gain", &mut filter.gain);
        }
        GuiConfigFilterKind::Clamp => {
            changed |= signed_f64_drag(ui, "Min V", &mut filter.min_v);
            changed |= signed_f64_drag(ui, "Max V", &mut filter.max_v);
        }
        GuiConfigFilterKind::Deadband => {
            changed |= signed_f64_drag(ui, "Threshold V", &mut filter.threshold_v);
        }
        GuiConfigFilterKind::BaselineSubtract => {
            changed |= signed_f64_drag(ui, "Baseline V", &mut filter.baseline_v);
        }
        GuiConfigFilterKind::Log | GuiConfigFilterKind::Exp => {
            changed |= f64_drag(ui, "Base", &mut filter.base, 0.001, 1_000_000.0);
        }
        GuiConfigFilterKind::Normalize => {
            changed |= normalize_mode_combo(
                ui,
                format!("{id_prefix}_normalize_mode"),
                &mut filter.normalize_mode,
            );
            if filter.normalize_mode == GuiConfigNormalizeMode::Range {
                changed |= signed_f64_drag(ui, "Input Min V", &mut filter.input_min_v);
                changed |= signed_f64_drag(ui, "Input Max V", &mut filter.input_max_v);
                changed |= signed_f64_drag(ui, "Output Min", &mut filter.output_min);
                changed |= signed_f64_drag(ui, "Output Max", &mut filter.output_max);
            }
        }
        GuiConfigFilterKind::SoftLimit => {
            changed |= f64_drag(ui, "Limit V", &mut filter.limit_v, 0.001, 1_000_000.0);
        }
        GuiConfigFilterKind::Resample => {
            changed |= f64_drag(
                ui,
                "Interval s",
                &mut filter.sample_interval_s,
                0.000_001,
                1_000_000.0,
            );
        }
        GuiConfigFilterKind::Downsample => {
            changed |= usize_drag(ui, "Factor", &mut filter.factor, 1, 1_000_000);
        }
        GuiConfigFilterKind::Decimate => {
            changed |= usize_drag(ui, "Factor", &mut filter.factor, 1, 1_000_000);
            changed |= f64_drag(ui, "Cutoff Hz", &mut filter.cutoff_hz, 0.001, 1_000_000.0);
        }
        GuiConfigFilterKind::Invert
        | GuiConfigFilterKind::DcRemove
        | GuiConfigFilterKind::AbsoluteValue
        | GuiConfigFilterKind::Square
        | GuiConfigFilterKind::SquareRoot
        | GuiConfigFilterKind::Tanh
        | GuiConfigFilterKind::Sigmoid => {}
    });
    changed
}

fn criterion_parameter_controls(
    ui: &mut egui::Ui,
    id_prefix: String,
    current_channel: &str,
    channel_names: &[String],
    criterion: &mut GuiConfigCriterionDraft,
) -> bool {
    let mut changed = false;
    ui.horizontal_wrapped(|ui| match criterion.kind {
        GuiConfigCriterionKind::MinimumVoltage | GuiConfigCriterionKind::MaximumVoltage => {
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
        }
        GuiConfigCriterionKind::StateTransitions => {
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
            changed |= usize_drag(ui, "Expected", &mut criterion.expected_count, 1, 1_000_000);
        }
        GuiConfigCriterionKind::PulseWidth => {
            changed |= state_combo(ui, format!("{id_prefix}_state"), &mut criterion.state);
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
            changed |= f64_drag(
                ui,
                "Min Width s",
                &mut criterion.min_width_s,
                0.0,
                1_000_000.0,
            );
            changed |= f64_drag(
                ui,
                "Max Width s",
                &mut criterion.max_width_s,
                0.0,
                1_000_000.0,
            );
        }
        GuiConfigCriterionKind::TransientDuration => {
            changed |= state_combo(
                ui,
                format!("{id_prefix}_expected_state"),
                &mut criterion.expected_state,
            );
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
            changed |= f64_drag(
                ui,
                "Max Duration s",
                &mut criterion.max_duration_s,
                0.0,
                1_000_000.0,
            );
        }
        GuiConfigCriterionKind::TransientEvent => {
            changed |= event_kind_combo(
                ui,
                format!("{id_prefix}_event_kind"),
                &mut criterion.event_kind,
            );
            changed |= state_combo(
                ui,
                format!("{id_prefix}_expected_state"),
                &mut criterion.expected_state,
            );
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
            changed |= f64_drag(
                ui,
                "Max Duration s",
                &mut criterion.max_duration_s,
                0.0,
                1_000_000.0,
            );
            changed |= f64_drag(ui, "Start s", &mut criterion.start_time_s, 0.0, 1_000_000.0);
            changed |= f64_drag(ui, "End s", &mut criterion.end_time_s, 0.0, 1_000_000.0);
        }
        GuiConfigCriterionKind::StableStateDuration => {
            changed |= state_combo(ui, format!("{id_prefix}_state"), &mut criterion.state);
            changed |= signed_f64_drag(ui, "Threshold V", &mut criterion.threshold_v);
            changed |= f64_drag(
                ui,
                "Min Duration s",
                &mut criterion.min_duration_s,
                0.0,
                1_000_000.0,
            );
        }
        GuiConfigCriterionKind::RiseFallTime => {
            changed |= direction_combo(
                ui,
                format!("{id_prefix}_direction"),
                &mut criterion.direction,
            );
            changed |= signed_f64_drag(ui, "Low V", &mut criterion.low_threshold_v);
            changed |= signed_f64_drag(ui, "High V", &mut criterion.high_threshold_v);
            changed |= f64_drag(
                ui,
                "Max Duration s",
                &mut criterion.max_duration_s,
                0.0,
                1_000_000.0,
            );
        }
        GuiConfigCriterionKind::ResponseLatency => {
            if criterion.source_channel.trim().is_empty() {
                criterion.source_channel = default_source_channel(current_channel, channel_names);
                changed = true;
            }
            changed |= channel_combo(
                ui,
                format!("{id_prefix}_source_channel"),
                &mut criterion.source_channel,
                channel_names,
            );
            changed |= state_combo(
                ui,
                format!("{id_prefix}_source_state"),
                &mut criterion.source_state,
            );
            changed |= state_combo(
                ui,
                format!("{id_prefix}_target_state"),
                &mut criterion.expected_target_state,
            );
            changed |= signed_f64_drag(ui, "Source V", &mut criterion.source_threshold_v);
            changed |= signed_f64_drag(ui, "Target V", &mut criterion.target_threshold_v);
            changed |= f64_drag(
                ui,
                "Max Latency s",
                &mut criterion.max_latency_s,
                0.0,
                1_000_000.0,
            );
        }
    });
    changed
}

fn normalize_mode_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigNormalizeMode,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_NORMALIZE_MODE_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn state_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigState,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_STATE_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn direction_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigDirection,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_DIRECTION_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn event_kind_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut GuiConfigEventKind,
) -> bool {
    let before = *value;
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.label())
        .show_ui(ui, |ui| {
            for option in CONFIG_EVENT_KIND_OPTIONS {
                ui.selectable_value(value, *option, option.label());
            }
        });
    *value != before
}

fn channel_combo(
    ui: &mut egui::Ui,
    id_source: impl std::hash::Hash,
    value: &mut String,
    channel_names: &[String],
) -> bool {
    let before = value.clone();
    egui::ComboBox::from_id_source(id_source)
        .selected_text(value.as_str())
        .show_ui(ui, |ui| {
            for channel in channel_names {
                ui.selectable_value(value, channel.clone(), channel.as_str());
            }
        });
    *value != before
}

fn usize_drag(ui: &mut egui::Ui, label: &str, value: &mut usize, min: usize, max: usize) -> bool {
    let before = *value;
    ui.label(label);
    ui.add(egui::DragValue::new(value).range(min..=max));
    *value != before
}

fn f64_drag(ui: &mut egui::Ui, label: &str, value: &mut f64, min: f64, max: f64) -> bool {
    let before = *value;
    ui.label(label);
    ui.add(egui::DragValue::new(value).speed(0.01).range(min..=max));
    *value != before
}

fn signed_f64_drag(ui: &mut egui::Ui, label: &str, value: &mut f64) -> bool {
    let before = *value;
    ui.label(label);
    ui.add(egui::DragValue::new(value).speed(0.01));
    *value != before
}

fn default_source_channel(current_channel: &str, channel_names: &[String]) -> String {
    channel_names
        .iter()
        .find(|channel| channel.as_str() != current_channel)
        .or_else(|| channel_names.first())
        .cloned()
        .unwrap_or_else(|| current_channel.to_string())
}

fn nav_button(ui: &mut egui::Ui, active: &mut GuiTab, tab: GuiTab, label: &str) {
    if ui.selectable_label(*active == tab, label).clicked() {
        *active = tab;
    }
}

fn labeled_text(ui: &mut egui::Ui, label: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.text_edit_singleline(value);
    });
}

fn unit_combo(ui: &mut egui::Ui, label: &str, id_source: &str, value: &mut String) {
    ui.horizontal(|ui| {
        ui.label(label);
        egui::ComboBox::from_id_source(id_source)
            .selected_text(value.as_str())
            .show_ui(ui, |ui| {
                for unit in TIME_UNIT_OPTIONS {
                    ui.selectable_value(value, (*unit).to_string(), *unit);
                }
            });
    });
}

fn pick_csv_file(session: &mut GuiSession) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("CSV", &["csv"])
        .pick_file()
    {
        session.set_input_path(path.display().to_string());
    }
}

fn pick_config_open_file(session: &mut GuiSession) {
    let mut dialog = rfd::FileDialog::new().add_filter("TOML", &["toml"]);
    if let Some(directory) = default_config_directory(session) {
        dialog = dialog.set_directory(directory);
    }
    if let Some(path) = dialog.pick_file() {
        session.load_config_text_from_path(path.display().to_string());
    }
}

fn pick_config_save_file(session: &mut GuiSession) -> bool {
    let mut dialog = rfd::FileDialog::new()
        .add_filter("TOML", &["toml"])
        .set_file_name(default_config_file_name(session));
    if let Some(directory) = default_config_directory(session) {
        dialog = dialog.set_directory(directory);
    }
    if let Some(mut path) = dialog.save_file() {
        if path.extension().is_none() {
            path.set_extension("toml");
        }
        session.config_path = path.display().to_string();
        true
    } else {
        false
    }
}

fn pick_output_directory(session: &mut GuiSession) {
    let mut dialog = rfd::FileDialog::new();
    if let Some(directory) = default_output_directory(session) {
        dialog = dialog.set_directory(directory);
    }
    if let Some(path) = dialog.pick_folder() {
        session.output_dir = path.display().to_string();
    }
}

fn default_config_file_name(session: &GuiSession) -> String {
    let config_path = session.config_path.trim();
    if !config_path.is_empty() {
        let path = Path::new(config_path);
        if !path.is_dir() {
            if let Some(file_name) = path
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.trim().is_empty())
            {
                return file_name.to_string();
            }
        }
    }

    Path::new(session.input_path.trim())
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| !stem.trim().is_empty())
        .map(|stem| format!("{stem}-config.toml"))
        .unwrap_or_else(|| "ferrisoxide-config.toml".to_string())
}

fn default_config_directory(session: &GuiSession) -> Option<PathBuf> {
    [&session.config_path, &session.input_path]
        .iter()
        .filter_map(|candidate| {
            let candidate = candidate.trim();
            if candidate.is_empty() {
                return None;
            }
            let path = Path::new(candidate);
            if path.is_dir() {
                return Some(path.to_path_buf());
            }
            path.parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .map(Path::to_path_buf)
        })
        .next()
}

fn default_output_directory(session: &GuiSession) -> Option<PathBuf> {
    [
        &session.output_dir,
        &session.config_path,
        &session.input_path,
    ]
    .iter()
    .filter_map(|candidate| {
        let candidate = candidate.trim();
        if candidate.is_empty() {
            return None;
        }
        let path = Path::new(candidate);
        if path.is_dir() {
            return Some(path.to_path_buf());
        }
        path.parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map(Path::to_path_buf)
    })
    .next()
}

fn file_selector_label(input_path: &str) -> String {
    if input_path.trim().is_empty() {
        return "Select CSV file...".to_string();
    }
    Path::new(input_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(input_path)
        .to_string()
}

fn file_selector_hover(input_path: &str) -> String {
    if input_path.trim().is_empty() {
        "Browse for a CSV file".to_string()
    } else {
        input_path.to_string()
    }
}

fn output_directory_label(output_dir: &str) -> String {
    if output_dir.trim().is_empty() {
        return "Select output directory...".to_string();
    }
    Path::new(output_dir)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(output_dir)
        .to_string()
}

fn output_directory_hover(output_dir: &str) -> String {
    if output_dir.trim().is_empty() {
        "Choose where evaluation bundle outputs will be written".to_string()
    } else {
        output_dir.to_string()
    }
}

fn config_file_label(config_path: &str) -> String {
    if config_path.trim().is_empty() {
        return "No config file selected".to_string();
    }
    Path::new(config_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(config_path)
        .to_string()
}

fn config_file_hover(config_path: &str) -> String {
    if config_path.trim().is_empty() {
        "Open an existing TOML config or choose Save As for a new config".to_string()
    } else {
        config_path.to_string()
    }
}

fn text_area(ui: &mut egui::Ui, label: &str, value: &mut String, rows: usize) {
    ui.label(label);
    ui.add(
        egui::TextEdit::multiline(value)
            .desired_rows(rows)
            .desired_width(f32::INFINITY),
    );
}

fn read_only_text_area(ui: &mut egui::Ui, label: &str, value: &str, rows: usize) {
    let mut preview = value.to_string();
    ui.label(label);
    ui.add_enabled(
        false,
        egui::TextEdit::multiline(&mut preview)
            .desired_rows(rows)
            .desired_width(f32::INFINITY),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_file_name_uses_input_stem() {
        let session = GuiSession {
            input_path: "/tmp/ferrisoxide/run-42.csv".to_string(),
            ..GuiSession::default()
        };

        assert_eq!(default_config_file_name(&session), "run-42-config.toml");
    }

    #[test]
    fn default_config_file_name_prefers_existing_config_name() {
        let session = GuiSession {
            input_path: "/tmp/ferrisoxide/run-42.csv".to_string(),
            config_path: "/tmp/ferrisoxide/custom-analysis.toml".to_string(),
            ..GuiSession::default()
        };

        assert_eq!(default_config_file_name(&session), "custom-analysis.toml");
    }

    #[test]
    fn config_file_label_uses_file_name_or_empty_prompt() {
        assert_eq!(config_file_label(""), "No config file selected");
        assert_eq!(
            config_file_label("/tmp/ferrisoxide/custom-analysis.toml"),
            "custom-analysis.toml"
        );
    }

    #[test]
    fn output_directory_label_uses_directory_name_or_empty_prompt() {
        assert_eq!(output_directory_label(""), "Select output directory...");
        assert_eq!(
            output_directory_label("/tmp/ferrisoxide/run-output"),
            "run-output"
        );
    }
}

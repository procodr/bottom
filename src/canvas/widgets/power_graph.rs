use ratatui::{Frame, layout::Rect};

use crate::{
    app::App,
    canvas::{
        Painter,
        components::time_series::{AxisBound, ChartScaling, GraphData},
        drawing_utils::should_hide_x_label,
    },
    components::time_series::GraphDrawCtx,
};

impl Painter {
    pub fn draw_power_graph(
        &self, f: &mut Frame<'_>, app: &mut App, draw_loc: Rect, widget_id: u64,
    ) {
        if let Some(graph) = app.states.power_state.get_mut(&widget_id) {
            let data = &app.data_store.get_data().time_series_data;
            let max = graph
                .y_max(std::iter::once(&data.power), &data.time)
                .max(1.0)
                .ceil();
            let labels = [
                "0W".into(),
                format!("{:.1}W", max / 2.0).into(),
                format!("{max:.0}W").into(),
            ];
            let title = match data.latest_power {
                Some(watts) => format!(" PMIC power: {watts:.2} W "),
                None => " PMIC power: unavailable (vcgencmd) ".into(),
            };
            let hide_x_labels = should_hide_x_label(
                app.app_config_fields.hide_time,
                app.app_config_fields.autohide_time,
                graph.state_mut().autohide_timer_mut(),
                draw_loc,
            );
            graph.draw(
                f,
                draw_loc,
                GraphDrawCtx {
                    title: title.into(),
                    border_style: self.get_border_style(widget_id, app.current_widget.widget_id),
                    title_style: self.styles.widget_title_style,
                    graph_style: self.styles.graph_style,
                    general_widget_style: self.styles.general_widget_style,
                    border_type: self.styles.border_type,
                    marker: self.get_marker(app.app_config_fields.use_dot),
                    hide_x_labels,
                    is_selected: app.current_widget.widget_id == widget_id,
                    is_expanded: app.is_expanded,
                    legend_position: None,
                    legend_constraints: None,
                },
                AxisBound::Max(max),
                &labels,
                ChartScaling::Linear,
                vec![
                    GraphData::default()
                        .time(&data.time)
                        .values(&data.power)
                        .style(self.styles.ram_style),
                ],
            );
        }
        if app.should_get_widget_bounds()
            && let Some(widget) = app.widget_map.get_mut(&widget_id)
        {
            widget.top_left_corner = Some((draw_loc.x, draw_loc.y));
            widget.bottom_right_corner = Some((draw_loc.right(), draw_loc.bottom()));
        }
    }
}

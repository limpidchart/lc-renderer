use crate::error::RendererError;
use crate::margin::get_margins;
use crate::proto::render::chart_renderer_server::ChartRenderer;
use crate::proto::render::{RenderChartReply, RenderChartRequest};
use crate::scale::{
    get_h_scale, get_v_scale, set_chart_bottom_axis, set_chart_left_axis, set_chart_right_axis,
    set_chart_top_axis,
};
use crate::size::get_sizes;
use crate::view::get_views;
use lc_render::Chart;
use tonic::{Request, Response, Status};

const ERR_UNABLE_TO_RENDER_CHART: &str = "Unable to render chart";

const LOG_KEY_ERR: &str = "err";
const LOG_KEY_REQ_ID: &str = "request_id";

#[derive(Debug)]
pub struct RendererServer {
    log: slog::Logger,
}

impl RendererServer {
    pub(crate) fn new(log: slog::Logger) -> RendererServer {
        RendererServer { log }
    }
}

#[tonic::async_trait]
impl ChartRenderer for RendererServer {
    async fn render_chart(
        &self,
        request: Request<RenderChartRequest>,
    ) -> Result<Response<RenderChartReply>, Status> {
        // Convert a request into RenderChartRequest.
        let r_req = request.into_inner();

        // Prepare request logger with request_id set.
        let log = self.log.new(o!(LOG_KEY_REQ_ID => r_req.request_id.clone()));

        // Get chart scales.
        let axes = match r_req.axes {
            Some(axes) => axes,
            None => {
                let err = RendererError::ChartAxesAreNotSpecified.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };
        let h_scale = match get_h_scale(&axes) {
            Ok(h_scale) => h_scale,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };
        let v_scale = match get_v_scale(&axes) {
            Ok(v_scale) => v_scale,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Get chart sizes.
        let sizes = match get_sizes(r_req.sizes) {
            Ok(sizes) => sizes,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Get chart margins.
        let margins = match get_margins(r_req.margins) {
            Ok(margins) => margins,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Get chart views.
        let chart_views = match get_views(&r_req.views, &h_scale, &v_scale) {
            Ok(chart_views) => chart_views,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Prepare a chart.
        let mut chart = Chart::new()
            .set_width(sizes.width)
            .set_height(sizes.height)
            .set_margin_top(margins.top)
            .set_margin_bottom(margins.bottom)
            .set_margin_left(margins.left)
            .set_margin_right(margins.right)
            .set_title(&r_req.title)
            .set_views(chart_views.iter().map(Box::as_ref).collect());

        // Set the needed top axis.
        chart = match set_chart_top_axis(chart, axes.axis_top, axes.axis_top_label) {
            Ok(chart) => chart,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Set the needed bottom axis.
        chart = match set_chart_bottom_axis(chart, axes.axis_bottom, axes.axis_bottom_label) {
            Ok(chart) => chart,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Set the needed left axis.
        chart = match set_chart_left_axis(chart, axes.axis_left, axes.axis_left_label) {
            Ok(chart) => chart,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        // Set the needed right axis.
        chart = match set_chart_right_axis(chart, axes.axis_right, axes.axis_right_label) {
            Ok(chart) => chart,
            Err(err) => {
                let err = err.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };

        Ok(Response::new(RenderChartReply {
            request_id: r_req.request_id,
            chart_data: chart.to_svg().to_string().into_bytes(),
        }))
    }
}

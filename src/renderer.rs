use crate::error::RendererError;
use crate::proto::render::chart_renderer_server::ChartRenderer;
use crate::proto::render::chart_scale::ChartScaleKind;
use crate::proto::render::{RenderChartReply, RenderChartRequest};
use crate::scale::{
    get_band_h_scale, get_band_v_scale, get_h_scale, get_linear_h_scale, get_linear_v_scale,
    get_v_scale,
};
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

        // Get the needed parts that are required to render a chart.
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
        let sizes = match r_req.sizes {
            Some(sizes) => sizes,
            None => {
                let err = RendererError::ChartSizesAreNotSpecified.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };
        let margins = match r_req.margins {
            Some(margins) => margins,
            None => {
                let err = RendererError::ChartMarginsAreNotSpecified.to_string();
                error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                return Err(Status::invalid_argument(err));
            }
        };
        let chart_views = match get_views(&r_req.views, &sizes, &margins, &h_scale, &v_scale) {
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
            .set_margin_top(margins.margin_top)
            .set_margin_bottom(margins.margin_bottom)
            .set_margin_left(margins.margin_left)
            .set_margin_right(margins.margin_right)
            .set_title(&r_req.title)
            .set_views(chart_views.iter().map(Box::as_ref).collect());

        // Set the needed top axis.
        chart = match axes.axis_top {
            Some(_) => match ChartScaleKind::from_i32(h_scale.kind) {
                Some(ChartScaleKind::Band) => chart
                    .set_axis_top_band(get_band_h_scale(&h_scale, &sizes, &margins))
                    .set_axis_top_label(&*axes.axis_top_label),
                Some(ChartScaleKind::Linear) => chart
                    .set_axis_top_linear(get_linear_h_scale(&h_scale, &sizes, &margins))
                    .set_axis_top_label(&*axes.axis_top_label),
                _ => {
                    let err = RendererError::TopAxisIsSetButItsNotBandOrLinear.to_string();
                    error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                    return Err(Status::invalid_argument(err));
                }
            },
            _ => chart,
        };

        // Set the needed bottom axis.
        chart = match axes.axis_bottom {
            Some(_) => match ChartScaleKind::from_i32(h_scale.kind) {
                Some(ChartScaleKind::Band) => chart
                    .set_axis_bottom_band(get_band_h_scale(&h_scale, &sizes, &margins))
                    .set_axis_bottom_label(&*axes.axis_bottom_label),
                Some(ChartScaleKind::Linear) => chart
                    .set_axis_bottom_linear(get_linear_h_scale(&h_scale, &sizes, &margins))
                    .set_axis_bottom_label(&*axes.axis_bottom_label),
                _ => {
                    let err = RendererError::BottomAxisIsSetButItsNotBandOrLinear.to_string();
                    error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                    return Err(Status::invalid_argument(err));
                }
            },
            _ => chart,
        };

        // Set the needed left axis.
        chart = match axes.axis_left {
            Some(_) => match ChartScaleKind::from_i32(v_scale.kind) {
                Some(ChartScaleKind::Band) => chart
                    .set_axis_left_band(get_band_v_scale(&v_scale, &sizes, &margins))
                    .set_axis_left_label(&*axes.axis_left_label),
                Some(ChartScaleKind::Linear) => chart
                    .set_axis_left_linear(get_linear_v_scale(&v_scale, &sizes, &margins))
                    .set_axis_left_label(&*axes.axis_left_label),
                _ => {
                    let err = RendererError::LeftAxisIsSetButItsNotBandOrLinear.to_string();
                    error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                    return Err(Status::invalid_argument(err));
                }
            },
            _ => chart,
        };

        // Set the needed right axis.
        chart = match axes.axis_right {
            Some(_) => match ChartScaleKind::from_i32(v_scale.kind) {
                Some(ChartScaleKind::Band) => chart
                    .set_axis_right_band(get_band_v_scale(&v_scale, &sizes, &margins))
                    .set_axis_right_label(&*axes.axis_right_label),
                Some(ChartScaleKind::Linear) => chart
                    .set_axis_right_linear(get_linear_v_scale(&v_scale, &sizes, &margins))
                    .set_axis_right_label(&*axes.axis_right_label),
                _ => {
                    let err = RendererError::RightAxisIsSetButItsNotBandOrLinear.to_string();
                    error!(log, "{}", ERR_UNABLE_TO_RENDER_CHART; LOG_KEY_ERR => err.clone());
                    return Err(Status::invalid_argument(err));
                }
            },
            _ => chart,
        };

        Ok(Response::new(RenderChartReply {
            request_id: r_req.request_id,
            chart_data: chart.to_svg().to_string().into_bytes(),
        }))
    }
}

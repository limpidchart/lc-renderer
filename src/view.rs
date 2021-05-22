use crate::bar::get_bar_label_position;
use crate::color::get_view_colors;
use crate::error::RendererError;
use crate::point::{get_point_label_position, get_point_type};
use crate::proto::render::chart_view::ChartViewKind;
use crate::proto::render::{ChartMargins, ChartScale, ChartSizes, ChartView};
use crate::scale::{get_band_h_scale, get_band_v_scale, get_linear_h_scale, get_linear_v_scale};
use crate::value::{get_bars_values, get_points_values, get_scalar_values};
use lc_render::{AreaView, HorizontalBarView, LineView, ScatterView, VerticalBarView, View};

pub(crate) fn get_views(
    views: &[ChartView],
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<Vec<Box<dyn View>>, RendererError> {
    let mut result: Vec<Box<dyn View>> = Vec::new();

    for proto_view in views.iter() {
        match ChartViewKind::from_i32(proto_view.kind) {
            Some(ChartViewKind::Area) => {
                let area_view = get_area_view(proto_view, sizes, margins, h_scale, v_scale)?;
                result.push(Box::new(area_view));
            }
            Some(ChartViewKind::HorizontalBar) => {
                let horizontal_bar_view =
                    get_horizontal_bar_view(proto_view, sizes, margins, h_scale, v_scale)?;
                result.push(Box::new(horizontal_bar_view));
            }
            Some(ChartViewKind::Line) => {
                let line_view = get_line_view(proto_view, sizes, margins, h_scale, v_scale)?;
                result.push(Box::new(line_view));
            }
            Some(ChartViewKind::Scatter) => {
                let scatter_view = get_scatter_view(proto_view, sizes, margins, h_scale, v_scale)?;
                result.push(Box::new(scatter_view));
            }
            Some(ChartViewKind::VerticalBar) => {
                let vertical_bar_view =
                    get_vertical_bar_view(proto_view, sizes, margins, h_scale, v_scale)?;
                result.push(Box::new(vertical_bar_view));
            }
            _ => return Err(RendererError::ViewKindIsUnknown),
        }
    }

    Ok(result)
}

fn get_area_view(
    view: &ChartView,
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<AreaView, RendererError> {
    let values = get_scalar_values(view)?;
    let x_scale = get_band_h_scale(&h_scale, &sizes, &margins);
    let y_scale = get_linear_v_scale(&v_scale, &sizes, &margins);
    let point_type = get_point_type(view)?;
    let point_label_position = get_point_label_position(view)?;
    let view_colors = get_view_colors(view.colors.clone())?;
    let fill_color = match view_colors.fill_color {
        Some(fill_color) => fill_color,
        None => return Err(RendererError::ExpectedFillColorForAreaView),
    };
    let stroke_color = match view_colors.stroke_color {
        Some(stroke_color) => stroke_color,
        None => return Err(RendererError::ExpectedStrokeColorForAreaView),
    };
    let point_fill_color = match view_colors.point_fill_color {
        Some(point_fill_color) => point_fill_color,
        None => return Err(RendererError::ExpectedPointFillColorForAreaView),
    };
    let point_stroke_color = match view_colors.point_stroke_color {
        Some(point_stroke_color) => point_stroke_color,
        None => return Err(RendererError::ExpectedPointStrokeColorForAreaView),
    };
    let area_view = match AreaView::new(x_scale, y_scale)
        .set_fill_color(fill_color)
        .set_stroke_color(stroke_color)
        .set_point_fill_color(point_fill_color)
        .set_point_stroke_color(point_stroke_color)
        .set_point_type(point_type)
        .set_point_visible(view.point_visible)
        .set_point_label_visible(view.point_label_visible)
        .set_point_label_position(point_label_position)
        .set_data(&values)
    {
        Ok(area_view) => area_view,
        Err(e) => return Err(RendererError::RenderError(e)),
    };

    Ok(area_view)
}

fn get_horizontal_bar_view(
    view: &ChartView,
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<HorizontalBarView, RendererError> {
    let values = get_bars_values(view)?;
    let x_scale = get_linear_h_scale(&h_scale, &sizes, &margins);
    let y_scale = get_band_v_scale(&v_scale, &sizes, &margins);
    let bar_label_position = get_bar_label_position(view)?;
    let horizontal_bar_view = match HorizontalBarView::new(x_scale, y_scale)
        .set_bar_label_visible(view.bar_label_visible)
        .set_bar_label_position(bar_label_position)
        .set_data(&values)
    {
        Ok(horizontal_bar_view) => horizontal_bar_view,
        Err(e) => return Err(RendererError::RenderError(e)),
    };

    Ok(horizontal_bar_view)
}

fn get_line_view(
    view: &ChartView,
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<LineView, RendererError> {
    let values = get_scalar_values(view)?;
    let x_scale = get_band_h_scale(&h_scale, &sizes, &margins);
    let y_scale = get_linear_v_scale(&v_scale, &sizes, &margins);
    let point_type = get_point_type(view)?;
    let point_label_position = get_point_label_position(view)?;
    let view_colors = get_view_colors(view.colors.clone())?;
    let stroke_color = match view_colors.stroke_color {
        Some(stroke_color) => stroke_color,
        None => return Err(RendererError::ExpectedStrokeColorForLineView),
    };
    let point_fill_color = match view_colors.point_fill_color {
        Some(point_fill_color) => point_fill_color,
        None => return Err(RendererError::ExpectedPointFillColorForLineView),
    };
    let point_stroke_color = match view_colors.point_stroke_color {
        Some(point_stroke_color) => point_stroke_color,
        None => return Err(RendererError::ExpectedPointStrokeColorForLineView),
    };
    let line_view = match LineView::new(x_scale, y_scale)
        .set_stroke_color(stroke_color)
        .set_point_fill_color(point_fill_color)
        .set_point_stroke_color(point_stroke_color)
        .set_point_type(point_type)
        .set_point_visible(view.point_visible)
        .set_point_label_visible(view.point_label_visible)
        .set_point_label_position(point_label_position)
        .set_data(&values)
    {
        Ok(line_view) => line_view,
        Err(e) => return Err(RendererError::RenderError(e)),
    };

    Ok(line_view)
}

fn get_scatter_view(
    view: &ChartView,
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<ScatterView, RendererError> {
    let values = get_points_values(view)?;
    let x_scale = get_linear_h_scale(&h_scale, &sizes, &margins);
    let y_scale = get_linear_v_scale(&v_scale, &sizes, &margins);
    let point_type = get_point_type(view)?;
    let point_label_position = get_point_label_position(view)?;
    let view_colors = get_view_colors(view.colors.clone())?;
    let point_fill_color = match view_colors.point_fill_color {
        Some(point_fill_color) => point_fill_color,
        None => return Err(RendererError::ExpectedPointFillColorForScatterView),
    };
    let point_stroke_color = match view_colors.point_stroke_color {
        Some(point_stroke_color) => point_stroke_color,
        None => return Err(RendererError::ExpectedPointStrokeColorForScatterView),
    };
    let scatter_view = match ScatterView::new(x_scale, y_scale)
        .set_point_fill_color(point_fill_color)
        .set_point_stroke_color(point_stroke_color)
        .set_point_type(point_type)
        .set_point_visible(view.point_visible)
        .set_point_label_visible(view.point_label_visible)
        .set_point_label_position(point_label_position)
        .set_data(&values)
    {
        Ok(scatter_view) => scatter_view,
        Err(e) => return Err(RendererError::RenderError(e)),
    };

    Ok(scatter_view)
}

fn get_vertical_bar_view(
    view: &ChartView,
    sizes: &ChartSizes,
    margins: &ChartMargins,
    h_scale: &ChartScale,
    v_scale: &ChartScale,
) -> Result<VerticalBarView, RendererError> {
    let values = get_bars_values(view)?;
    let x_scale = get_band_v_scale(&h_scale, &sizes, &margins);
    let y_scale = get_linear_h_scale(&v_scale, &sizes, &margins);
    let bar_label_position = get_bar_label_position(view)?;
    let vertical_bar_view = match VerticalBarView::new(x_scale, y_scale)
        .set_bar_label_visible(view.bar_label_visible)
        .set_bar_label_position(bar_label_position)
        .set_data(&values)
    {
        Ok(vertical_bar_view) => vertical_bar_view,
        Err(e) => return Err(RendererError::RenderError(e)),
    };

    Ok(vertical_bar_view)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::render::chart_element_color::ColorValue;
    use crate::proto::render::chart_scale::ChartScaleKind;
    use crate::proto::render::chart_view::{
        ChartViewBarLabelPosition, ChartViewPointLabelPosition, ChartViewPointType, Values,
    };
    use crate::proto::render::chart_view_bars_values::BarsDataset;
    use crate::proto::render::chart_view_points_values::Point;
    use crate::proto::render::{
        ChartElementColor, ChartViewBarsValues, ChartViewColors, ChartViewPointsValues,
        ChartViewScalarValues,
    };

    fn chart_view_colors() -> ChartViewColors {
        ChartViewColors {
            fill_color: Some(ChartElementColor {
                color_value: Some(ColorValue::ColorHex("#2b4b49".to_string())),
            }),
            stroke_color: Some(ChartElementColor {
                color_value: Some(ColorValue::ColorHex("#226974".to_string())),
            }),
            point_fill_color: Some(ChartElementColor {
                color_value: Some(ColorValue::ColorHex("#1a888b".to_string())),
            }),
            point_stroke_color: Some(ChartElementColor {
                color_value: Some(ColorValue::ColorHex("#50c5b6".to_string())),
            }),
        }
    }

    fn chart_sizes() -> ChartSizes {
        ChartSizes {
            width: 1000,
            height: 800,
        }
    }

    fn chart_margins() -> ChartMargins {
        ChartMargins {
            margin_top: 20,
            margin_bottom: 70,
            margin_left: 40,
            margin_right: 30,
        }
    }

    fn chart_scale_linear() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Linear as i32,
            range_start: 0,
            range_end: 100,
            domain_num_start: 200_f32,
            domain_num_end: 800_f32,
            domain_categories: Vec::new(),
            no_boundaries_offset: false,
            inner_padding: 0.1_f32,
            outer_padding: 0.1_f32,
        }
    }

    fn chart_scale_band() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Band as i32,
            range_start: 0,
            range_end: 100,
            domain_num_start: 0_f32,
            domain_num_end: 0_f32,
            domain_categories: vec!["a".to_string(), "b".to_string()],
            no_boundaries_offset: false,
            inner_padding: 0.1_f32,
            outer_padding: 0.1_f32,
        }
    }

    fn chart_view_empty() -> ChartView {
        ChartView {
            kind: 0,
            x_scale: None,
            y_scale: None,
            colors: Some(chart_view_colors()),
            bar_label_visible: false,
            bar_label_position: 0,
            point_visible: false,
            point_type: 0,
            point_label_visible: false,
            point_label_position: 0,
            values: None,
        }
    }

    #[test]
    fn get_area_view_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::ScalarValues(ChartViewScalarValues {
            values: vec![4_f32, 8_f32],
        }));
        view.point_type = ChartViewPointType::X as i32;
        view.point_label_position = ChartViewPointLabelPosition::BottomLeft as i32;
        view.point_visible = true;
        view.point_label_visible = true;

        get_area_view(
            &view,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_band(),
            &chart_scale_linear(),
        )
        .unwrap();
    }

    #[test]
    fn get_horizontal_bar_view_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::BarsValues(ChartViewBarsValues {
            bars_datasets: vec![BarsDataset {
                values: vec![16_f32, 32_f32],
                fill_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#a496c4".to_string())),
                }),
                stroke_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#7d69ac".to_string())),
                }),
            }],
        }));
        view.bar_label_visible = true;
        view.bar_label_position = ChartViewBarLabelPosition::EndOutside as i32;

        get_horizontal_bar_view(
            &view,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_linear(),
            &chart_scale_band(),
        )
        .unwrap();
    }

    #[test]
    fn get_line_view_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::ScalarValues(ChartViewScalarValues {
            values: vec![4_f32, 8_f32],
        }));
        view.point_type = ChartViewPointType::Square as i32;
        view.point_label_position = ChartViewPointLabelPosition::TopLeft as i32;
        view.point_visible = true;
        view.point_label_visible = true;

        get_line_view(
            &view,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_band(),
            &chart_scale_linear(),
        )
        .unwrap();
    }

    #[test]
    fn get_scatter_view_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::PointsValues(ChartViewPointsValues {
            points: vec![
                Point { x: 2_f32, y: 4_f32 },
                Point {
                    x: 32_f32,
                    y: 16_f32,
                },
            ],
        }));
        view.point_type = ChartViewPointType::Circle as i32;
        view.point_label_position = ChartViewPointLabelPosition::TopRight as i32;
        view.point_visible = true;
        view.point_label_visible = true;

        get_scatter_view(
            &view,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_band(),
            &chart_scale_linear(),
        )
        .unwrap();
    }

    #[test]
    fn get_vertical_bar_view_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::BarsValues(ChartViewBarsValues {
            bars_datasets: vec![BarsDataset {
                values: vec![64_f32, 32_f32],
                fill_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#028c02".to_string())),
                }),
                stroke_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#02b502".to_string())),
                }),
            }],
        }));
        view.bar_label_visible = true;
        view.bar_label_position = ChartViewBarLabelPosition::StartInside as i32;

        get_vertical_bar_view(
            &view,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_band(),
            &chart_scale_linear(),
        )
        .unwrap();
    }

    #[test]
    fn get_views_basic() {
        let mut line_view = chart_view_empty();
        line_view.values = Some(Values::ScalarValues(ChartViewScalarValues {
            values: vec![10_f32, 100_f32],
        }));
        line_view.point_type = ChartViewPointType::Circle as i32;
        line_view.point_label_position = ChartViewPointLabelPosition::Right as i32;
        line_view.point_visible = true;
        line_view.point_label_visible = true;
        line_view.kind = ChartViewKind::Line as i32;

        let mut vertical_bar_view = chart_view_empty();
        vertical_bar_view.values = Some(Values::BarsValues(ChartViewBarsValues {
            bars_datasets: vec![BarsDataset {
                values: vec![20_f32, 200_f32],
                fill_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#cc6633".to_string())),
                }),
                stroke_color: Some(ChartElementColor {
                    color_value: Some(ColorValue::ColorHex("#ff9933".to_string())),
                }),
            }],
        }));
        vertical_bar_view.bar_label_visible = true;
        vertical_bar_view.bar_label_position = ChartViewBarLabelPosition::Center as i32;
        vertical_bar_view.kind = ChartViewKind::VerticalBar as i32;

        let views = vec![line_view, vertical_bar_view];

        get_views(
            &views,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_band(),
            &chart_scale_linear(),
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn get_views_err() {
        let views = vec![chart_view_empty()];
        get_views(
            &views,
            &chart_sizes(),
            &chart_margins(),
            &chart_scale_linear(),
            &chart_scale_band(),
        )
        .unwrap();
    }
}

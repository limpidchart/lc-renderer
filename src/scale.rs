use crate::error::RendererError;
use crate::proto::render::chart_scale::ChartScaleKind;
use crate::proto::render::{ChartAxes, ChartScale};
use lc_render::{BandScale, Chart, LinearScale};

// Get horizontal scale from protobuf.
pub(crate) fn get_h_scale(axes: &ChartAxes) -> Result<ChartScale, RendererError> {
    match (axes.axis_top.clone(), axes.axis_bottom.clone()) {
        (Some(axis_top), _) => Ok(axis_top),
        (_, Some(axis_bottom)) => Ok(axis_bottom),
        _ => Err(RendererError::TopOrBottomAxisShouldBeSpecified),
    }
}

// Get vertical scale from protobuf.
pub(crate) fn get_v_scale(axes: &ChartAxes) -> Result<ChartScale, RendererError> {
    match (axes.axis_left.clone(), axes.axis_right.clone()) {
        (Some(axis_left), _) => Ok(axis_left),
        (_, Some(axis_right)) => Ok(axis_right),
        _ => Err(RendererError::LeftOrRightAxisShouldBeSpecified),
    }
}

// Get linear horizontal scale from protobuf.
pub(crate) fn get_linear_h_scale(h_scale: &ChartScale) -> Result<LinearScale, RendererError> {
    let range_start = match h_scale.range_start {
        Some(range_start) => range_start,
        None => return Err(RendererError::ScaleRangeStartIsNotSpecified),
    };
    let range_end = match h_scale.range_end {
        Some(range_end) => range_end,
        None => return Err(RendererError::ScaleRangeEndIsNotSpecified),
    };
    Ok(LinearScale::new(
        h_scale.domain_num_start,
        h_scale.domain_num_end,
        range_start,
        range_end,
    ))
}

// Get linear vertical scale from protobuf.
pub(crate) fn get_linear_v_scale(v_scale: &ChartScale) -> Result<LinearScale, RendererError> {
    let range_start = match v_scale.range_start {
        Some(range_start) => range_start,
        None => return Err(RendererError::ScaleRangeStartIsNotSpecified),
    };
    let range_end = match v_scale.range_end {
        Some(range_end) => range_end,
        None => return Err(RendererError::ScaleRangeEndIsNotSpecified),
    };
    Ok(LinearScale::new(
        v_scale.domain_num_start,
        v_scale.domain_num_end,
        range_start,
        range_end,
    ))
}

// Get band horizontal scale from protobuf.
pub(crate) fn get_band_h_scale(h_scale: &ChartScale) -> Result<BandScale, RendererError> {
    let range_start = match h_scale.range_start {
        Some(range_start) => range_start,
        None => return Err(RendererError::ScaleRangeStartIsNotSpecified),
    };
    let range_end = match h_scale.range_end {
        Some(range_end) => range_end,
        None => return Err(RendererError::ScaleRangeEndIsNotSpecified),
    };
    let inner_padding = match h_scale.inner_padding {
        Some(inner_padding) => inner_padding,
        None => return Err(RendererError::BandScaleInnerPaddingIsNotSpecified),
    };
    let outer_padding = match h_scale.outer_padding {
        Some(outer_padding) => outer_padding,
        None => return Err(RendererError::BandScaleOuterPaddingIsNotSpecified),
    };
    Ok(
        BandScale::new(h_scale.domain_categories.clone(), range_start, range_end)
            .set_inner_padding(inner_padding)
            .set_outer_padding(outer_padding)
            .set_no_boundaries_offset(h_scale.no_boundaries_offset),
    )
}

// Get band vertical scale from protobuf.
pub(crate) fn get_band_v_scale(v_scale: &ChartScale) -> Result<BandScale, RendererError> {
    let range_start = match v_scale.range_start {
        Some(range_start) => range_start,
        None => return Err(RendererError::ScaleRangeStartIsNotSpecified),
    };
    let range_end = match v_scale.range_end {
        Some(range_end) => range_end,
        None => return Err(RendererError::ScaleRangeEndIsNotSpecified),
    };
    let inner_padding = match v_scale.inner_padding {
        Some(inner_padding) => inner_padding,
        None => return Err(RendererError::BandScaleInnerPaddingIsNotSpecified),
    };
    let outer_padding = match v_scale.outer_padding {
        Some(outer_padding) => outer_padding,
        None => return Err(RendererError::BandScaleOuterPaddingIsNotSpecified),
    };
    Ok(
        BandScale::new(v_scale.domain_categories.clone(), range_start, range_end)
            .set_inner_padding(inner_padding)
            .set_outer_padding(outer_padding)
            .set_no_boundaries_offset(v_scale.no_boundaries_offset),
    )
}

pub(crate) fn set_chart_top_axis(
    chart: Chart,
    scale: Option<ChartScale>,
    label: String,
) -> Result<Chart, RendererError> {
    match scale {
        Some(scale) => match ChartScaleKind::from_i32(scale.kind) {
            Some(ChartScaleKind::Band) => {
                let h_scale = match get_band_h_scale(&scale) {
                    Ok(h_scale) => h_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart.set_axis_top_band(h_scale).set_axis_top_label(&label))
            }
            Some(ChartScaleKind::Linear) => {
                let h_scale = match get_linear_h_scale(&scale) {
                    Ok(h_scale) => h_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_top_linear(h_scale)
                    .set_axis_top_label(&label))
            }
            _ => Err(RendererError::TopAxisIsSetButItsNotBandOrLinear),
        },
        _ => Ok(chart),
    }
}

pub(crate) fn set_chart_bottom_axis(
    chart: Chart,
    scale: Option<ChartScale>,
    label: String,
) -> Result<Chart, RendererError> {
    match scale {
        Some(scale) => match ChartScaleKind::from_i32(scale.kind) {
            Some(ChartScaleKind::Band) => {
                let h_scale = match get_band_h_scale(&scale) {
                    Ok(h_scale) => h_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_bottom_band(h_scale)
                    .set_axis_bottom_label(&label))
            }
            Some(ChartScaleKind::Linear) => {
                let h_scale = match get_linear_h_scale(&scale) {
                    Ok(h_scale) => h_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_bottom_linear(h_scale)
                    .set_axis_bottom_label(&label))
            }
            _ => Err(RendererError::BottomAxisIsSetButItsNotBandOrLinear),
        },
        _ => Ok(chart),
    }
}

pub(crate) fn set_chart_left_axis(
    chart: Chart,
    scale: Option<ChartScale>,
    label: String,
) -> Result<Chart, RendererError> {
    match scale {
        Some(scale) => match ChartScaleKind::from_i32(scale.kind) {
            Some(ChartScaleKind::Band) => {
                let v_scale = match get_band_v_scale(&scale) {
                    Ok(v_scale) => v_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_left_band(v_scale)
                    .set_axis_left_label(&label))
            }
            Some(ChartScaleKind::Linear) => {
                let v_scale = match get_linear_v_scale(&scale) {
                    Ok(v_scale) => v_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_left_linear(v_scale)
                    .set_axis_left_label(&label))
            }
            _ => Err(RendererError::LeftAxisIsSetButItsNotBandOrLinear),
        },
        _ => Ok(chart),
    }
}

pub(crate) fn set_chart_right_axis(
    chart: Chart,
    scale: Option<ChartScale>,
    label: String,
) -> Result<Chart, RendererError> {
    match scale {
        Some(scale) => match ChartScaleKind::from_i32(scale.kind) {
            Some(ChartScaleKind::Band) => {
                let v_scale = match get_band_v_scale(&scale) {
                    Ok(v_scale) => v_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_right_band(v_scale)
                    .set_axis_right_label(&label))
            }
            Some(ChartScaleKind::Linear) => {
                let v_scale = match get_linear_v_scale(&scale) {
                    Ok(v_scale) => v_scale,
                    Err(err) => return Err(err),
                };
                Ok(chart
                    .set_axis_right_linear(v_scale)
                    .set_axis_right_label(&label))
            }
            _ => Err(RendererError::RightAxisIsSetButItsNotBandOrLinear),
        },
        _ => Ok(chart),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::render::chart_scale::ChartScaleKind;
    use lc_render::{Scale, ScaleKind};

    fn chart_scale_linear() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Linear as i32,
            range_start: Some(10),
            range_end: Some(1000),
            domain_num_start: 80_f32,
            domain_num_end: 160_f32,
            domain_categories: Vec::new(),
            no_boundaries_offset: true,
            inner_padding: Some(0_f32),
            outer_padding: Some(0_f32),
        }
    }

    fn chart_scale_band() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Band as i32,
            range_start: Some(100),
            range_end: Some(10000),
            domain_num_start: 120_f32,
            domain_num_end: 300_f32,
            domain_categories: vec!["a".to_string(), "b".to_string()],
            no_boundaries_offset: false,
            inner_padding: Some(10_f32),
            outer_padding: Some(20_f32),
        }
    }

    fn chart_axes_empty() -> ChartAxes {
        ChartAxes {
            axis_top: None,
            axis_top_label: String::new(),
            axis_bottom: None,
            axis_bottom_label: String::new(),
            axis_left: None,
            axis_left_label: String::new(),
            axis_right: None,
            axis_right_label: String::new(),
        }
    }

    #[test]
    fn get_h_scale_top() {
        let mut axes = chart_axes_empty();
        axes.axis_top = Some(chart_scale_band());

        let h_scale = get_h_scale(&axes).unwrap();

        assert_eq!(ChartScaleKind::Band, h_scale.kind());
    }

    #[test]
    fn get_h_scale_bottom() {
        let mut axes = chart_axes_empty();
        axes.axis_bottom = Some(chart_scale_band());

        let h_scale = get_h_scale(&axes).unwrap();

        assert_eq!(ChartScaleKind::Band, h_scale.kind());
    }

    #[test]
    #[should_panic]
    fn get_h_scale_err() {
        let axes = chart_axes_empty();

        get_h_scale(&axes).unwrap();
    }

    #[test]
    fn get_v_scale_left() {
        let mut axes = chart_axes_empty();
        axes.axis_left = Some(chart_scale_linear());

        let v_scale = get_v_scale(&axes).unwrap();

        assert_eq!(ChartScaleKind::Linear, v_scale.kind());
    }

    #[test]
    fn get_v_scale_right() {
        let mut axes = chart_axes_empty();
        axes.axis_right = Some(chart_scale_linear());

        let v_scale = get_v_scale(&axes).unwrap();

        assert_eq!(ChartScaleKind::Linear, v_scale.kind());
    }

    #[test]
    #[should_panic]
    fn get_v_scale_err() {
        let axes = chart_axes_empty();

        get_v_scale(&axes).unwrap();
    }

    #[test]
    fn get_linear_h_scale_basic() {
        let scale = chart_scale_linear();

        let linear_scale = get_linear_h_scale(&scale).unwrap();

        assert_eq!(10, linear_scale.range_start());
        assert_eq!(1000, linear_scale.range_end());
        assert_eq!(ScaleKind::Linear, linear_scale.kind());
    }

    #[test]
    fn get_linear_v_scale_basic() {
        let scale = chart_scale_linear();

        let linear_scale = get_linear_v_scale(&scale).unwrap();

        assert_eq!(10, linear_scale.range_start());
        assert_eq!(1000, linear_scale.range_end());
        assert_eq!(ScaleKind::Linear, linear_scale.kind());
    }

    #[test]
    fn get_band_h_scale_basic() {
        let scale = chart_scale_band();

        let band_scale = get_band_h_scale(&scale).unwrap();

        assert_eq!(100, band_scale.range_start());
        assert_eq!(10000, band_scale.range_end());
        assert_eq!(ScaleKind::Band, band_scale.kind());
    }

    #[test]
    fn get_band_v_scale_basic() {
        let scale = chart_scale_band();

        let band_scale = get_band_v_scale(&scale).unwrap();

        assert_eq!(100, band_scale.range_start());
        assert_eq!(10000, band_scale.range_end());
        assert_eq!(ScaleKind::Band, band_scale.kind());
    }
}

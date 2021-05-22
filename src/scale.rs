use crate::error::RendererError;
use crate::proto::render::{ChartAxes, ChartMargins, ChartScale, ChartSizes};
use lc_render::{BandScale, LinearScale};

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
pub(crate) fn get_linear_h_scale(
    h_scale: &ChartScale,
    sizes: &ChartSizes,
    margins: &ChartMargins,
) -> LinearScale {
    LinearScale::new(
        h_scale.domain_num_start,
        h_scale.domain_num_end,
        h_scale.range_start,
        sizes.width - margins.margin_left - margins.margin_right,
    )
}

// Get linear vertical scale from protobuf.
pub(crate) fn get_linear_v_scale(
    v_scale: &ChartScale,
    sizes: &ChartSizes,
    margins: &ChartMargins,
) -> LinearScale {
    LinearScale::new(
        v_scale.domain_num_start,
        v_scale.domain_num_end,
        sizes.height - margins.margin_top - margins.margin_bottom,
        v_scale.range_end,
    )
}

// Get band horizontal scale from protobuf.
pub(crate) fn get_band_h_scale(
    h_scale: &ChartScale,
    sizes: &ChartSizes,
    margins: &ChartMargins,
) -> BandScale {
    BandScale::new(
        h_scale.domain_categories.clone(),
        h_scale.range_start,
        sizes.width - margins.margin_left - margins.margin_right,
    )
    .set_inner_padding(h_scale.inner_padding)
    .set_outer_padding(h_scale.outer_padding)
    .set_no_boundaries_offset(h_scale.no_boundaries_offset)
}

// Get band vertical scale from protobuf.
pub(crate) fn get_band_v_scale(
    v_scale: &ChartScale,
    sizes: &ChartSizes,
    margins: &ChartMargins,
) -> BandScale {
    BandScale::new(
        v_scale.domain_categories.clone(),
        v_scale.range_start,
        sizes.height - margins.margin_top - margins.margin_bottom,
    )
    .set_inner_padding(v_scale.inner_padding)
    .set_outer_padding(v_scale.outer_padding)
    .set_no_boundaries_offset(v_scale.no_boundaries_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::render::chart_scale::ChartScaleKind;
    use lc_render::{Scale, ScaleKind};

    fn chart_scale_linear() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Linear as i32,
            range_start: 10,
            range_end: 1000,
            domain_num_start: 80_f32,
            domain_num_end: 160_f32,
            domain_categories: Vec::new(),
            no_boundaries_offset: true,
            inner_padding: 0_f32,
            outer_padding: 0_f32,
        }
    }

    fn chart_scale_band() -> ChartScale {
        ChartScale {
            kind: ChartScaleKind::Band as i32,
            range_start: 100,
            range_end: 10000,
            domain_num_start: 120_f32,
            domain_num_end: 300_f32,
            domain_categories: vec!["a".to_string(), "b".to_string()],
            no_boundaries_offset: false,
            inner_padding: 10_f32,
            outer_padding: 20_f32,
        }
    }

    fn chart_sizes() -> ChartSizes {
        ChartSizes {
            width: 800,
            height: 600,
        }
    }

    fn chart_margins() -> ChartMargins {
        ChartMargins {
            margin_top: 100,
            margin_bottom: 10,
            margin_left: 200,
            margin_right: 20,
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
        let sizes = chart_sizes();
        let margins = chart_margins();

        let linear_scale = get_linear_h_scale(&scale, &sizes, &margins);

        assert_eq!(10, linear_scale.range_start());
        assert_eq!(580, linear_scale.range_end());
        assert_eq!(ScaleKind::Linear, linear_scale.kind());
    }

    #[test]
    fn get_linear_v_scale_basic() {
        let scale = chart_scale_linear();
        let sizes = chart_sizes();
        let margins = chart_margins();

        let linear_scale = get_linear_v_scale(&scale, &sizes, &margins);

        assert_eq!(490, linear_scale.range_start());
        assert_eq!(1000, linear_scale.range_end());
        assert_eq!(ScaleKind::Linear, linear_scale.kind());
    }

    #[test]
    fn get_band_h_scale_basic() {
        let scale = chart_scale_band();
        let sizes = chart_sizes();
        let margins = chart_margins();

        let band_scale = get_band_h_scale(&scale, &sizes, &margins);

        assert_eq!(100, band_scale.range_start());
        assert_eq!(580, band_scale.range_end());
        assert_eq!(ScaleKind::Band, band_scale.kind());
    }

    #[test]
    fn get_band_v_scale_basic() {
        let scale = chart_scale_band();
        let sizes = chart_sizes();
        let margins = chart_margins();

        let band_scale = get_band_v_scale(&scale, &sizes, &margins);

        assert_eq!(100, band_scale.range_start());
        assert_eq!(490, band_scale.range_end());
        assert_eq!(ScaleKind::Band, band_scale.kind());
    }
}

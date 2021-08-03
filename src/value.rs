use crate::color::get_color;
use crate::error::RendererError;
use crate::proto::render::chart_view::Values;
use crate::proto::render::{
    ChartView, ChartViewBarsValues, ChartViewPointsValues, ChartViewScalarValues,
};
use lc_render::BarsValues;

// Get scalar values from chart view.
pub(crate) fn get_scalar_values(view: &ChartView) -> Result<&[f32], RendererError> {
    let chart_view_scalar_values: &ChartViewScalarValues;

    match &view.values {
        Some(view_values) => {
            chart_view_scalar_values = match view_values {
                Values::ScalarValues(scalar_values) => scalar_values,
                _ => return Err(RendererError::ExpectedScalarValues),
            };
        }
        None => return Err(RendererError::ViewValuesAreNotSpecified),
    };

    Ok(&chart_view_scalar_values.values)
}

// Get bars values from chart view.
pub(crate) fn get_bars_values(view: &ChartView) -> Result<Vec<BarsValues>, RendererError> {
    let chart_view_bars_values: &ChartViewBarsValues;

    match &view.values {
        Some(view_values) => {
            chart_view_bars_values = match view_values {
                Values::BarsValues(bars_values) => bars_values,
                _ => return Err(RendererError::ExpectedBarsValues),
            };
        }
        None => return Err(RendererError::ViewValuesAreNotSpecified),
    };

    let mut res = Vec::with_capacity(chart_view_bars_values.bars_datasets.len());
    for bars_value in chart_view_bars_values.bars_datasets.iter() {
        let colors = match bars_value.colors.clone() {
            Some(colors) => colors,
            None => return Err(RendererError::ColorsForBarsValuesAreNotSpecified),
        };
        let fill_color = match get_color(colors.fill.clone()) {
            Some(fill_color) => fill_color,
            None => return Err(RendererError::FillColorForBarsValuesIsNotSpecified),
        };
        let stroke_color = match get_color(colors.stroke.clone()) {
            Some(stroke_color) => stroke_color,
            None => return Err(RendererError::StrokeColorForBarsValuesIsNotSpecified),
        };
        res.push(
            BarsValues::new(bars_value.values.clone())
                .set_fill_color(fill_color)
                .set_stroke_color(stroke_color),
        );
    }

    Ok(res)
}

// Get points values from chart view.
pub(crate) fn get_points_values(view: &ChartView) -> Result<Vec<(f32, f32)>, RendererError> {
    let chart_view_points_values: &ChartViewPointsValues;

    match &view.values {
        Some(view_values) => {
            chart_view_points_values = match view_values {
                Values::PointsValues(chart_view_points_values) => chart_view_points_values,
                _ => return Err(RendererError::ExpectedPointsValues),
            };
        }
        None => return Err(RendererError::ViewValuesAreNotSpecified),
    };

    let mut values: Vec<(f32, f32)> = Vec::with_capacity(chart_view_points_values.points.len());
    for points_value in chart_view_points_values.points.iter() {
        values.push((points_value.x, points_value.y));
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::render::chart_element_color::ColorValue;
    use crate::proto::render::chart_view_bars_values::{BarsDataset, ChartViewBarsColors};
    use crate::proto::render::chart_view_points_values::Point;
    use crate::proto::render::ChartElementColor;
    use lc_render::Color;

    fn chart_view_empty() -> ChartView {
        ChartView {
            kind: 0,
            colors: None,
            bar_label_visible: Some(false),
            bar_label_position: 0,
            point_visible: Some(false),
            point_type: 0,
            point_label_visible: Some(false),
            point_label_position: 0,
            values: None,
        }
    }

    #[test]
    fn get_scalar_values_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::ScalarValues(ChartViewScalarValues {
            values: vec![1_f32, 2_f32],
        }));

        let scalar_values = get_scalar_values(&view).unwrap();

        assert_eq!(vec![1_f32, 2_f32], scalar_values);
    }

    #[test]
    fn get_bars_values_basic() {
        let expected_bars_values = vec![
            BarsValues::new(vec![1_f32, 2_f32])
                .set_fill_color(Color::new_from_hex("#FA4988"))
                .set_stroke_color(Color::new_from_hex("#9C0412")),
            BarsValues::new(vec![3_f32, 4_f32])
                .set_fill_color(Color::new_from_hex("#A9DEF2"))
                .set_stroke_color(Color::new_from_hex("#004F84")),
        ];

        let mut view = chart_view_empty();
        view.values = Some(Values::BarsValues(ChartViewBarsValues {
            bars_datasets: vec![
                BarsDataset {
                    values: vec![1_f32, 2_f32],
                    colors: Some(ChartViewBarsColors {
                        fill: Some(ChartElementColor {
                            color_value: Some(ColorValue::ColorHex("#FA4988".to_string())),
                        }),
                        stroke: Some(ChartElementColor {
                            color_value: Some(ColorValue::ColorHex("#9C0412".to_string())),
                        }),
                    }),
                },
                BarsDataset {
                    values: vec![3_f32, 4_f32],
                    colors: Some(ChartViewBarsColors {
                        fill: Some(ChartElementColor {
                            color_value: Some(ColorValue::ColorHex("#A9DEF2".to_string())),
                        }),
                        stroke: Some(ChartElementColor {
                            color_value: Some(ColorValue::ColorHex("#004F84".to_string())),
                        }),
                    }),
                },
            ],
        }));

        let bars_values = get_bars_values(&view).unwrap();

        for (i, bars_value) in bars_values.iter().enumerate() {
            assert_eq!(expected_bars_values[i].values(), bars_value.values());
            assert_eq!(
                expected_bars_values[i].fill_color(),
                bars_value.fill_color()
            );
            assert_eq!(
                expected_bars_values[i].stroke_color(),
                bars_value.stroke_color()
            );
        }
    }

    #[test]
    fn get_points_values_basic() {
        let mut view = chart_view_empty();
        view.values = Some(Values::PointsValues(ChartViewPointsValues {
            points: vec![Point { x: 1_f32, y: 2_f32 }, Point { x: 3_f32, y: 4_f32 }],
        }));

        let points_values = get_points_values(&view).unwrap();

        assert_eq!(vec![(1_f32, 2_f32), (3_f32, 4_f32)], points_values);
    }

    #[test]
    #[should_panic]
    fn get_scalar_values_err() {
        let view = chart_view_empty();

        get_scalar_values(&view).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_bars_values_err() {
        let view = chart_view_empty();

        get_bars_values(&view).unwrap();
    }

    #[test]
    #[should_panic]
    fn get_points_values_err() {
        let view = chart_view_empty();

        get_points_values(&view).unwrap();
    }
}

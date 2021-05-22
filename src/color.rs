use crate::error::RendererError;
use crate::proto::render::chart_element_color::ColorValue;
use crate::proto::render::{ChartElementColor, ChartViewColors};
use lc_render::Color;

// Get lc-render color from protobuf color.
pub(crate) fn get_color(chart_element_color: Option<ChartElementColor>) -> Option<Color> {
    match chart_element_color {
        Some(chart_element_color) => match chart_element_color.color_value {
            Some(ColorValue::ColorHex(hex)) => Some(Color::new_from_hex(&hex)),
            Some(ColorValue::ColorRgb(rgb)) => {
                Some(Color::new_from_rgb(rgb.r as u8, rgb.g as u8, rgb.b as u8))
            }
            None => None,
        },
        None => None,
    }
}

// ViewColors contains all colors retrieved from protobuf.
struct ViewColors {
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    point_fill_color: Option<Color>,
    point_stroke_color: Option<Color>,
}

// Get ViewColors from protobuf.
fn get_view_colors(
    chart_view_colors: Option<ChartViewColors>,
) -> Result<ViewColors, RendererError> {
    let fill_color: Option<Color>;
    let stroke_color: Option<Color>;
    let point_fill_color: Option<Color>;
    let point_stroke_color: Option<Color>;

    match chart_view_colors {
        Some(chart_view_colors) => {
            fill_color = get_color(chart_view_colors.fill_color);
            stroke_color = get_color(chart_view_colors.stroke_color);
            point_fill_color = get_color(chart_view_colors.point_fill_color);
            point_stroke_color = get_color(chart_view_colors.point_stroke_color);
        }
        None => return Err(RendererError::ViewColorsAreNotSpecified),
    }

    Ok(ViewColors {
        fill_color,
        stroke_color,
        point_fill_color,
        point_stroke_color,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::proto::render::chart_element_color::Rgb;

    fn color_hex() -> Option<ChartElementColor> {
        Some(ChartElementColor {
            color_value: Some(ColorValue::ColorHex("#0E0C50".to_string())),
        })
    }

    fn color_rgb() -> Option<ChartElementColor> {
        Some(ChartElementColor {
            color_value: Some(ColorValue::ColorRgb(Rgb {
                r: 14,
                g: 12,
                b: 80,
            })),
        })
    }

    #[test]
    fn get_color_hex() {
        let chart_element_color = color_hex();

        let color = get_color(chart_element_color);

        assert_eq!("#0E0C50".to_string(), color.unwrap().to_string())
    }

    #[test]
    fn get_color_rgb() {
        let chart_element_color = color_rgb();

        let color = get_color(chart_element_color);

        assert_eq!("rgb(14,12,80)".to_string(), color.unwrap().to_string())
    }

    #[test]
    fn get_colors_basic() {
        let chart_view_colors = ChartViewColors {
            fill_color: color_hex(),
            stroke_color: color_rgb(),
            point_fill_color: color_hex(),
            point_stroke_color: color_rgb(),
        };

        let view_colors = get_view_colors(Some(chart_view_colors)).unwrap();

        assert_eq!(
            "#0E0C50".to_string(),
            view_colors.fill_color.unwrap().to_string()
        );
        assert_eq!(
            "rgb(14,12,80)".to_string(),
            view_colors.stroke_color.unwrap().to_string()
        );
        assert_eq!(
            "#0E0C50".to_string(),
            view_colors.point_fill_color.unwrap().to_string()
        );
        assert_eq!(
            "rgb(14,12,80)".to_string(),
            view_colors.point_stroke_color.unwrap().to_string()
        );
    }
}

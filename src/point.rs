use crate::error::RendererError;
use crate::proto::render::chart_view::{ChartViewPointLabelPosition, ChartViewPointType};
use crate::proto::render::ChartView;
use lc_render::{PointLabelPosition, PointType};

// Get label position for point from protobuf.
fn get_point_label_position(view: &ChartView) -> Result<PointLabelPosition, RendererError> {
    match ChartViewPointLabelPosition::from_i32(view.point_label_position) {
        Some(ChartViewPointLabelPosition::Top) => Ok(PointLabelPosition::Top),
        Some(ChartViewPointLabelPosition::TopLeft) => Ok(PointLabelPosition::TopLeft),
        Some(ChartViewPointLabelPosition::TopRight) => Ok(PointLabelPosition::TopRight),
        Some(ChartViewPointLabelPosition::Left) => Ok(PointLabelPosition::Left),
        Some(ChartViewPointLabelPosition::Right) => Ok(PointLabelPosition::Right),
        Some(ChartViewPointLabelPosition::Bottom) => Ok(PointLabelPosition::Bottom),
        Some(ChartViewPointLabelPosition::BottomLeft) => Ok(PointLabelPosition::BottomLeft),
        Some(ChartViewPointLabelPosition::BottomRight) => Ok(PointLabelPosition::BottomRight),
        _ => Err(RendererError::PointLabelPositionIsUnknown),
    }
}

// Get point type from protobuf.
fn get_point_type(view: &ChartView) -> Result<PointType, RendererError> {
    match ChartViewPointType::from_i32(view.point_type) {
        Some(ChartViewPointType::Circle) => Ok(PointType::Circle),
        Some(ChartViewPointType::Square) => Ok(PointType::Square),
        Some(ChartViewPointType::X) => Ok(PointType::X),
        _ => Err(RendererError::PointTypeIsUnknown),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chart_view_empty() -> ChartView {
        ChartView {
            kind: 0,
            x_scale: None,
            y_scale: None,
            colors: None,
            bar_label_visible: false,
            bar_label_position: 0,
            point_visible: false,
            point_type: ChartViewPointType::UnspecifiedPointType as i32,
            point_label_visible: false,
            point_label_position: ChartViewPointLabelPosition::UnspecifiedPointLabelPosition as i32,
            values: None,
        }
    }

    #[test]
    fn get_point_label_position_top() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::Top as i32;

        assert_eq!(
            PointLabelPosition::Top,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_top_left() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::TopLeft as i32;

        assert_eq!(
            PointLabelPosition::TopLeft,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_top_right() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::TopRight as i32;

        assert_eq!(
            PointLabelPosition::TopRight,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_left() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::Left as i32;

        assert_eq!(
            PointLabelPosition::Left,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_right() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::Right as i32;

        assert_eq!(
            PointLabelPosition::Right,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_bottom() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::Bottom as i32;

        assert_eq!(
            PointLabelPosition::Bottom,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_bottom_left() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::BottomLeft as i32;

        assert_eq!(
            PointLabelPosition::BottomLeft,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_point_label_position_bottom_right() {
        let mut view = chart_view_empty();
        view.point_label_position = ChartViewPointLabelPosition::BottomRight as i32;

        assert_eq!(
            PointLabelPosition::BottomRight,
            get_point_label_position(&view).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn get_point_label_position_unknown() {
        let view = chart_view_empty();

        get_point_label_position(&view).unwrap();
    }

    #[test]
    fn get_point_type_circle() {
        let mut view = chart_view_empty();
        view.point_type = ChartViewPointType::Circle as i32;

        assert_eq!(PointType::Circle, get_point_type(&view).unwrap());
    }

    #[test]
    fn get_point_type_square() {
        let mut view = chart_view_empty();
        view.point_type = ChartViewPointType::Square as i32;

        assert_eq!(PointType::Square, get_point_type(&view).unwrap());
    }

    #[test]
    fn get_point_type_x() {
        let mut view = chart_view_empty();
        view.point_type = ChartViewPointType::X as i32;

        assert_eq!(PointType::X, get_point_type(&view).unwrap());
    }

    #[test]
    #[should_panic]
    fn get_point_type_unknown() {
        let view = chart_view_empty();

        get_point_type(&view).unwrap();
    }
}

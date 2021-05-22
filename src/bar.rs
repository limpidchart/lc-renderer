use crate::error::RendererError;
use crate::proto::render::chart_view::ChartViewBarLabelPosition;
use crate::proto::render::ChartView;
use lc_render::BarLabelPosition;

// Get bar label position from protobuf.
pub(crate) fn get_bar_label_position(view: &ChartView) -> Result<BarLabelPosition, RendererError> {
    match ChartViewBarLabelPosition::from_i32(view.bar_label_position) {
        Some(ChartViewBarLabelPosition::Center) => Ok(BarLabelPosition::Center),
        Some(ChartViewBarLabelPosition::EndInside) => Ok(BarLabelPosition::EndInside),
        Some(ChartViewBarLabelPosition::EndOutside) => Ok(BarLabelPosition::EndOutside),
        Some(ChartViewBarLabelPosition::StartInside) => Ok(BarLabelPosition::StartInside),
        Some(ChartViewBarLabelPosition::StartOutside) => Ok(BarLabelPosition::StartOutside),
        _ => Err(RendererError::BarLabelPositionIsUnknown),
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
            bar_label_position: ChartViewBarLabelPosition::UnspecifiedBarLabelPosition as i32,
            point_visible: false,
            point_type: 0,
            point_label_visible: false,
            point_label_position: 0,
            values: None,
        }
    }

    #[test]
    fn get_bar_label_position_center() {
        let mut view = chart_view_empty();
        view.bar_label_position = ChartViewBarLabelPosition::Center as i32;

        assert_eq!(
            BarLabelPosition::Center,
            get_bar_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_bar_label_position_end_inside() {
        let mut view = chart_view_empty();
        view.bar_label_position = ChartViewBarLabelPosition::EndInside as i32;

        assert_eq!(
            BarLabelPosition::EndInside,
            get_bar_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_bar_label_position_end_outside() {
        let mut view = chart_view_empty();
        view.bar_label_position = ChartViewBarLabelPosition::EndOutside as i32;

        assert_eq!(
            BarLabelPosition::EndOutside,
            get_bar_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_bar_label_position_start_inside() {
        let mut view = chart_view_empty();
        view.bar_label_position = ChartViewBarLabelPosition::StartInside as i32;

        assert_eq!(
            BarLabelPosition::StartInside,
            get_bar_label_position(&view).unwrap()
        );
    }

    #[test]
    fn get_bar_label_position_start_outside() {
        let mut view = chart_view_empty();
        view.bar_label_position = ChartViewBarLabelPosition::StartOutside as i32;

        assert_eq!(
            BarLabelPosition::StartOutside,
            get_bar_label_position(&view).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn get_bar_label_position_unknown() {
        let view = chart_view_empty();

        get_bar_label_position(&view).unwrap();
    }
}

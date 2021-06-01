use crate::error::RendererError;
use crate::proto::render::ChartMargins;

pub(crate) struct Margins {
    pub(crate) top: i32,
    pub(crate) bottom: i32,
    pub(crate) left: i32,
    pub(crate) right: i32,
}

pub(crate) fn get_margins(margins: Option<ChartMargins>) -> Result<Margins, RendererError> {
    match margins {
        Some(margins) => {
            let mut res = Margins {
                top: 0,
                bottom: 0,
                left: 0,
                right: 0,
            };
            match margins.margin_top {
                Some(margin_top) => res.top = margin_top,
                _ => return Err(RendererError::ChartTopMarginIsNotSpecified),
            };
            match margins.margin_bottom {
                Some(margin_bottom) => res.bottom = margin_bottom,
                _ => return Err(RendererError::ChartBottomMarginIsNotSpecified),
            };
            match margins.margin_left {
                Some(margin_left) => res.left = margin_left,
                _ => return Err(RendererError::ChartLeftMarginIsNotSpecified),
            };
            match margins.margin_right {
                Some(margin_right) => res.right = margin_right,
                _ => return Err(RendererError::ChartRightMarginIsNotSpecified),
            };

            Ok(res)
        }
        _ => Err(RendererError::ChartMarginsAreNotSpecified),
    }
}

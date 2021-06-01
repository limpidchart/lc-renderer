use crate::error::RendererError;
use crate::proto::render::ChartSizes;

pub(crate) struct Sizes {
    pub(crate) width: i32,
    pub(crate) height: i32,
}

pub(crate) fn get_sizes(sizes: Option<ChartSizes>) -> Result<Sizes, RendererError> {
    match sizes {
        Some(sizes) => {
            let mut res = Sizes {
                width: 0,
                height: 0,
            };
            match sizes.width {
                Some(width) => res.width = width,
                _ => return Err(RendererError::ChartWidthIsNotSpecified),
            };
            match sizes.height {
                Some(height) => res.height = height,
                _ => return Err(RendererError::ChartHeightIsNotSpecified),
            };

            Ok(res)
        }
        _ => Err(RendererError::ChartSizesAreNotSpecified),
    }
}

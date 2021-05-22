/// Renderer error return type.
#[derive(Debug)]
pub enum RendererError {
    /// RenderError contains chart rendering error from rendering lib.
    RenderError(lc_render::Error),

    /// View colors are not specified.
    ViewColorsAreNotSpecified,

    /// View values are not specified.
    ViewValuesAreNotSpecified,

    /// Expected scalar values but got other kind.
    ExpectedScalarValues,

    /// Expected bars values but got other kind.
    ExpectedBarsValues,

    /// Expected points values but got other kind.
    ExpectedPointsValues,

    /// Bars values fill color is not provided.
    ExpectedFillColorForBarsValues,

    /// Bars values stroke color is not provided.
    ExpectedStrokeColorForBarsValues,

    /// Got unknown bar label position.
    BarLabelPositionIsUnknown,

    /// Got unknown point label position.
    PointLabelPositionIsUnknown,

    /// Got unknown point type.
    PointTypeIsUnknown,

    /// Top or bottom axis should be specified.
    TopOrBottomAxisShouldBeSpecified,

    /// Left or right axis should be specified.
    LeftOrRightAxisShouldBeSpecified,

    /// Expected configured fill color for area view.
    ExpectedFillColorForAreaView,

    /// Expected configured stroke color for area view.
    ExpectedStrokeColorForAreaView,

    /// Expected configured point fill color for area view.
    ExpectedPointFillColorForAreaView,

    /// Expected configured point stroke color for area view.
    ExpectedPointStrokeColorForAreaView,

    /// Expected configured stroke color for line view.
    ExpectedStrokeColorForLineView,

    /// Expected configured point fill color for line view.
    ExpectedPointFillColorForLineView,

    /// Expected configured point stroke color for line view.
    ExpectedPointStrokeColorForLineView,

    /// Expected configured point fill color for scatter view.
    ExpectedPointFillColorForScatterView,

    /// Expected configured point stroke color for scatter view.
    ExpectedPointStrokeColorForScatterView,

    /// View kind is unknown.
    ViewKindIsUnknown,
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RendererError::RenderError(e) => {
                format!("unable to render chart: {}", e.to_string()).fmt(f)
            }
            RendererError::ViewColorsAreNotSpecified => {
                "view colors should be specified".to_string().fmt(f)
            }
            RendererError::ViewValuesAreNotSpecified => {
                "view values should be specified".to_string().fmt(f)
            }
            RendererError::ExpectedScalarValues => {
                "expected scalar values for view".to_string().fmt(f)
            }
            RendererError::ExpectedBarsValues => "expected bars values for view".to_string().fmt(f),
            RendererError::ExpectedPointsValues => {
                "expected points values for view".to_string().fmt(f)
            }
            RendererError::ExpectedFillColorForBarsValues => {
                "expected fill color for bars values".to_string().fmt(f)
            }
            RendererError::ExpectedStrokeColorForBarsValues => {
                "expected stroke color for bars values".to_string().fmt(f)
            }
            RendererError::BarLabelPositionIsUnknown => {
                "view bar label position is unknown".to_string().fmt(f)
            }
            RendererError::PointLabelPositionIsUnknown => {
                "view point label position is unknown".to_string().fmt(f)
            }
            RendererError::PointTypeIsUnknown => "view point type is unknown".to_string().fmt(f),
            RendererError::TopOrBottomAxisShouldBeSpecified => {
                "top or bottom axis should be specified".to_string().fmt(f)
            }
            RendererError::LeftOrRightAxisShouldBeSpecified => {
                "left or right axis should be specified".to_string().fmt(f)
            }
            RendererError::ExpectedFillColorForAreaView => {
                "expected fill color for area view".to_string().fmt(f)
            }
            RendererError::ExpectedStrokeColorForAreaView => {
                "expected stroke color for area view".to_string().fmt(f)
            }
            RendererError::ExpectedPointFillColorForAreaView => {
                "expected point fill color for area view".to_string().fmt(f)
            }
            RendererError::ExpectedPointStrokeColorForAreaView => {
                "expected point stroke color for area view"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ExpectedStrokeColorForLineView => {
                "expected stroke color for line view".to_string().fmt(f)
            }
            RendererError::ExpectedPointFillColorForLineView => {
                "expected point fill color for line view".to_string().fmt(f)
            }
            RendererError::ExpectedPointStrokeColorForLineView => {
                "expected point stroke color for line view"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ExpectedPointFillColorForScatterView => {
                "expected point fill color for scatter view"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ExpectedPointStrokeColorForScatterView => {
                "expected point stroke color for scatter view"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ViewKindIsUnknown => "view kind is unknown".to_string().fmt(f),
        }
    }
}

impl std::error::Error for RendererError {}

impl std::convert::From<lc_render::Error> for RendererError {
    fn from(e: lc_render::Error) -> Self {
        RendererError::RenderError(e)
    }
}

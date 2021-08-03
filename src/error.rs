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

    /// Bars values are not specified.
    ColorsForBarsValuesAreNotSpecified,

    /// Bars values fill color is not specified.
    FillColorForBarsValuesIsNotSpecified,

    /// Bars values stroke color is not specified.
    StrokeColorForBarsValuesIsNotSpecified,

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

    /// Fill color for area view is not specified.
    FillColorForAreaViewIsNotSpecified,

    /// Stroke color for area view is not specified.
    StrokeColorForAreaViewIsNotSpecified,

    /// Point fill color for area view is not specified.
    PointFillColorForAreaViewIsNotSpecified,

    /// Point stroke color for area view is not specified.
    PointStrokeColorForAreaViewIsNotSpecified,

    /// Stroke color for line view is not specified.
    StrokeColorForLineViewIsNotSpecified,

    /// Point fill color for line view is not specified.
    PointFillColorForLineViewIsNotSpecified,

    /// Point stroke color for line view is not specified.
    PointStrokeColorForLineViewIsNotSpecified,

    /// Point fill color for scatter view is not specified.
    PointFillColorForScatterViewIsNotSpecified,

    /// Point stroke color for scatter view is not specified.
    PointStrokeColorForScatterViewIsNotSpecified,

    /// View kind is unknown.
    ViewKindIsUnknown,

    /// Chart axes are not specified.
    ChartAxesAreNotSpecified,

    /// Chart sizes are not specified.
    ChartSizesAreNotSpecified,

    /// Chart width is not specified.
    ChartWidthIsNotSpecified,

    /// Chart height is not specified.
    ChartHeightIsNotSpecified,

    /// Chart margins are not specified.
    ChartMarginsAreNotSpecified,

    /// Chart top margin is not specified.
    ChartTopMarginIsNotSpecified,

    /// Chart bottom margin is not specified.
    ChartBottomMarginIsNotSpecified,

    /// Chart left margin is not specified.
    ChartLeftMarginIsNotSpecified,

    /// Chart right margin is not specified.
    ChartRightMarginIsNotSpecified,

    /// Top axis is set but it's not of band or linear kind
    TopAxisIsSetButItsNotBandOrLinear,

    /// Bottom axis is set but it's not of band or linear kind
    BottomAxisIsSetButItsNotBandOrLinear,

    /// Left axis is set but it's not of band or linear kind
    LeftAxisIsSetButItsNotBandOrLinear,

    /// Right axis is set but it's not of band or linear kind
    RightAxisIsSetButItsNotBandOrLinear,

    /// Point visibility for area view is not specified.
    PointVisibilityForAreaViewIsNotSpecified,

    /// Point label visibility for area view is not specified.
    PointLabelVisibilityForAreaViewIsNotSpecified,

    /// Point visibility for line view is not specified.
    PointVisibilityForLineViewIsNotSpecified,

    /// Point label visibility for line view is not specified.
    PointLabelVisibilityForLineViewIsNotSpecified,

    /// Point visibility for scatter view is not specified.
    PointVisibilityForScatterViewIsNotSpecified,

    /// Point label visibility for scatter view is not specified.
    PointLabelVisibilityForScatterViewIsNotSpecified,

    /// Bar label visibility for horizontal bar is not specified.
    BarLabelVisibilityForHorizontalBarViewIsNotSpecified,

    /// Bar label visibility for vertical bar is not specified.
    BarLabelVisibilityForVerticalBarViewIsNotSpecified,

    /// Range start for scale is not specified.
    ScaleRangeStartIsNotSpecified,

    /// Range end for scale is not specified.
    ScaleRangeEndIsNotSpecified,

    /// Domain for scale is not specified.
    ScaleDomainIsNotSpecified,

    /// Linear scale numeric domain is not specified.
    LinearScaleNumericDomainIsNotSpecified,

    /// Band scale categories domain is not specified.
    BandScaleCategoriesDomainIsNotSpecified,

    /// Band scale inner padding is not specified.
    BandScaleInnerPaddingIsNotSpecified,

    /// Band scale outer padding is not specified.
    BandScaleOuterPaddingIsNotSpecified,
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RendererError::RenderError(e) => e.fmt(f),
            RendererError::ViewColorsAreNotSpecified => {
                "view colors are not specified".to_string().fmt(f)
            }
            RendererError::ViewValuesAreNotSpecified => {
                "view values are not specified".to_string().fmt(f)
            }
            RendererError::ExpectedScalarValues => {
                "expected scalar values for view".to_string().fmt(f)
            }
            RendererError::ExpectedBarsValues => "expected bars values for view".to_string().fmt(f),
            RendererError::ExpectedPointsValues => {
                "expected points values for view".to_string().fmt(f)
            }
            RendererError::ColorsForBarsValuesAreNotSpecified => {
                "colors for bars values are not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::FillColorForBarsValuesIsNotSpecified => {
                "fill color for bars values is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::StrokeColorForBarsValuesIsNotSpecified => {
                "stroke color for bars values is not specified"
                    .to_string()
                    .fmt(f)
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
            RendererError::FillColorForAreaViewIsNotSpecified => {
                "fill color for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::StrokeColorForAreaViewIsNotSpecified => {
                "stroke color for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointFillColorForAreaViewIsNotSpecified => {
                "point fill color for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointStrokeColorForAreaViewIsNotSpecified => {
                "point stroke color for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::StrokeColorForLineViewIsNotSpecified => {
                "stroke color for line view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointFillColorForLineViewIsNotSpecified => {
                "point fill color for line view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointStrokeColorForLineViewIsNotSpecified => {
                "point stroke color for line view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointFillColorForScatterViewIsNotSpecified => {
                "point fill color for scatter view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointStrokeColorForScatterViewIsNotSpecified => {
                "point stroke color for scatter view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ViewKindIsUnknown => "view kind is unknown".to_string().fmt(f),
            RendererError::ChartAxesAreNotSpecified => {
                "chart axes are not be specified".to_string().fmt(f)
            }
            RendererError::ChartSizesAreNotSpecified => {
                "chart sizes are not specified".to_string().fmt(f)
            }
            RendererError::ChartWidthIsNotSpecified => {
                "chart width is not specified".to_string().fmt(f)
            }
            RendererError::ChartHeightIsNotSpecified => {
                "chart height is not specified".to_string().fmt(f)
            }
            RendererError::ChartMarginsAreNotSpecified => {
                "chart margins are not specified".to_string().fmt(f)
            }
            RendererError::ChartTopMarginIsNotSpecified => {
                "chart top margin is not specified".to_string().fmt(f)
            }
            RendererError::ChartBottomMarginIsNotSpecified => {
                "chart bottom margin is not specified".to_string().fmt(f)
            }
            RendererError::ChartLeftMarginIsNotSpecified => {
                "chart left margin is not specified".to_string().fmt(f)
            }
            RendererError::ChartRightMarginIsNotSpecified => {
                "chart right margin is not specified".to_string().fmt(f)
            }
            RendererError::TopAxisIsSetButItsNotBandOrLinear => {
                "top axis is set but it's not of band or linear kind"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BottomAxisIsSetButItsNotBandOrLinear => {
                "bottom axis is set but it's not of band or linear kind"
                    .to_string()
                    .fmt(f)
            }
            RendererError::LeftAxisIsSetButItsNotBandOrLinear => {
                "left axis is set but it's not of band or linear kind"
                    .to_string()
                    .fmt(f)
            }
            RendererError::RightAxisIsSetButItsNotBandOrLinear => {
                "right axis is set but it's not of band or linear kind"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointVisibilityForAreaViewIsNotSpecified => {
                "point visibility for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointLabelVisibilityForAreaViewIsNotSpecified => {
                "point label visibility for area view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointVisibilityForLineViewIsNotSpecified => {
                "point visibility for line view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointLabelVisibilityForLineViewIsNotSpecified => {
                "point label visibility for line view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointVisibilityForScatterViewIsNotSpecified => {
                "point visibility for scatter view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::PointLabelVisibilityForScatterViewIsNotSpecified => {
                "point label visibility for scatter view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BarLabelVisibilityForHorizontalBarViewIsNotSpecified => {
                "bar label visibility for horizontal bar view is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BarLabelVisibilityForVerticalBarViewIsNotSpecified => {
                "bar label visibility for vertical bar is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::ScaleRangeStartIsNotSpecified => {
                "range start for scale is not specified".to_string().fmt(f)
            }
            RendererError::ScaleRangeEndIsNotSpecified => {
                "range end for scale is not specified".to_string().fmt(f)
            }
            RendererError::ScaleDomainIsNotSpecified => {
                "domain for scale is not specified".to_string().fmt(f)
            }
            RendererError::LinearScaleNumericDomainIsNotSpecified => {
                "numeric domain for linear scale is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BandScaleCategoriesDomainIsNotSpecified => {
                "categories domain for band scale is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BandScaleInnerPaddingIsNotSpecified => {
                "inner padding for band scale is not specified"
                    .to_string()
                    .fmt(f)
            }
            RendererError::BandScaleOuterPaddingIsNotSpecified => {
                "outer padding for band scale is not specified"
                    .to_string()
                    .fmt(f)
            }
        }
    }
}

impl std::error::Error for RendererError {}

impl std::convert::From<lc_render::Error> for RendererError {
    fn from(e: lc_render::Error) -> Self {
        RendererError::RenderError(e)
    }
}

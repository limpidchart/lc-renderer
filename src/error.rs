/// Renderer error return type.
#[derive(Debug)]
pub enum RendererError {
    /// RenderError contains chart rendering error from rendering lib.
    RenderError(lc_render::Error),

    /// Expected view colors but they are not specified.
    ViewColorsAreNotSpecified,
}

impl std::fmt::Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &*self {
            RendererError::RenderError(e) => {
                format!("unable to render chart: {}", e.to_string()).fmt(f)
            }
            RendererError::ViewColorsAreNotSpecified => {
                "chart view colors should be specified".to_string().fmt(f)
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

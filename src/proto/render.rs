/// ChartScale represents options to configure chart scale.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartScale {
    /// One of the available scale kinds.
    #[prost(enumeration = "chart_scale::ChartScaleKind", tag = "1")]
    pub kind: i32,
    /// Start of the scale range.
    #[prost(message, optional, tag = "2")]
    pub range_start: ::core::option::Option<i32>,
    /// End of the scale range.
    #[prost(message, optional, tag = "3")]
    pub range_end: ::core::option::Option<i32>,
    /// Does this scale needs an offset from the start and end of an axis.
    /// This is usually need for an area or line views.
    #[prost(bool, tag = "7")]
    pub no_boundaries_offset: bool,
    /// Inner padding for categories.
    #[prost(message, optional, tag = "8")]
    pub inner_padding: ::core::option::Option<f32>,
    /// Outer padding for categories.
    #[prost(message, optional, tag = "9")]
    pub outer_padding: ::core::option::Option<f32>,
    /// Scale domain with one of available kind.
    #[prost(oneof = "chart_scale::Domain", tags = "4, 5")]
    pub domain: ::core::option::Option<chart_scale::Domain>,
}
/// Nested message and enum types in `ChartScale`.
pub mod chart_scale {
    /// ChartScaleKind contains available scale kinds.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChartScaleKind {
        UnspecifiedScale = 0,
        Linear = 1,
        Band = 2,
    }
    /// Scale domain with one of available kind.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Domain {
        /// Numeric scale domain.
        #[prost(message, tag = "4")]
        DomainNumeric(super::DomainNumeric),
        /// String scale domain categories.
        #[prost(message, tag = "5")]
        DomainCategories(super::DomainCategories),
    }
}
/// DomainNumeric represents numeric scale domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainNumeric {
    /// Start of the numeric scale domain.
    #[prost(float, tag = "1")]
    pub start: f32,
    /// End of the numeric scale domain.
    #[prost(float, tag = "2")]
    pub end: f32,
}
/// DomainCategories represents string categorical scale domain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DomainCategories {
    #[prost(string, repeated, tag = "1")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ChartSizes represents options to configure chart sizes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartSizes {
    /// Chart width.
    #[prost(message, optional, tag = "1")]
    pub width: ::core::option::Option<i32>,
    /// Chart height.
    #[prost(message, optional, tag = "2")]
    pub height: ::core::option::Option<i32>,
}
/// ChartMargins represents options to configure chart margins.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartMargins {
    /// Top margin.
    #[prost(message, optional, tag = "1")]
    pub margin_top: ::core::option::Option<i32>,
    /// Bottom margin.
    #[prost(message, optional, tag = "2")]
    pub margin_bottom: ::core::option::Option<i32>,
    /// Left margin.
    #[prost(message, optional, tag = "3")]
    pub margin_left: ::core::option::Option<i32>,
    /// Right margin.
    #[prost(message, optional, tag = "4")]
    pub margin_right: ::core::option::Option<i32>,
}
/// ChartAxes represents options to configure chart axes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartAxes {
    /// Configured scale for top axis.
    #[prost(message, optional, tag = "1")]
    pub axis_top: ::core::option::Option<ChartScale>,
    /// Label for top axis.
    #[prost(string, tag = "2")]
    pub axis_top_label: ::prost::alloc::string::String,
    /// Configured scale for bottom axis.
    #[prost(message, optional, tag = "3")]
    pub axis_bottom: ::core::option::Option<ChartScale>,
    /// Label for bottom axis.
    #[prost(string, tag = "4")]
    pub axis_bottom_label: ::prost::alloc::string::String,
    /// Configured scale for left axis.
    #[prost(message, optional, tag = "5")]
    pub axis_left: ::core::option::Option<ChartScale>,
    /// Label for left axis.
    #[prost(string, tag = "6")]
    pub axis_left_label: ::prost::alloc::string::String,
    /// Configured scale for right axis.
    #[prost(message, optional, tag = "7")]
    pub axis_right: ::core::option::Option<ChartScale>,
    /// Label for right axis.
    #[prost(string, tag = "8")]
    pub axis_right_label: ::prost::alloc::string::String,
}
/// ChartElementColor represents options to configure color for chart elements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartElementColor {
    /// Value of color.
    /// Can contain HEX or RGB value.
    #[prost(oneof = "chart_element_color::ColorValue", tags = "1, 2")]
    pub color_value: ::core::option::Option<chart_element_color::ColorValue>,
}
/// Nested message and enum types in `ChartElementColor`.
pub mod chart_element_color {
    /// RGB contains values for RGB color.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Rgb {
        #[prost(uint32, tag = "1")]
        pub r: u32,
        #[prost(uint32, tag = "2")]
        pub g: u32,
        #[prost(uint32, tag = "3")]
        pub b: u32,
    }
    /// Value of color.
    /// Can contain HEX or RGB value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ColorValue {
        #[prost(string, tag = "1")]
        ColorHex(::prost::alloc::string::String),
        #[prost(message, tag = "2")]
        ColorRgb(Rgb),
    }
}
/// ChartViewBarsValues represents options for bar values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartViewBarsValues {
    /// Array of configured bars datasets.
    #[prost(message, repeated, tag = "1")]
    pub bars_datasets: ::prost::alloc::vec::Vec<chart_view_bars_values::BarsDataset>,
}
/// Nested message and enum types in `ChartViewBarsValues`.
pub mod chart_view_bars_values {
    /// BarsDataset represents a single dataset with several bars.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BarsDataset {
        #[prost(float, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<f32>,
        #[prost(message, optional, tag = "2")]
        pub fill_color: ::core::option::Option<super::ChartElementColor>,
        #[prost(message, optional, tag = "3")]
        pub stroke_color: ::core::option::Option<super::ChartElementColor>,
    }
}
/// ChartViewPointsValues represents options for point values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartViewPointsValues {
    /// Array of configured points.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<chart_view_points_values::Point>,
}
/// Nested message and enum types in `ChartViewPointsValues`.
pub mod chart_view_points_values {
    /// Point represents parameters for a single point.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        #[prost(float, tag = "1")]
        pub x: f32,
        #[prost(float, tag = "2")]
        pub y: f32,
    }
}
/// ChartViewScalarValues represents options for scalar values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartViewScalarValues {
    /// Array of scalar values.
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
/// ChartView represents options to configure chart view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartView {
    /// One of the available view kinds.
    #[prost(enumeration = "chart_view::ChartViewKind", tag = "1")]
    pub kind: i32,
    /// Configured colors for view.
    #[prost(message, optional, tag = "5")]
    pub colors: ::core::option::Option<ChartViewColors>,
    /// Set bar label visibility for view with bars.
    #[prost(message, optional, tag = "6")]
    pub bar_label_visible: ::core::option::Option<bool>,
    /// One of the available bar label positions for view with bars.
    #[prost(enumeration = "chart_view::ChartViewBarLabelPosition", tag = "7")]
    pub bar_label_position: i32,
    /// Set point visibility for view with points.
    #[prost(message, optional, tag = "8")]
    pub point_visible: ::core::option::Option<bool>,
    /// One of the available point types for view with points.
    #[prost(enumeration = "chart_view::ChartViewPointType", tag = "9")]
    pub point_type: i32,
    /// Set point label visibility for view with points.
    #[prost(message, optional, tag = "10")]
    pub point_label_visible: ::core::option::Option<bool>,
    /// One of the available point label positions for view with points.
    #[prost(enumeration = "chart_view::ChartViewPointLabelPosition", tag = "11")]
    pub point_label_position: i32,
    /// View values with one of available kind of values.
    #[prost(oneof = "chart_view::Values", tags = "2, 3, 4")]
    pub values: ::core::option::Option<chart_view::Values>,
}
/// Nested message and enum types in `ChartView`.
pub mod chart_view {
    /// ChartViewKind contains available view kinds.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChartViewKind {
        UnspecifiedKind = 0,
        Area = 1,
        HorizontalBar = 2,
        Line = 3,
        Scatter = 4,
        VerticalBar = 5,
    }
    /// ChartViewBarLabelPosition contains available view label positions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChartViewBarLabelPosition {
        UnspecifiedBarLabelPosition = 0,
        StartOutside = 1,
        StartInside = 2,
        Center = 3,
        EndInside = 4,
        EndOutside = 5,
    }
    /// ChartViewPointType contains available view point types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChartViewPointType {
        UnspecifiedPointType = 0,
        Circle = 1,
        Square = 2,
        X = 3,
    }
    /// ChartViewPointLabelPosition contains available view point label positions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChartViewPointLabelPosition {
        UnspecifiedPointLabelPosition = 0,
        Top = 1,
        TopRight = 2,
        TopLeft = 3,
        Left = 4,
        Right = 5,
        Bottom = 6,
        BottomLeft = 7,
        BottomRight = 8,
    }
    /// View values with one of available kind of values.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        #[prost(message, tag = "2")]
        ScalarValues(super::ChartViewScalarValues),
        #[prost(message, tag = "3")]
        PointsValues(super::ChartViewPointsValues),
        #[prost(message, tag = "4")]
        BarsValues(super::ChartViewBarsValues),
    }
}
/// ChartViewColors represents options to configure view colors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartViewColors {
    /// View fill color.
    #[prost(message, optional, tag = "1")]
    pub fill_color: ::core::option::Option<ChartElementColor>,
    /// View stroke color.
    #[prost(message, optional, tag = "2")]
    pub stroke_color: ::core::option::Option<ChartElementColor>,
    /// View point fill color.
    #[prost(message, optional, tag = "3")]
    pub point_fill_color: ::core::option::Option<ChartElementColor>,
    /// View point stroke color.
    #[prost(message, optional, tag = "4")]
    pub point_stroke_color: ::core::option::Option<ChartElementColor>,
}
/// RenderChartRequest represents chart rendering request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderChartRequest {
    /// ID of the request.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Chart title.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Configured chart sizes.
    #[prost(message, optional, tag = "3")]
    pub sizes: ::core::option::Option<ChartSizes>,
    /// Configured chart margins.
    #[prost(message, optional, tag = "4")]
    pub margins: ::core::option::Option<ChartMargins>,
    /// Configured chart axes.
    #[prost(message, optional, tag = "5")]
    pub axes: ::core::option::Option<ChartAxes>,
    /// Configured chart views.
    #[prost(message, repeated, tag = "6")]
    pub views: ::prost::alloc::vec::Vec<ChartView>,
}
/// RenderChartReply represents chart rendering reply.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderChartReply {
    /// ID of the request.
    #[prost(string, tag = "1")]
    pub request_id: ::prost::alloc::string::String,
    /// Chart raw bytes representation.
    #[prost(bytes = "vec", tag = "2")]
    pub chart_data: ::prost::alloc::vec::Vec<u8>,
}
#[doc = r" Generated client implementations."]
pub mod chart_renderer_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " ChartRenderer represents a service that uses lc-render library to create charts."]
    pub struct ChartRendererClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ChartRendererClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ChartRendererClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Render chart and return its raw bytes representation."]
        pub async fn render_chart(
            &mut self,
            request: impl tonic::IntoRequest<super::RenderChartRequest>,
        ) -> Result<tonic::Response<super::RenderChartReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/render.ChartRenderer/RenderChart");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ChartRendererClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ChartRendererClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ChartRendererClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod chart_renderer_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ChartRendererServer."]
    #[async_trait]
    pub trait ChartRenderer: Send + Sync + 'static {
        #[doc = " Render chart and return its raw bytes representation."]
        async fn render_chart(
            &self,
            request: tonic::Request<super::RenderChartRequest>,
        ) -> Result<tonic::Response<super::RenderChartReply>, tonic::Status>;
    }
    #[doc = " ChartRenderer represents a service that uses lc-render library to create charts."]
    #[derive(Debug)]
    pub struct ChartRendererServer<T: ChartRenderer> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ChartRenderer> ChartRendererServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ChartRendererServer<T>
    where
        T: ChartRenderer,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/render.ChartRenderer/RenderChart" => {
                    #[allow(non_camel_case_types)]
                    struct RenderChartSvc<T: ChartRenderer>(pub Arc<T>);
                    impl<T: ChartRenderer> tonic::server::UnaryService<super::RenderChartRequest>
                        for RenderChartSvc<T>
                    {
                        type Response = super::RenderChartReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RenderChartRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).render_chart(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RenderChartSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ChartRenderer> Clone for ChartRendererServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ChartRenderer> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ChartRenderer> tonic::transport::NamedService for ChartRendererServer<T> {
        const NAME: &'static str = "render.ChartRenderer";
    }
}

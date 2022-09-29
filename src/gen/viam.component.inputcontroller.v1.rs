// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControlsRequest {
    /// Name of an input controller
    #[prost(string, tag="1")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetControlsResponse {
    /// Returns a list of all the controls (buttons and axes) that are
    /// available to a given Input Controller
    #[prost(string, repeated, tag="1")]
    pub controls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsRequest {
    /// Name of an input controller
    #[prost(string, tag="1")]
    pub controller: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventsResponse {
    /// Returns a list of the most recent event for each control on a given InputController. Effectively provides the current "state" of all
    /// buttons/axes on a given input controller
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerEventRequest {
    /// Name of an input controller
    #[prost(string, tag="1")]
    pub controller: ::prost::alloc::string::String,
    /// Digitally assert a given event
    #[prost(message, optional, tag="2")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TriggerEventResponse {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// Timestamp of event
    #[prost(message, optional, tag="1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// An event type (eg: ButtonPress, ButtonRelease)
    #[prost(string, tag="2")]
    pub event: ::prost::alloc::string::String,
    /// A control, can be a button (eg: ButtonSouth) or an axis (eg: AbsoluteX)
    #[prost(string, tag="3")]
    pub control: ::prost::alloc::string::String,
    /// 0 or 1 for buttons, -1.0 to +1.0 for axes
    #[prost(double, tag="4")]
    pub value: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsRequest {
    /// Name of an input controller
    #[prost(string, tag="1")]
    pub controller: ::prost::alloc::string::String,
    /// A list of Events
    #[prost(message, repeated, tag="2")]
    pub events: ::prost::alloc::vec::Vec<stream_events_request::Events>,
}
/// Nested message and enum types in `StreamEventsRequest`.
pub mod stream_events_request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Events {
        /// Name of a control (button or axis)
        #[prost(string, tag="1")]
        pub control: ::prost::alloc::string::String,
        /// Specify which event types to recieve events for
        /// To Do (FA): Right now this can be an empty list, but we should error in this case as opening a stream with no messages is expensive
        #[prost(string, repeated, tag="2")]
        pub events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Specify which event types to stop recieving events for
        /// This can be an empty list
        #[prost(string, repeated, tag="3")]
        pub cancelled_events: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamEventsResponse {
    /// Event for a controller
    #[prost(message, optional, tag="1")]
    pub event: ::core::option::Option<Event>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
}
/// Encoded file descriptor set for the `viam.component.inputcontroller.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb2, 0x29, 0x0a, 0x33, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x69,
    0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2f, 0x76,
    0x31, 0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x21, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63,
    0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1f, 0x67, 0x6f, 0x6f, 0x67, 0x6c,
    0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x34, 0x0a, 0x12, 0x47, 0x65,
    0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72,
    0x22, 0x31, 0x0a, 0x13, 0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x52, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x73, 0x22, 0x32, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x22, 0x55, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x40, 0x0a, 0x06,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e,
    0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31,
    0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x22, 0x75,
    0x0a, 0x13, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x12, 0x3e, 0x0a, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x05,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x16, 0x0a, 0x14, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x7d, 0x0a,
    0x05, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x2e, 0x0a, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x52, 0x04, 0x74, 0x69, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x18, 0x0a, 0x07,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xf3, 0x01, 0x0a,
    0x13, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x1e, 0x0a, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0a, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x6c, 0x65, 0x72, 0x12, 0x55, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x3d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x45, 0x76, 0x65,
    0x6e, 0x74, 0x73, 0x52, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x1a, 0x65, 0x0a, 0x06, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x12,
    0x16, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x52,
    0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x29, 0x0a, 0x10, 0x63, 0x61, 0x6e, 0x63, 0x65,
    0x6c, 0x6c, 0x65, 0x64, 0x5f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x0f, 0x63, 0x61, 0x6e, 0x63, 0x65, 0x6c, 0x6c, 0x65, 0x64, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x73, 0x22, 0x56, 0x0a, 0x14, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3e, 0x0a, 0x05, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x76,
    0x65, 0x6e, 0x74, 0x52, 0x05, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x22, 0x4a, 0x0a, 0x06, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x12, 0x40, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x28, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x06,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x32, 0x85, 0x06, 0x0a, 0x16, 0x49, 0x6e, 0x70, 0x75, 0x74,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x12, 0xb8, 0x01, 0x0a, 0x0b, 0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x73, 0x12, 0x35, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x36, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74,
    0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x22, 0x3a, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x34, 0x12, 0x32, 0x2f, 0x76, 0x69, 0x61, 0x6d, 0x2f,
    0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2f, 0x7b, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x7d, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x12, 0xb0, 0x01, 0x0a,
    0x09, 0x47, 0x65, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x33, 0x2e, 0x76, 0x69, 0x61,
    0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x47,
    0x65, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a,
    0x34, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74,
    0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72,
    0x2e, 0x76, 0x31, 0x2e, 0x47, 0x65, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x38, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x32, 0x12, 0x30, 0x2f,
    0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2f, 0x7b, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x7d, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12,
    0xc1, 0x01, 0x0a, 0x0c, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x12, 0x36, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e,
    0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65,
    0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x37, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e,
    0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x74, 0x72,
    0x65, 0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x3e, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x38, 0x12, 0x36, 0x2f, 0x76, 0x69, 0x61, 0x6d,
    0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e,
    0x74, 0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2f, 0x7b, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x72, 0x7d, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x74, 0x72, 0x65, 0x61,
    0x6d, 0x30, 0x01, 0x12, 0xb8, 0x01, 0x0a, 0x0c, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x12, 0x36, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x1a, 0x37, 0x2e, 0x76,
    0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e,
    0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31,
    0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x37, 0x82, 0xd3, 0xe4, 0x93, 0x02, 0x31, 0x22, 0x2f, 0x2f,
    0x76, 0x69, 0x61, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x76, 0x31, 0x2f, 0x63, 0x6f, 0x6d, 0x70,
    0x6f, 0x6e, 0x65, 0x6e, 0x74, 0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x2f, 0x7b, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x7d, 0x2f, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x42, 0x55,
    0x0a, 0x25, 0x63, 0x6f, 0x6d, 0x2e, 0x76, 0x69, 0x61, 0x6d, 0x2e, 0x63, 0x6f, 0x6d, 0x70, 0x6f,
    0x6e, 0x65, 0x6e, 0x74, 0x2e, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x76, 0x31, 0x5a, 0x2c, 0x67, 0x6f, 0x2e, 0x76, 0x69, 0x61, 0x6d,
    0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x61, 0x70, 0x69, 0x2f, 0x63, 0x6f, 0x6d, 0x70, 0x6f, 0x6e, 0x65,
    0x6e, 0x74, 0x2f, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x2f, 0x76, 0x31, 0x4a, 0x95, 0x1a, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x6c, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x02, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x26, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x29, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x07, 0x00, 0x43, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x07, 0x00, 0x43, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x3e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x08, 0x00, 0x3e, 0x0a, 0x5f, 0x0a, 0x02, 0x06, 0x00, 0x12, 0x04, 0x0b, 0x00, 0x28, 0x01,
    0x1a, 0x53, 0x20, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c,
    0x65, 0x72, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63,
    0x65, 0x73, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x69, 0x6e, 0x70, 0x75,
    0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x61, 0x73, 0x73,
    0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x72,
    0x6f, 0x62, 0x6f, 0x74, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08,
    0x1e, 0x0a, 0x54, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x00, 0x12, 0x04, 0x0d, 0x02, 0x11, 0x03, 0x1a,
    0x46, 0x20, 0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x47, 0x65, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x76,
    0x69, 0x64, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x74,
    0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0d, 0x06, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x0d, 0x12, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x2f,
    0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x00, 0x04, 0x12, 0x04, 0x0e, 0x04, 0x10, 0x06,
    0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x00, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x0e,
    0x04, 0x10, 0x06, 0x0a, 0x79, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x01, 0x12, 0x04, 0x14, 0x02, 0x18,
    0x03, 0x1a, 0x6b, 0x20, 0x47, 0x65, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x72, 0x65,
    0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x73, 0x74, 0x20, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x20, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x06, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x14, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x2b, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x15, 0x04, 0x17, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00, 0x02, 0x01, 0x04,
    0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x15, 0x04, 0x17, 0x06, 0x0a, 0x89, 0x01, 0x0a, 0x04, 0x06,
    0x00, 0x02, 0x02, 0x12, 0x04, 0x1b, 0x02, 0x1f, 0x03, 0x1a, 0x7b, 0x20, 0x53, 0x74, 0x72, 0x65,
    0x61, 0x6d, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x73, 0x20,
    0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x6f, 0x66, 0x20, 0x49, 0x6e, 0x70, 0x75,
    0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x28, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e,
    0x73, 0x2f, 0x61, 0x78, 0x65, 0x73, 0x29, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x72, 0x6f, 0x62,
    0x6f, 0x74, 0x27, 0x73, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1b, 0x06, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1b,
    0x13, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x31, 0x37,
    0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1b, 0x38, 0x4c, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1c, 0x04, 0x1e, 0x06, 0x0a, 0x11, 0x0a,
    0x09, 0x06, 0x00, 0x02, 0x02, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x1c, 0x04, 0x1e, 0x06,
    0x0a, 0xae, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x04, 0x23, 0x02, 0x27, 0x03, 0x1a,
    0x9f, 0x01, 0x20, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2c,
    0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64,
    0x2c, 0x20, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x28, 0x76,
    0x69, 0x72, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x29, 0x20, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61,
    0x74, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20,
    0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x20, 0x6f,
    0x72, 0x20, 0x61, 0x78, 0x69, 0x73, 0x20, 0x6d, 0x6f, 0x76, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23, 0x06, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x23, 0x13, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x23, 0x31, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x24, 0x04, 0x26, 0x06, 0x0a, 0x11, 0x0a, 0x09, 0x06, 0x00,
    0x02, 0x03, 0x04, 0xb0, 0xca, 0xbc, 0x22, 0x12, 0x04, 0x24, 0x04, 0x26, 0x06, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x2a, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x2a, 0x08, 0x1a, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2c,
    0x02, 0x18, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20,
    0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x09, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x2f, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x2f, 0x08, 0x1b, 0x0a, 0x74, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x1f,
    0x1a, 0x67, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x73, 0x20, 0x28, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x73, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x61, 0x78, 0x65, 0x73, 0x29, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x72,
    0x65, 0x0a, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x49, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x43, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x32, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1d,
    0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x35, 0x08, 0x18, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x37, 0x02, 0x18, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x37, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x37, 0x09,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x37, 0x16, 0x17, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3a, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x19, 0x0a, 0xbe, 0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00,
    0x12, 0x03, 0x3d, 0x02, 0x1c, 0x1a, 0xb0, 0x01, 0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73,
    0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x6f, 0x73, 0x74, 0x20, 0x72, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f,
    0x6c, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x49, 0x6e, 0x70,
    0x75, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x2e, 0x20, 0x45, 0x66,
    0x66, 0x65, 0x63, 0x74, 0x69, 0x76, 0x65, 0x6c, 0x79, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x22,
    0x73, 0x74, 0x61, 0x74, 0x65, 0x22, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x0a, 0x20, 0x62,
    0x75, 0x74, 0x74, 0x6f, 0x6e, 0x73, 0x2f, 0x61, 0x78, 0x65, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x61,
    0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x3d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x3d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x1a, 0x1b,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x40, 0x00, 0x45, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x04, 0x01, 0x12, 0x03, 0x40, 0x08, 0x1b, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00,
    0x12, 0x03, 0x42, 0x02, 0x18, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x6e, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x42,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x42, 0x09, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x42, 0x16, 0x17, 0x0a, 0x2d,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x44, 0x02, 0x12, 0x1a, 0x20, 0x20, 0x44, 0x69,
    0x67, 0x69, 0x74, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x61, 0x73, 0x73, 0x65, 0x72, 0x74, 0x20, 0x61,
    0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x44, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x44, 0x10, 0x11, 0x0a, 0x09, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x03, 0x47,
    0x00, 0x1f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x47, 0x08, 0x1c, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x49, 0x00, 0x52, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06,
    0x01, 0x12, 0x03, 0x49, 0x08, 0x0d, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03,
    0x4b, 0x02, 0x25, 0x1a, 0x14, 0x20, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20,
    0x6f, 0x66, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x4b, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4b, 0x1c, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4b, 0x23, 0x24, 0x0a, 0x3d, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4d, 0x02, 0x13,
    0x1a, 0x30, 0x20, 0x41, 0x6e, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65,
    0x20, 0x28, 0x65, 0x67, 0x3a, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x50, 0x72, 0x65, 0x73,
    0x73, 0x2c, 0x20, 0x42, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65,
    0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4d, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4d, 0x09, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4d, 0x11, 0x12, 0x0a, 0x56, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x02, 0x15, 0x1a, 0x49, 0x20, 0x41, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x2c, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x20,
    0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x20, 0x28, 0x65, 0x67, 0x3a, 0x20, 0x42, 0x75, 0x74, 0x74,
    0x6f, 0x6e, 0x53, 0x6f, 0x75, 0x74, 0x68, 0x29, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x61,
    0x78, 0x69, 0x73, 0x20, 0x28, 0x65, 0x67, 0x3a, 0x20, 0x41, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74,
    0x65, 0x58, 0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x09, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x13, 0x14, 0x0a, 0x38,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x51, 0x02, 0x13, 0x1a, 0x2b, 0x20, 0x30, 0x20,
    0x6f, 0x72, 0x20, 0x31, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x75, 0x74, 0x74, 0x6f, 0x6e, 0x73,
    0x2c, 0x20, 0x2d, 0x31, 0x2e, 0x30, 0x20, 0x74, 0x6f, 0x20, 0x2b, 0x31, 0x2e, 0x30, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x61, 0x78, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03,
    0x05, 0x12, 0x03, 0x51, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x51, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x51,
    0x11, 0x12, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x54, 0x00, 0x63, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x54, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07,
    0x03, 0x00, 0x12, 0x04, 0x55, 0x02, 0x5e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x03, 0x00,
    0x01, 0x12, 0x03, 0x55, 0x0a, 0x10, 0x0a, 0x33, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x57, 0x04, 0x17, 0x1a, 0x24, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x28, 0x62, 0x75, 0x74, 0x74, 0x6f,
    0x6e, 0x20, 0x6f, 0x72, 0x20, 0x61, 0x78, 0x69, 0x73, 0x29, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x0b, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x07, 0x03, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x15, 0x16, 0x0a, 0xc6, 0x01, 0x0a, 0x06,
    0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x12, 0x03, 0x5a, 0x04, 0x1f, 0x1a, 0xb6, 0x01, 0x20, 0x53,
    0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x63, 0x69,
    0x65, 0x76, 0x65, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20,
    0x54, 0x6f, 0x20, 0x44, 0x6f, 0x20, 0x28, 0x46, 0x41, 0x29, 0x3a, 0x20, 0x52, 0x69, 0x67, 0x68,
    0x74, 0x20, 0x6e, 0x6f, 0x77, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x2c,
    0x20, 0x62, 0x75, 0x74, 0x20, 0x77, 0x65, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x73,
    0x65, 0x20, 0x61, 0x73, 0x20, 0x6f, 0x70, 0x65, 0x6e, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x65, 0x6e, 0x73,
    0x69, 0x76, 0x65, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x5a, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x5a, 0x14, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x5a, 0x1d, 0x1e, 0x0a, 0x62, 0x0a, 0x06, 0x04, 0x07, 0x03, 0x00, 0x02, 0x02, 0x12, 0x03,
    0x5d, 0x04, 0x29, 0x1a, 0x53, 0x20, 0x53, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x20, 0x77, 0x68,
    0x69, 0x63, 0x68, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x20, 0x72, 0x65, 0x63, 0x69, 0x65, 0x76, 0x69, 0x6e,
    0x67, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x54, 0x68,
    0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6d, 0x70,
    0x74, 0x79, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x5d, 0x0d, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x5d, 0x14, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x03, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x27, 0x28, 0x0a, 0x2a, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00,
    0x12, 0x03, 0x60, 0x02, 0x18, 0x1a, 0x1d, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x6e, 0x20, 0x69, 0x6e, 0x70, 0x75, 0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c,
    0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x09, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x16, 0x17, 0x0a, 0x1f,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x62, 0x02, 0x1d, 0x1a, 0x12, 0x20, 0x41, 0x20,
    0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x62, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x62, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x65,
    0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x65, 0x08, 0x1c, 0x0a,
    0x25, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x67, 0x02, 0x12, 0x1a, 0x18, 0x20, 0x45,
    0x76, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x6c, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x67, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x10, 0x11,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x6b, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6b, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x11, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x1a, 0x1b, 0x62, 0x06, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("viam.component.inputcontroller.v1.tonic.rs");
// @@protoc_insertion_point(module)
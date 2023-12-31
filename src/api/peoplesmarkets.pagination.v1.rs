/// deprecate in favor of PaginationRequest/PaginationResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pagination {
    #[prost(uint64, tag = "1")]
    pub page: u64,
    #[prost(uint64, tag = "2")]
    pub size: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationRequest {
    #[prost(uint32, tag = "1")]
    pub page: u32,
    #[prost(uint32, tag = "2")]
    pub size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationResponse {
    #[prost(uint32, tag = "1")]
    pub page: u32,
    #[prost(uint32, tag = "2")]
    pub size: u32,
    #[prost(uint32, tag = "3")]
    pub total_elements: u32,
}

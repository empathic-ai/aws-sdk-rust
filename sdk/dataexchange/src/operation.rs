// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CancelJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`cancel_job`](crate::client::Client::cancel_job).
///
/// See [`crate::client::fluent_builders::CancelJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CancelJob {
    _private: (),
}
impl CancelJob {
    /// Creates a new builder-style object to manufacture [`CancelJobInput`](crate::input::CancelJobInput)
    pub fn builder() -> crate::input::cancel_job_input::Builder {
        crate::input::cancel_job_input::Builder::default()
    }
    /// Creates a new `CancelJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CancelJob {
    type Output = std::result::Result<crate::output::CancelJobOutput, crate::error::CancelJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_cancel_job_error(response)
        } else {
            crate::operation_deser::parse_cancel_job_response(response)
        }
    }
}

/// Operation shape for `CreateDataSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_data_set`](crate::client::Client::create_data_set).
///
/// See [`crate::client::fluent_builders::CreateDataSet`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateDataSet {
    _private: (),
}
impl CreateDataSet {
    /// Creates a new builder-style object to manufacture [`CreateDataSetInput`](crate::input::CreateDataSetInput)
    pub fn builder() -> crate::input::create_data_set_input::Builder {
        crate::input::create_data_set_input::Builder::default()
    }
    /// Creates a new `CreateDataSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateDataSet {
    type Output =
        std::result::Result<crate::output::CreateDataSetOutput, crate::error::CreateDataSetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_data_set_error(response)
        } else {
            crate::operation_deser::parse_create_data_set_response(response)
        }
    }
}

/// Operation shape for `CreateEventAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_event_action`](crate::client::Client::create_event_action).
///
/// See [`crate::client::fluent_builders::CreateEventAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateEventAction {
    _private: (),
}
impl CreateEventAction {
    /// Creates a new builder-style object to manufacture [`CreateEventActionInput`](crate::input::CreateEventActionInput)
    pub fn builder() -> crate::input::create_event_action_input::Builder {
        crate::input::create_event_action_input::Builder::default()
    }
    /// Creates a new `CreateEventAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateEventAction {
    type Output = std::result::Result<
        crate::output::CreateEventActionOutput,
        crate::error::CreateEventActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_event_action_error(response)
        } else {
            crate::operation_deser::parse_create_event_action_response(response)
        }
    }
}

/// Operation shape for `CreateJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_job`](crate::client::Client::create_job).
///
/// See [`crate::client::fluent_builders::CreateJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateJob {
    _private: (),
}
impl CreateJob {
    /// Creates a new builder-style object to manufacture [`CreateJobInput`](crate::input::CreateJobInput)
    pub fn builder() -> crate::input::create_job_input::Builder {
        crate::input::create_job_input::Builder::default()
    }
    /// Creates a new `CreateJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateJob {
    type Output = std::result::Result<crate::output::CreateJobOutput, crate::error::CreateJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_job_error(response)
        } else {
            crate::operation_deser::parse_create_job_response(response)
        }
    }
}

/// Operation shape for `CreateRevision`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_revision`](crate::client::Client::create_revision).
///
/// See [`crate::client::fluent_builders::CreateRevision`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CreateRevision {
    _private: (),
}
impl CreateRevision {
    /// Creates a new builder-style object to manufacture [`CreateRevisionInput`](crate::input::CreateRevisionInput)
    pub fn builder() -> crate::input::create_revision_input::Builder {
        crate::input::create_revision_input::Builder::default()
    }
    /// Creates a new `CreateRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateRevision {
    type Output =
        std::result::Result<crate::output::CreateRevisionOutput, crate::error::CreateRevisionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 201 {
            crate::operation_deser::parse_create_revision_error(response)
        } else {
            crate::operation_deser::parse_create_revision_response(response)
        }
    }
}

/// Operation shape for `DeleteAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_asset`](crate::client::Client::delete_asset).
///
/// See [`crate::client::fluent_builders::DeleteAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteAsset {
    _private: (),
}
impl DeleteAsset {
    /// Creates a new builder-style object to manufacture [`DeleteAssetInput`](crate::input::DeleteAssetInput)
    pub fn builder() -> crate::input::delete_asset_input::Builder {
        crate::input::delete_asset_input::Builder::default()
    }
    /// Creates a new `DeleteAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteAsset {
    type Output =
        std::result::Result<crate::output::DeleteAssetOutput, crate::error::DeleteAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_asset_error(response)
        } else {
            crate::operation_deser::parse_delete_asset_response(response)
        }
    }
}

/// Operation shape for `DeleteDataSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_data_set`](crate::client::Client::delete_data_set).
///
/// See [`crate::client::fluent_builders::DeleteDataSet`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteDataSet {
    _private: (),
}
impl DeleteDataSet {
    /// Creates a new builder-style object to manufacture [`DeleteDataSetInput`](crate::input::DeleteDataSetInput)
    pub fn builder() -> crate::input::delete_data_set_input::Builder {
        crate::input::delete_data_set_input::Builder::default()
    }
    /// Creates a new `DeleteDataSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteDataSet {
    type Output =
        std::result::Result<crate::output::DeleteDataSetOutput, crate::error::DeleteDataSetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_data_set_error(response)
        } else {
            crate::operation_deser::parse_delete_data_set_response(response)
        }
    }
}

/// Operation shape for `DeleteEventAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_event_action`](crate::client::Client::delete_event_action).
///
/// See [`crate::client::fluent_builders::DeleteEventAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteEventAction {
    _private: (),
}
impl DeleteEventAction {
    /// Creates a new builder-style object to manufacture [`DeleteEventActionInput`](crate::input::DeleteEventActionInput)
    pub fn builder() -> crate::input::delete_event_action_input::Builder {
        crate::input::delete_event_action_input::Builder::default()
    }
    /// Creates a new `DeleteEventAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteEventAction {
    type Output = std::result::Result<
        crate::output::DeleteEventActionOutput,
        crate::error::DeleteEventActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_event_action_error(response)
        } else {
            crate::operation_deser::parse_delete_event_action_response(response)
        }
    }
}

/// Operation shape for `DeleteRevision`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_revision`](crate::client::Client::delete_revision).
///
/// See [`crate::client::fluent_builders::DeleteRevision`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DeleteRevision {
    _private: (),
}
impl DeleteRevision {
    /// Creates a new builder-style object to manufacture [`DeleteRevisionInput`](crate::input::DeleteRevisionInput)
    pub fn builder() -> crate::input::delete_revision_input::Builder {
        crate::input::delete_revision_input::Builder::default()
    }
    /// Creates a new `DeleteRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteRevision {
    type Output =
        std::result::Result<crate::output::DeleteRevisionOutput, crate::error::DeleteRevisionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_delete_revision_error(response)
        } else {
            crate::operation_deser::parse_delete_revision_response(response)
        }
    }
}

/// Operation shape for `GetAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_asset`](crate::client::Client::get_asset).
///
/// See [`crate::client::fluent_builders::GetAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetAsset {
    _private: (),
}
impl GetAsset {
    /// Creates a new builder-style object to manufacture [`GetAssetInput`](crate::input::GetAssetInput)
    pub fn builder() -> crate::input::get_asset_input::Builder {
        crate::input::get_asset_input::Builder::default()
    }
    /// Creates a new `GetAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetAsset {
    type Output = std::result::Result<crate::output::GetAssetOutput, crate::error::GetAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_asset_error(response)
        } else {
            crate::operation_deser::parse_get_asset_response(response)
        }
    }
}

/// Operation shape for `GetDataSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_data_set`](crate::client::Client::get_data_set).
///
/// See [`crate::client::fluent_builders::GetDataSet`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetDataSet {
    _private: (),
}
impl GetDataSet {
    /// Creates a new builder-style object to manufacture [`GetDataSetInput`](crate::input::GetDataSetInput)
    pub fn builder() -> crate::input::get_data_set_input::Builder {
        crate::input::get_data_set_input::Builder::default()
    }
    /// Creates a new `GetDataSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetDataSet {
    type Output =
        std::result::Result<crate::output::GetDataSetOutput, crate::error::GetDataSetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_data_set_error(response)
        } else {
            crate::operation_deser::parse_get_data_set_response(response)
        }
    }
}

/// Operation shape for `GetEventAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_event_action`](crate::client::Client::get_event_action).
///
/// See [`crate::client::fluent_builders::GetEventAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetEventAction {
    _private: (),
}
impl GetEventAction {
    /// Creates a new builder-style object to manufacture [`GetEventActionInput`](crate::input::GetEventActionInput)
    pub fn builder() -> crate::input::get_event_action_input::Builder {
        crate::input::get_event_action_input::Builder::default()
    }
    /// Creates a new `GetEventAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetEventAction {
    type Output =
        std::result::Result<crate::output::GetEventActionOutput, crate::error::GetEventActionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_event_action_error(response)
        } else {
            crate::operation_deser::parse_get_event_action_response(response)
        }
    }
}

/// Operation shape for `GetJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_job`](crate::client::Client::get_job).
///
/// See [`crate::client::fluent_builders::GetJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetJob {
    _private: (),
}
impl GetJob {
    /// Creates a new builder-style object to manufacture [`GetJobInput`](crate::input::GetJobInput)
    pub fn builder() -> crate::input::get_job_input::Builder {
        crate::input::get_job_input::Builder::default()
    }
    /// Creates a new `GetJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetJob {
    type Output = std::result::Result<crate::output::GetJobOutput, crate::error::GetJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_job_error(response)
        } else {
            crate::operation_deser::parse_get_job_response(response)
        }
    }
}

/// Operation shape for `GetRevision`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_revision`](crate::client::Client::get_revision).
///
/// See [`crate::client::fluent_builders::GetRevision`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetRevision {
    _private: (),
}
impl GetRevision {
    /// Creates a new builder-style object to manufacture [`GetRevisionInput`](crate::input::GetRevisionInput)
    pub fn builder() -> crate::input::get_revision_input::Builder {
        crate::input::get_revision_input::Builder::default()
    }
    /// Creates a new `GetRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetRevision {
    type Output =
        std::result::Result<crate::output::GetRevisionOutput, crate::error::GetRevisionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_revision_error(response)
        } else {
            crate::operation_deser::parse_get_revision_response(response)
        }
    }
}

/// Operation shape for `ListDataSetRevisions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_data_set_revisions`](crate::client::Client::list_data_set_revisions).
///
/// See [`crate::client::fluent_builders::ListDataSetRevisions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataSetRevisions {
    _private: (),
}
impl ListDataSetRevisions {
    /// Creates a new builder-style object to manufacture [`ListDataSetRevisionsInput`](crate::input::ListDataSetRevisionsInput)
    pub fn builder() -> crate::input::list_data_set_revisions_input::Builder {
        crate::input::list_data_set_revisions_input::Builder::default()
    }
    /// Creates a new `ListDataSetRevisions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDataSetRevisions {
    type Output = std::result::Result<
        crate::output::ListDataSetRevisionsOutput,
        crate::error::ListDataSetRevisionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_data_set_revisions_error(response)
        } else {
            crate::operation_deser::parse_list_data_set_revisions_response(response)
        }
    }
}

/// Operation shape for `ListDataSets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_data_sets`](crate::client::Client::list_data_sets).
///
/// See [`crate::client::fluent_builders::ListDataSets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListDataSets {
    _private: (),
}
impl ListDataSets {
    /// Creates a new builder-style object to manufacture [`ListDataSetsInput`](crate::input::ListDataSetsInput)
    pub fn builder() -> crate::input::list_data_sets_input::Builder {
        crate::input::list_data_sets_input::Builder::default()
    }
    /// Creates a new `ListDataSets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListDataSets {
    type Output =
        std::result::Result<crate::output::ListDataSetsOutput, crate::error::ListDataSetsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_data_sets_error(response)
        } else {
            crate::operation_deser::parse_list_data_sets_response(response)
        }
    }
}

/// Operation shape for `ListEventActions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_event_actions`](crate::client::Client::list_event_actions).
///
/// See [`crate::client::fluent_builders::ListEventActions`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListEventActions {
    _private: (),
}
impl ListEventActions {
    /// Creates a new builder-style object to manufacture [`ListEventActionsInput`](crate::input::ListEventActionsInput)
    pub fn builder() -> crate::input::list_event_actions_input::Builder {
        crate::input::list_event_actions_input::Builder::default()
    }
    /// Creates a new `ListEventActions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListEventActions {
    type Output = std::result::Result<
        crate::output::ListEventActionsOutput,
        crate::error::ListEventActionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_event_actions_error(response)
        } else {
            crate::operation_deser::parse_list_event_actions_response(response)
        }
    }
}

/// Operation shape for `ListJobs`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_jobs`](crate::client::Client::list_jobs).
///
/// See [`crate::client::fluent_builders::ListJobs`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListJobs {
    _private: (),
}
impl ListJobs {
    /// Creates a new builder-style object to manufacture [`ListJobsInput`](crate::input::ListJobsInput)
    pub fn builder() -> crate::input::list_jobs_input::Builder {
        crate::input::list_jobs_input::Builder::default()
    }
    /// Creates a new `ListJobs` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListJobs {
    type Output = std::result::Result<crate::output::ListJobsOutput, crate::error::ListJobsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_jobs_error(response)
        } else {
            crate::operation_deser::parse_list_jobs_response(response)
        }
    }
}

/// Operation shape for `ListRevisionAssets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_revision_assets`](crate::client::Client::list_revision_assets).
///
/// See [`crate::client::fluent_builders::ListRevisionAssets`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListRevisionAssets {
    _private: (),
}
impl ListRevisionAssets {
    /// Creates a new builder-style object to manufacture [`ListRevisionAssetsInput`](crate::input::ListRevisionAssetsInput)
    pub fn builder() -> crate::input::list_revision_assets_input::Builder {
        crate::input::list_revision_assets_input::Builder::default()
    }
    /// Creates a new `ListRevisionAssets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListRevisionAssets {
    type Output = std::result::Result<
        crate::output::ListRevisionAssetsOutput,
        crate::error::ListRevisionAssetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_revision_assets_error(response)
        } else {
            crate::operation_deser::parse_list_revision_assets_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput)
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `SendApiAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_api_asset`](crate::client::Client::send_api_asset).
///
/// See [`crate::client::fluent_builders::SendApiAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct SendApiAsset {
    _private: (),
}
impl SendApiAsset {
    /// Creates a new builder-style object to manufacture [`SendApiAssetInput`](crate::input::SendApiAssetInput)
    pub fn builder() -> crate::input::send_api_asset_input::Builder {
        crate::input::send_api_asset_input::Builder::default()
    }
    /// Creates a new `SendApiAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendApiAsset {
    type Output =
        std::result::Result<crate::output::SendApiAssetOutput, crate::error::SendApiAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_api_asset_error(response)
        } else {
            crate::operation_deser::parse_send_api_asset_response(response)
        }
    }
}

/// Operation shape for `StartJob`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_job`](crate::client::Client::start_job).
///
/// See [`crate::client::fluent_builders::StartJob`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct StartJob {
    _private: (),
}
impl StartJob {
    /// Creates a new builder-style object to manufacture [`StartJobInput`](crate::input::StartJobInput)
    pub fn builder() -> crate::input::start_job_input::Builder {
        crate::input::start_job_input::Builder::default()
    }
    /// Creates a new `StartJob` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartJob {
    type Output = std::result::Result<crate::output::StartJobOutput, crate::error::StartJobError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 202 {
            crate::operation_deser::parse_start_job_error(response)
        } else {
            crate::operation_deser::parse_start_job_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput)
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput)
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 204 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateAsset`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_asset`](crate::client::Client::update_asset).
///
/// See [`crate::client::fluent_builders::UpdateAsset`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateAsset {
    _private: (),
}
impl UpdateAsset {
    /// Creates a new builder-style object to manufacture [`UpdateAssetInput`](crate::input::UpdateAssetInput)
    pub fn builder() -> crate::input::update_asset_input::Builder {
        crate::input::update_asset_input::Builder::default()
    }
    /// Creates a new `UpdateAsset` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateAsset {
    type Output =
        std::result::Result<crate::output::UpdateAssetOutput, crate::error::UpdateAssetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_asset_error(response)
        } else {
            crate::operation_deser::parse_update_asset_response(response)
        }
    }
}

/// Operation shape for `UpdateDataSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_data_set`](crate::client::Client::update_data_set).
///
/// See [`crate::client::fluent_builders::UpdateDataSet`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateDataSet {
    _private: (),
}
impl UpdateDataSet {
    /// Creates a new builder-style object to manufacture [`UpdateDataSetInput`](crate::input::UpdateDataSetInput)
    pub fn builder() -> crate::input::update_data_set_input::Builder {
        crate::input::update_data_set_input::Builder::default()
    }
    /// Creates a new `UpdateDataSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateDataSet {
    type Output =
        std::result::Result<crate::output::UpdateDataSetOutput, crate::error::UpdateDataSetError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_data_set_error(response)
        } else {
            crate::operation_deser::parse_update_data_set_response(response)
        }
    }
}

/// Operation shape for `UpdateEventAction`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_event_action`](crate::client::Client::update_event_action).
///
/// See [`crate::client::fluent_builders::UpdateEventAction`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateEventAction {
    _private: (),
}
impl UpdateEventAction {
    /// Creates a new builder-style object to manufacture [`UpdateEventActionInput`](crate::input::UpdateEventActionInput)
    pub fn builder() -> crate::input::update_event_action_input::Builder {
        crate::input::update_event_action_input::Builder::default()
    }
    /// Creates a new `UpdateEventAction` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateEventAction {
    type Output = std::result::Result<
        crate::output::UpdateEventActionOutput,
        crate::error::UpdateEventActionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_event_action_error(response)
        } else {
            crate::operation_deser::parse_update_event_action_response(response)
        }
    }
}

/// Operation shape for `UpdateRevision`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_revision`](crate::client::Client::update_revision).
///
/// See [`crate::client::fluent_builders::UpdateRevision`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct UpdateRevision {
    _private: (),
}
impl UpdateRevision {
    /// Creates a new builder-style object to manufacture [`UpdateRevisionInput`](crate::input::UpdateRevisionInput)
    pub fn builder() -> crate::input::update_revision_input::Builder {
        crate::input::update_revision_input::Builder::default()
    }
    /// Creates a new `UpdateRevision` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateRevision {
    type Output =
        std::result::Result<crate::output::UpdateRevisionOutput, crate::error::UpdateRevisionError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_revision_error(response)
        } else {
            crate::operation_deser::parse_update_revision_response(response)
        }
    }
}

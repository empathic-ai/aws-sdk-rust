// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetCelebrityRecognitionOutput {
    /// <p>The current status of the celebrity recognition job.</p>
    #[doc(hidden)]
    pub job_status: std::option::Option<crate::types::VideoJobStatus>,
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    #[doc(hidden)]
    pub status_message: std::option::Option<std::string::String>,
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    #[doc(hidden)]
    pub video_metadata: std::option::Option<crate::types::VideoMetadata>,
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Array of celebrities recognized in the video.</p>
    #[doc(hidden)]
    pub celebrities: std::option::Option<std::vec::Vec<crate::types::CelebrityRecognition>>,
    _request_id: Option<String>,
}
impl GetCelebrityRecognitionOutput {
    /// <p>The current status of the celebrity recognition job.</p>
    pub fn job_status(&self) -> std::option::Option<&crate::types::VideoJobStatus> {
        self.job_status.as_ref()
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn status_message(&self) -> std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn video_metadata(&self) -> std::option::Option<&crate::types::VideoMetadata> {
        self.video_metadata.as_ref()
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Array of celebrities recognized in the video.</p>
    pub fn celebrities(&self) -> std::option::Option<&[crate::types::CelebrityRecognition]> {
        self.celebrities.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetCelebrityRecognitionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCelebrityRecognitionOutput {
    /// Creates a new builder-style object to manufacture [`GetCelebrityRecognitionOutput`](crate::operation::get_celebrity_recognition::GetCelebrityRecognitionOutput).
    pub fn builder(
    ) -> crate::operation::get_celebrity_recognition::builders::GetCelebrityRecognitionOutputBuilder
    {
        crate::operation::get_celebrity_recognition::builders::GetCelebrityRecognitionOutputBuilder::default()
    }
}

/// A builder for [`GetCelebrityRecognitionOutput`](crate::operation::get_celebrity_recognition::GetCelebrityRecognitionOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
pub struct GetCelebrityRecognitionOutputBuilder {
    pub(crate) job_status: std::option::Option<crate::types::VideoJobStatus>,
    pub(crate) status_message: std::option::Option<std::string::String>,
    pub(crate) video_metadata: std::option::Option<crate::types::VideoMetadata>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) celebrities: std::option::Option<std::vec::Vec<crate::types::CelebrityRecognition>>,
    _request_id: Option<String>,
}
impl GetCelebrityRecognitionOutputBuilder {
    /// <p>The current status of the celebrity recognition job.</p>
    pub fn job_status(mut self, input: crate::types::VideoJobStatus) -> Self {
        self.job_status = Some(input);
        self
    }
    /// <p>The current status of the celebrity recognition job.</p>
    pub fn set_job_status(
        mut self,
        input: std::option::Option<crate::types::VideoJobStatus>,
    ) -> Self {
        self.job_status = input;
        self
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn status_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.status_message = Some(input.into());
        self
    }
    /// <p>If the job fails, <code>StatusMessage</code> provides a descriptive error message.</p>
    pub fn set_status_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn video_metadata(mut self, input: crate::types::VideoMetadata) -> Self {
        self.video_metadata = Some(input);
        self
    }
    /// <p>Information about a video that Amazon Rekognition Video analyzed. <code>Videometadata</code> is returned in every page of paginated responses from a Amazon Rekognition Video operation.</p>
    pub fn set_video_metadata(
        mut self,
        input: std::option::Option<crate::types::VideoMetadata>,
    ) -> Self {
        self.video_metadata = input;
        self
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>If the response is truncated, Amazon Rekognition Video returns this token that you can use in the subsequent request to retrieve the next set of celebrities.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// Appends an item to `celebrities`.
    ///
    /// To override the contents of this collection use [`set_celebrities`](Self::set_celebrities).
    ///
    /// <p>Array of celebrities recognized in the video.</p>
    pub fn celebrities(mut self, input: crate::types::CelebrityRecognition) -> Self {
        let mut v = self.celebrities.unwrap_or_default();
        v.push(input);
        self.celebrities = Some(v);
        self
    }
    /// <p>Array of celebrities recognized in the video.</p>
    pub fn set_celebrities(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::CelebrityRecognition>>,
    ) -> Self {
        self.celebrities = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetCelebrityRecognitionOutput`](crate::operation::get_celebrity_recognition::GetCelebrityRecognitionOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_celebrity_recognition::GetCelebrityRecognitionOutput {
        crate::operation::get_celebrity_recognition::GetCelebrityRecognitionOutput {
            job_status: self.job_status,
            status_message: self.status_message,
            video_metadata: self.video_metadata,
            next_token: self.next_token,
            celebrities: self.celebrities,
            _request_id: self._request_id,
        }
    }
}

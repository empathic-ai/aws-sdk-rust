// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDeniedException(crate::error::AccessDeniedException),
    /// <p>The request conflicts with an existing resource.</p>
    ConflictException(crate::error::ConflictException),
    /// <p>The request processing has failed because of an unknown error, exception or
    /// failure.</p>
    InternalServerException(crate::error::InternalServerException),
    /// <p>A limit has exceeded.</p>
    LimitExceededException(crate::error::LimitExceededException),
    /// <p>One or more resources can't be found.</p>
    ResourceNotFoundException(crate::error::ResourceNotFoundException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an AWS service.</p>
    ValidationException(crate::error::ValidationException),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::LimitExceededException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateChangesetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateChangesetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateChangesetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateChangesetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateChangesetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateChangesetErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateChangesetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateChangesetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateChangesetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateChangesetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::CreateDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateDatasetErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateDataViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateDataViewError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateDataViewErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::CreateDataViewErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::CreateDataViewErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::CreateDataViewErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::CreateDataViewErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::CreateDataViewErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::CreateDataViewErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::DeleteDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::DeleteDatasetErrorKind::LimitExceededException(inner) => {
                    Error::LimitExceededException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::DeleteDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::DeleteDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetChangesetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetChangesetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetChangesetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetChangesetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::GetChangesetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetChangesetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetChangesetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetChangesetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetChangesetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::GetDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetDataViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetDataViewError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetDataViewErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::GetDataViewErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetDataViewErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::GetDataViewErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetDataViewErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetDataViewErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R>
    From<aws_smithy_http::result::SdkError<crate::error::GetProgrammaticAccessCredentialsError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<
            crate::error::GetProgrammaticAccessCredentialsError,
            R,
        >,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::GetProgrammaticAccessCredentialsErrorKind::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
                crate::error::GetProgrammaticAccessCredentialsErrorKind::InternalServerException(inner) => Error::InternalServerException(inner),
                crate::error::GetProgrammaticAccessCredentialsErrorKind::ThrottlingException(inner) => Error::ThrottlingException(inner),
                crate::error::GetProgrammaticAccessCredentialsErrorKind::ValidationException(inner) => Error::ValidationException(inner),
                crate::error::GetProgrammaticAccessCredentialsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetWorkingLocationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetWorkingLocationError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetWorkingLocationErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::GetWorkingLocationErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::GetWorkingLocationErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::GetWorkingLocationErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::GetWorkingLocationErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListChangesetsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListChangesetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListChangesetsErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::ListChangesetsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListChangesetsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListChangesetsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListChangesetsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListChangesetsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListChangesetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDatasetsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDatasetsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDatasetsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListDatasetsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListDatasetsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListDatasetsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListDatasetsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListDatasetsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDataViewsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListDataViewsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDataViewsErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::ListDataViewsErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::ListDataViewsErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::ListDataViewsErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::ListDataViewsErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::ListDataViewsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateChangesetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateChangesetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateChangesetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateChangesetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateChangesetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateChangesetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateChangesetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateChangesetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateChangesetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UpdateDatasetError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UpdateDatasetError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UpdateDatasetErrorKind::AccessDeniedException(inner) => {
                    Error::AccessDeniedException(inner)
                }
                crate::error::UpdateDatasetErrorKind::ConflictException(inner) => {
                    Error::ConflictException(inner)
                }
                crate::error::UpdateDatasetErrorKind::InternalServerException(inner) => {
                    Error::InternalServerException(inner)
                }
                crate::error::UpdateDatasetErrorKind::ResourceNotFoundException(inner) => {
                    Error::ResourceNotFoundException(inner)
                }
                crate::error::UpdateDatasetErrorKind::ThrottlingException(inner) => {
                    Error::ThrottlingException(inner)
                }
                crate::error::UpdateDatasetErrorKind::ValidationException(inner) => {
                    Error::ValidationException(inner)
                }
                crate::error::UpdateDatasetErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_scaling_plan_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CreateScalingPlanOutput, crate::error::CreateScalingPlanError>
{
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::CreateScalingPlanError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CreateScalingPlanError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::CreateScalingPlanError {
            meta: generic,
            kind: crate::error::CreateScalingPlanErrorKind::ConcurrentUpdateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_concurrent_update_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::CreateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::CreateScalingPlanError {
            meta: generic,
            kind: crate::error::CreateScalingPlanErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "LimitExceededException" => crate::error::CreateScalingPlanError {
            meta: generic,
            kind: crate::error::CreateScalingPlanErrorKind::LimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_limit_exceeded_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::CreateScalingPlanError {
            meta: generic,
            kind: crate::error::CreateScalingPlanErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::CreateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::CreateScalingPlanError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_create_scaling_plan_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CreateScalingPlanOutput, crate::error::CreateScalingPlanError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_scaling_plan_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_create_scaling_plan(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::CreateScalingPlanError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_scaling_plan_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteScalingPlanOutput, crate::error::DeleteScalingPlanError>
{
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::DeleteScalingPlanError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DeleteScalingPlanError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::DeleteScalingPlanError {
            meta: generic,
            kind: crate::error::DeleteScalingPlanErrorKind::ConcurrentUpdateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_concurrent_update_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::DeleteScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DeleteScalingPlanError {
            meta: generic,
            kind: crate::error::DeleteScalingPlanErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ObjectNotFoundException" => crate::error::DeleteScalingPlanError {
            meta: generic,
            kind: crate::error::DeleteScalingPlanErrorKind::ObjectNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_object_not_found_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DeleteScalingPlanError {
            meta: generic,
            kind: crate::error::DeleteScalingPlanErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DeleteScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DeleteScalingPlanError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_delete_scaling_plan_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DeleteScalingPlanOutput, crate::error::DeleteScalingPlanError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_scaling_plan_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_scaling_plan_resources_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeScalingPlanResourcesOutput,
    crate::error::DescribeScalingPlanResourcesError,
> {
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::DescribeScalingPlanResourcesError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::DescribeScalingPlanResourcesError {
            meta: generic,
            kind: crate::error::DescribeScalingPlanResourcesErrorKind::ConcurrentUpdateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_concurrent_update_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DescribeScalingPlanResourcesError {
            meta: generic,
            kind: crate::error::DescribeScalingPlanResourcesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidNextTokenException" => crate::error::DescribeScalingPlanResourcesError {
            meta: generic,
            kind: crate::error::DescribeScalingPlanResourcesErrorKind::InvalidNextTokenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_invalid_next_token_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeScalingPlanResourcesError {
            meta: generic,
            kind: crate::error::DescribeScalingPlanResourcesErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeScalingPlanResourcesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_scaling_plan_resources_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeScalingPlanResourcesOutput,
    crate::error::DescribeScalingPlanResourcesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_scaling_plan_resources_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_describe_scaling_plan_resources(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeScalingPlanResourcesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_scaling_plans_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeScalingPlansOutput,
    crate::error::DescribeScalingPlansError,
> {
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::DescribeScalingPlansError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeScalingPlansError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::DescribeScalingPlansError {
            meta: generic,
            kind: crate::error::DescribeScalingPlansErrorKind::ConcurrentUpdateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_concurrent_update_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::DescribeScalingPlansError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DescribeScalingPlansError {
            meta: generic,
            kind: crate::error::DescribeScalingPlansErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeScalingPlansError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InvalidNextTokenException" => {
            crate::error::DescribeScalingPlansError {
                meta: generic,
                kind: crate::error::DescribeScalingPlansErrorKind::InvalidNextTokenException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output =
                            crate::error::invalid_next_token_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_invalid_next_token_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::DescribeScalingPlansError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "ValidationException" => crate::error::DescribeScalingPlansError {
            meta: generic,
            kind: crate::error::DescribeScalingPlansErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::DescribeScalingPlansError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeScalingPlansError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_scaling_plans_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::DescribeScalingPlansOutput,
    crate::error::DescribeScalingPlansError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_scaling_plans_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_describe_scaling_plans(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeScalingPlansError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_scaling_plan_resource_forecast_data_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetScalingPlanResourceForecastDataOutput,
    crate::error::GetScalingPlanResourceForecastDataError,
> {
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::GetScalingPlanResourceForecastDataError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::error::GetScalingPlanResourceForecastDataError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::GetScalingPlanResourceForecastDataError {
            meta: generic,
            kind:
                crate::error::GetScalingPlanResourceForecastDataErrorKind::InternalServiceException(
                    {
                        #[allow(unused_mut)]
                        let mut tmp = {
                            #[allow(unused_mut)]
                            let mut output =
                                crate::error::internal_service_exception::Builder::default();
                            let _ = response;
                            output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(response.body().as_ref(), output).map_err(crate::error::GetScalingPlanResourceForecastDataError::unhandled)?;
                            output.build()
                        };
                        if (&tmp.message).is_none() {
                            tmp.message = _error_message;
                        }
                        tmp
                    },
                ),
        },
        "ValidationException" => crate::error::GetScalingPlanResourceForecastDataError {
            meta: generic,
            kind: crate::error::GetScalingPlanResourceForecastDataErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::GetScalingPlanResourceForecastDataError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetScalingPlanResourceForecastDataError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_scaling_plan_resource_forecast_data_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetScalingPlanResourceForecastDataOutput,
    crate::error::GetScalingPlanResourceForecastDataError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::output::get_scaling_plan_resource_forecast_data_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_get_scaling_plan_resource_forecast_data(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetScalingPlanResourceForecastDataError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_scaling_plan_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UpdateScalingPlanOutput, crate::error::UpdateScalingPlanError>
{
    let generic = crate::json_deser::parse_generic_error(&response)
        .map_err(crate::error::UpdateScalingPlanError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::UpdateScalingPlanError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::UpdateScalingPlanError {
            meta: generic,
            kind: crate::error::UpdateScalingPlanErrorKind::ConcurrentUpdateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_concurrent_update_exceptionjson_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::UpdateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::UpdateScalingPlanError {
            meta: generic,
            kind: crate::error::UpdateScalingPlanErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_internal_service_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::UpdateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ObjectNotFoundException" => crate::error::UpdateScalingPlanError {
            meta: generic,
            kind: crate::error::UpdateScalingPlanErrorKind::ObjectNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_object_not_found_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::UpdateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::UpdateScalingPlanError {
            meta: generic,
            kind: crate::error::UpdateScalingPlanErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_validation_exceptionjson_err(
                        response.body().as_ref(),
                        output,
                    )
                    .map_err(crate::error::UpdateScalingPlanError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::UpdateScalingPlanError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_update_scaling_plan_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::UpdateScalingPlanOutput, crate::error::UpdateScalingPlanError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_scaling_plan_output::Builder::default();
        let _ = response;
        output.build()
    })
}

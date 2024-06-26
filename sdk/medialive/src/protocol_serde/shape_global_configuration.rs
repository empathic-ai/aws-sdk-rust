// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_global_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GlobalConfiguration,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.initial_audio_gain != 0 {
        object.key("initialAudioGain").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.initial_audio_gain).into()),
        );
    }
    if let Some(var_1) = &input.input_end_action {
        object.key("inputEndAction").string(var_1.as_str());
    }
    if let Some(var_2) = &input.input_loss_behavior {
        #[allow(unused_mut)]
        let mut object_3 = object.key("inputLossBehavior").start_object();
        crate::protocol_serde::shape_input_loss_behavior::ser_input_loss_behavior(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.output_locking_mode {
        object.key("outputLockingMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.output_timing_source {
        object.key("outputTimingSource").string(var_5.as_str());
    }
    if let Some(var_6) = &input.support_low_framerate_inputs {
        object
            .key("supportLowFramerateInputs")
            .string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_global_configuration<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::GlobalConfiguration>,
    aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<
        Item = Result<
            aws_smithy_json::deserialize::Token<'a>,
            aws_smithy_json::deserialize::error::DeserializeError,
        >,
    >,
{
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GlobalConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "initialAudioGain" => {
                                builder = builder.set_initial_audio_gain(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "inputEndAction" => {
                                builder = builder.set_input_end_action(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::GlobalConfigurationInputEndAction::from(
                                                u.as_ref(),
                                            )
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "inputLossBehavior" => {
                                builder = builder.set_input_loss_behavior(
                                    crate::protocol_serde::shape_input_loss_behavior::de_input_loss_behavior(tokens)?
                                );
                            }
                            "outputLockingMode" => {
                                builder = builder.set_output_locking_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::types::GlobalConfigurationOutputLockingMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "outputTimingSource" => {
                                builder = builder.set_output_timing_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::types::GlobalConfigurationOutputTimingSource::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "supportLowFramerateInputs" => {
                                builder = builder.set_support_low_framerate_inputs(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::types::GlobalConfigurationLowFramerateInputs::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?,
                        }
                    }
                    other => {
                        return Err(
                            aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                                "expected object key or end object, found: {:?}",
                                other
                            )),
                        )
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(
            aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ),
        ),
    }
}

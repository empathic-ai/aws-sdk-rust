// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_frame_capture_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FrameCaptureSettings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.framerate_denominator != 0 {
        object.key("framerateDenominator").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.framerate_denominator).into()),
        );
    }
    if input.framerate_numerator != 0 {
        object.key("framerateNumerator").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.framerate_numerator).into()),
        );
    }
    if input.max_captures != 0 {
        object.key("maxCaptures").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_captures).into()),
        );
    }
    if input.quality != 0 {
        object.key("quality").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.quality).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_frame_capture_settings<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::FrameCaptureSettings>,
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
            let mut builder = crate::types::builders::FrameCaptureSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "framerateDenominator" => {
                                builder = builder.set_framerate_denominator(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "framerateNumerator" => {
                                builder = builder.set_framerate_numerator(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "maxCaptures" => {
                                builder = builder.set_max_captures(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "quality" => {
                                builder = builder.set_quality(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
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

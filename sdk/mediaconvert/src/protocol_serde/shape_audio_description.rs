// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_audio_description(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AudioDescription,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_channel_tagging_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("audioChannelTaggingSettings").start_object();
        crate::protocol_serde::shape_audio_channel_tagging_settings::ser_audio_channel_tagging_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.audio_normalization_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("audioNormalizationSettings").start_object();
        crate::protocol_serde::shape_audio_normalization_settings::ser_audio_normalization_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.audio_source_name {
        object.key("audioSourceName").string(var_5.as_str());
    }
    if input.audio_type != 0 {
        object.key("audioType").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.audio_type).into()),
        );
    }
    if let Some(var_6) = &input.audio_type_control {
        object.key("audioTypeControl").string(var_6.as_str());
    }
    if let Some(var_7) = &input.codec_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("codecSettings").start_object();
        crate::protocol_serde::shape_audio_codec_settings::ser_audio_codec_settings(
            &mut object_8,
            var_7,
        )?;
        object_8.finish();
    }
    if let Some(var_9) = &input.custom_language_code {
        object.key("customLanguageCode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.language_code {
        object.key("languageCode").string(var_10.as_str());
    }
    if let Some(var_11) = &input.language_code_control {
        object.key("languageCodeControl").string(var_11.as_str());
    }
    if let Some(var_12) = &input.remix_settings {
        #[allow(unused_mut)]
        let mut object_13 = object.key("remixSettings").start_object();
        crate::protocol_serde::shape_remix_settings::ser_remix_settings(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.stream_name {
        object.key("streamName").string(var_14.as_str());
    }
    Ok(())
}

pub(crate) fn de_audio_description<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<
    Option<crate::types::AudioDescription>,
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
            let mut builder = crate::types::builders::AudioDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioChannelTaggingSettings" => {
                                builder = builder.set_audio_channel_tagging_settings(
                                    crate::protocol_serde::shape_audio_channel_tagging_settings::de_audio_channel_tagging_settings(tokens)?
                                );
                            }
                            "audioNormalizationSettings" => {
                                builder = builder.set_audio_normalization_settings(
                                    crate::protocol_serde::shape_audio_normalization_settings::de_audio_normalization_settings(tokens)?
                                );
                            }
                            "audioSourceName" => {
                                builder = builder.set_audio_source_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "audioType" => {
                                builder = builder.set_audio_type(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "audioTypeControl" => {
                                builder = builder.set_audio_type_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::AudioTypeControl::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "codecSettings" => {
                                builder = builder.set_codec_settings(
                                    crate::protocol_serde::shape_audio_codec_settings::de_audio_codec_settings(tokens)?
                                );
                            }
                            "customLanguageCode" => {
                                builder = builder.set_custom_language_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                                );
                            }
                            "languageCode" => {
                                builder = builder.set_language_code(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::LanguageCode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "languageCodeControl" => {
                                builder = builder.set_language_code_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::AudioLanguageCodeControl::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "remixSettings" => {
                                builder = builder.set_remix_settings(
                                    crate::protocol_serde::shape_remix_settings::de_remix_settings(
                                        tokens,
                                    )?,
                                );
                            }
                            "streamName" => {
                                builder = builder.set_stream_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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

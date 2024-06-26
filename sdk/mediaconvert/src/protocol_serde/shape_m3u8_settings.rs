// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_m3u8_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::M3u8Settings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_duration {
        object.key("audioDuration").string(var_1.as_str());
    }
    if input.audio_frames_per_pes != 0 {
        object.key("audioFramesPerPes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.audio_frames_per_pes).into()),
        );
    }
    if let Some(var_2) = &input.audio_pids {
        let mut array_3 = object.key("audioPids").start_array();
        for item_4 in var_2 {
            {
                array_3.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*item_4).into()),
                );
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.data_pts_control {
        object.key("dataPTSControl").string(var_5.as_str());
    }
    if input.max_pcr_interval != 0 {
        object.key("maxPcrInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_pcr_interval).into()),
        );
    }
    if let Some(var_6) = &input.nielsen_id3 {
        object.key("nielsenId3").string(var_6.as_str());
    }
    if input.pat_interval != 0 {
        object.key("patInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.pat_interval).into()),
        );
    }
    if let Some(var_7) = &input.pcr_control {
        object.key("pcrControl").string(var_7.as_str());
    }
    if input.pcr_pid != 0 {
        object.key("pcrPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.pcr_pid).into()),
        );
    }
    if input.pmt_interval != 0 {
        object.key("pmtInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.pmt_interval).into()),
        );
    }
    if input.pmt_pid != 0 {
        object.key("pmtPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.pmt_pid).into()),
        );
    }
    if input.private_metadata_pid != 0 {
        object.key("privateMetadataPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.private_metadata_pid).into()),
        );
    }
    if input.program_number != 0 {
        object.key("programNumber").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.program_number).into()),
        );
    }
    if input.scte35_pid != 0 {
        object.key("scte35Pid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.scte35_pid).into()),
        );
    }
    if let Some(var_8) = &input.scte35_source {
        object.key("scte35Source").string(var_8.as_str());
    }
    if let Some(var_9) = &input.timed_metadata {
        object.key("timedMetadata").string(var_9.as_str());
    }
    if input.timed_metadata_pid != 0 {
        object.key("timedMetadataPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.timed_metadata_pid).into()),
        );
    }
    if input.transport_stream_id != 0 {
        object.key("transportStreamId").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.transport_stream_id).into()),
        );
    }
    if input.video_pid != 0 {
        object.key("videoPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.video_pid).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_m3u8_settings<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::types::M3u8Settings>, aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::M3u8SettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioDuration" => {
                                builder = builder.set_audio_duration(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M3u8AudioDuration::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "audioFramesPerPes" => {
                                builder = builder.set_audio_frames_per_pes(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "audioPids" => {
                                builder = builder.set_audio_pids(
                                    crate::protocol_serde::shape___list_of__integer_min32_max8182::de___list_of__integer_min32_max8182(tokens)?
                                );
                            }
                            "dataPTSControl" => {
                                builder = builder.set_data_pts_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M3u8DataPtsControl::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "maxPcrInterval" => {
                                builder = builder.set_max_pcr_interval(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "nielsenId3" => {
                                builder = builder.set_nielsen_id3(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::M3u8NielsenId3::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "patInterval" => {
                                builder = builder.set_pat_interval(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "pcrControl" => {
                                builder = builder.set_pcr_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::M3u8PcrControl::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "pcrPid" => {
                                builder = builder.set_pcr_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "pmtInterval" => {
                                builder = builder.set_pmt_interval(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "pmtPid" => {
                                builder = builder.set_pmt_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "privateMetadataPid" => {
                                builder = builder.set_private_metadata_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "programNumber" => {
                                builder = builder.set_program_number(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "scte35Pid" => {
                                builder = builder.set_scte35_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "scte35Source" => {
                                builder = builder.set_scte35_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M3u8Scte35Source::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "timedMetadata" => {
                                builder = builder.set_timed_metadata(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::TimedMetadata::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "timedMetadataPid" => {
                                builder = builder.set_timed_metadata_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "transportStreamId" => {
                                builder = builder.set_transport_stream_id(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "videoPid" => {
                                builder = builder.set_video_pid(
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

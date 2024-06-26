// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_m2ts_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::M2tsSettings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_buffer_model {
        object.key("audioBufferModel").string(var_1.as_str());
    }
    if let Some(var_2) = &input.audio_duration {
        object.key("audioDuration").string(var_2.as_str());
    }
    if input.audio_frames_per_pes != 0 {
        object.key("audioFramesPerPes").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.audio_frames_per_pes).into()),
        );
    }
    if let Some(var_3) = &input.audio_pids {
        let mut array_4 = object.key("audioPids").start_array();
        for item_5 in var_3 {
            {
                array_4.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*item_5).into()),
                );
            }
        }
        array_4.finish();
    }
    if input.bitrate != 0 {
        object.key("bitrate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.bitrate).into()),
        );
    }
    if let Some(var_6) = &input.buffer_model {
        object.key("bufferModel").string(var_6.as_str());
    }
    if let Some(var_7) = &input.data_pts_control {
        object.key("dataPTSControl").string(var_7.as_str());
    }
    if let Some(var_8) = &input.dvb_nit_settings {
        #[allow(unused_mut)]
        let mut object_9 = object.key("dvbNitSettings").start_object();
        crate::protocol_serde::shape_dvb_nit_settings::ser_dvb_nit_settings(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.dvb_sdt_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("dvbSdtSettings").start_object();
        crate::protocol_serde::shape_dvb_sdt_settings::ser_dvb_sdt_settings(
            &mut object_11,
            var_10,
        )?;
        object_11.finish();
    }
    if let Some(var_12) = &input.dvb_sub_pids {
        let mut array_13 = object.key("dvbSubPids").start_array();
        for item_14 in var_12 {
            {
                array_13.value().number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::NegInt((*item_14).into()),
                );
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.dvb_tdt_settings {
        #[allow(unused_mut)]
        let mut object_16 = object.key("dvbTdtSettings").start_object();
        crate::protocol_serde::shape_dvb_tdt_settings::ser_dvb_tdt_settings(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if input.dvb_teletext_pid != 0 {
        object.key("dvbTeletextPid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.dvb_teletext_pid).into()),
        );
    }
    if let Some(var_17) = &input.ebp_audio_interval {
        object.key("ebpAudioInterval").string(var_17.as_str());
    }
    if let Some(var_18) = &input.ebp_placement {
        object.key("ebpPlacement").string(var_18.as_str());
    }
    if let Some(var_19) = &input.es_rate_in_pes {
        object.key("esRateInPes").string(var_19.as_str());
    }
    if let Some(var_20) = &input.force_ts_video_ebp_order {
        object.key("forceTsVideoEbpOrder").string(var_20.as_str());
    }
    if input.fragment_time != 0.0 {
        object.key("fragmentTime").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.fragment_time).into()),
        );
    }
    if let Some(var_21) = &input.klv_metadata {
        object.key("klvMetadata").string(var_21.as_str());
    }
    if input.max_pcr_interval != 0 {
        object.key("maxPcrInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_pcr_interval).into()),
        );
    }
    if input.min_ebp_interval != 0 {
        object.key("minEbpInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.min_ebp_interval).into()),
        );
    }
    if let Some(var_22) = &input.nielsen_id3 {
        object.key("nielsenId3").string(var_22.as_str());
    }
    if input.null_packet_bitrate != 0.0 {
        object.key("nullPacketBitrate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.null_packet_bitrate).into()),
        );
    }
    if input.pat_interval != 0 {
        object.key("patInterval").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.pat_interval).into()),
        );
    }
    if let Some(var_23) = &input.pcr_control {
        object.key("pcrControl").string(var_23.as_str());
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
    if let Some(var_24) = &input.rate_mode {
        object.key("rateMode").string(var_24.as_str());
    }
    if let Some(var_25) = &input.scte35_esam {
        #[allow(unused_mut)]
        let mut object_26 = object.key("scte35Esam").start_object();
        crate::protocol_serde::shape_m2ts_scte35_esam::ser_m2ts_scte35_esam(
            &mut object_26,
            var_25,
        )?;
        object_26.finish();
    }
    if input.scte35_pid != 0 {
        object.key("scte35Pid").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.scte35_pid).into()),
        );
    }
    if let Some(var_27) = &input.scte35_source {
        object.key("scte35Source").string(var_27.as_str());
    }
    if let Some(var_28) = &input.segmentation_markers {
        object.key("segmentationMarkers").string(var_28.as_str());
    }
    if let Some(var_29) = &input.segmentation_style {
        object.key("segmentationStyle").string(var_29.as_str());
    }
    if input.segmentation_time != 0.0 {
        object.key("segmentationTime").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::Float((input.segmentation_time).into()),
        );
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

pub(crate) fn de_m2ts_settings<'a, I>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Option<crate::types::M2tsSettings>, aws_smithy_json::deserialize::error::DeserializeError>
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
            let mut builder = crate::types::builders::M2tsSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioBufferModel" => {
                                builder = builder.set_audio_buffer_model(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsAudioBufferModel::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "audioDuration" => {
                                builder = builder.set_audio_duration(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsAudioDuration::from(u.as_ref())
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
                            "bitrate" => {
                                builder = builder.set_bitrate(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "bufferModel" => {
                                builder = builder.set_buffer_model(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsBufferModel::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "dataPTSControl" => {
                                builder = builder.set_data_pts_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsDataPtsControl::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "dvbNitSettings" => {
                                builder = builder.set_dvb_nit_settings(
                                    crate::protocol_serde::shape_dvb_nit_settings::de_dvb_nit_settings(tokens)?
                                );
                            }
                            "dvbSdtSettings" => {
                                builder = builder.set_dvb_sdt_settings(
                                    crate::protocol_serde::shape_dvb_sdt_settings::de_dvb_sdt_settings(tokens)?
                                );
                            }
                            "dvbSubPids" => {
                                builder = builder.set_dvb_sub_pids(
                                    crate::protocol_serde::shape___list_of__integer_min32_max8182::de___list_of__integer_min32_max8182(tokens)?
                                );
                            }
                            "dvbTdtSettings" => {
                                builder = builder.set_dvb_tdt_settings(
                                    crate::protocol_serde::shape_dvb_tdt_settings::de_dvb_tdt_settings(tokens)?
                                );
                            }
                            "dvbTeletextPid" => {
                                builder = builder.set_dvb_teletext_pid(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(i32::try_from)
                                    .transpose()?,
                                );
                            }
                            "ebpAudioInterval" => {
                                builder = builder.set_ebp_audio_interval(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsEbpAudioInterval::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "ebpPlacement" => {
                                builder = builder.set_ebp_placement(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsEbpPlacement::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "esRateInPes" => {
                                builder = builder.set_es_rate_in_pes(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsEsRateInPes::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "forceTsVideoEbpOrder" => {
                                builder = builder.set_force_ts_video_ebp_order(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsForceTsVideoEbpOrder::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "fragmentTime" => {
                                builder = builder.set_fragment_time(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_f64_lossy()),
                                );
                            }
                            "klvMetadata" => {
                                builder = builder.set_klv_metadata(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsKlvMetadata::from(u.as_ref())
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
                            "minEbpInterval" => {
                                builder = builder.set_min_ebp_interval(
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
                                            .map(|u| crate::types::M2tsNielsenId3::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "nullPacketBitrate" => {
                                builder = builder.set_null_packet_bitrate(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_f64_lossy()),
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
                                            .map(|u| crate::types::M2tsPcrControl::from(u.as_ref()))
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
                            "rateMode" => {
                                builder = builder.set_rate_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::M2tsRateMode::from(u.as_ref()))
                                    })
                                    .transpose()?,
                                );
                            }
                            "scte35Esam" => {
                                builder = builder.set_scte35_esam(
                                    crate::protocol_serde::shape_m2ts_scte35_esam::de_m2ts_scte35_esam(tokens)?
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
                                            crate::types::M2tsScte35Source::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "segmentationMarkers" => {
                                builder = builder.set_segmentation_markers(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsSegmentationMarkers::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "segmentationStyle" => {
                                builder = builder.set_segmentation_style(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|s| {
                                        s.to_unescaped().map(|u| {
                                            crate::types::M2tsSegmentationStyle::from(u.as_ref())
                                        })
                                    })
                                    .transpose()?,
                                );
                            }
                            "segmentationTime" => {
                                builder = builder.set_segmentation_time(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(
                                        tokens.next(),
                                    )?
                                    .map(|v| v.to_f64_lossy()),
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

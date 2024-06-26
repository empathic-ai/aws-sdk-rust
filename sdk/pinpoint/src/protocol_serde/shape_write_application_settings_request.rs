// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_write_application_settings_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WriteApplicationSettingsRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.campaign_hook {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CampaignHook").start_object();
        crate::protocol_serde::shape_campaign_hook::ser_campaign_hook(&mut object_2, var_1)?;
        object_2.finish();
    }
    if input.cloud_watch_metrics_enabled {
        object
            .key("CloudWatchMetricsEnabled")
            .boolean(input.cloud_watch_metrics_enabled);
    }
    if input.event_tagging_enabled {
        object
            .key("EventTaggingEnabled")
            .boolean(input.event_tagging_enabled);
    }
    if let Some(var_3) = &input.limits {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Limits").start_object();
        crate::protocol_serde::shape_campaign_limits::ser_campaign_limits(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.quiet_time {
        #[allow(unused_mut)]
        let mut object_6 = object.key("QuietTime").start_object();
        crate::protocol_serde::shape_quiet_time::ser_quiet_time(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

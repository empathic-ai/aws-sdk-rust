// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_input_device_configurable_settings(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InputDeviceConfigurableSettings,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configured_input {
        object.key("configuredInput").string(var_1.as_str());
    }
    if input.max_bitrate != 0 {
        object.key("maxBitrate").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_bitrate).into()),
        );
    }
    if input.latency_ms != 0 {
        object.key("latencyMs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.latency_ms).into()),
        );
    }
    Ok(())
}

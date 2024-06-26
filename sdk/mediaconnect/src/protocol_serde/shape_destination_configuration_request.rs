// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_destination_configuration_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DestinationConfigurationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.destination_ip {
        object.key("destinationIp").string(var_1.as_str());
    }
    {
        object.key("destinationPort").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.destination_port).into()),
        );
    }
    if let Some(var_2) = &input.interface {
        #[allow(unused_mut)]
        let mut object_3 = object.key("interface").start_object();
        crate::protocol_serde::shape_interface_request::ser_interface_request(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    Ok(())
}

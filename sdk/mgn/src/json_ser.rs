// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_change_server_life_cycle_state_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ChangeServerLifeCycleStateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.life_cycle {
        let mut object_2 = object.key("lifeCycle").start_object();
        crate::json_ser::serialize_structure_crate_model_change_server_life_cycle_state_source_server_lifecycle(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.source_server_id {
        object.key("sourceServerID").string(var_3);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_replication_configuration_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReplicationConfigurationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_4) = &input.associate_default_security_group {
        object.key("associateDefaultSecurityGroup").boolean(*var_4);
    }
    {
        object.key("bandwidthThrottling").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.bandwidth_throttling).into()),
        );
    }
    if let Some(var_5) = &input.create_public_ip {
        object.key("createPublicIP").boolean(*var_5);
    }
    if let Some(var_6) = &input.data_plane_routing {
        object.key("dataPlaneRouting").string(var_6.as_str());
    }
    if let Some(var_7) = &input.default_large_staging_disk_type {
        object
            .key("defaultLargeStagingDiskType")
            .string(var_7.as_str());
    }
    if let Some(var_8) = &input.ebs_encryption {
        object.key("ebsEncryption").string(var_8.as_str());
    }
    if let Some(var_9) = &input.ebs_encryption_key_arn {
        object.key("ebsEncryptionKeyArn").string(var_9);
    }
    if let Some(var_10) = &input.replication_server_instance_type {
        object.key("replicationServerInstanceType").string(var_10);
    }
    if let Some(var_11) = &input.replication_servers_security_groups_i_ds {
        let mut array_12 = object
            .key("replicationServersSecurityGroupsIDs")
            .start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13);
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.staging_area_subnet_id {
        object.key("stagingAreaSubnetId").string(var_14);
    }
    if let Some(var_15) = &input.staging_area_tags {
        let mut object_16 = object.key("stagingAreaTags").start_object();
        for (key_17, value_18) in var_15 {
            {
                object_16.key(key_17).string(value_18);
            }
        }
        object_16.finish();
    }
    if let Some(var_19) = &input.tags {
        let mut object_20 = object.key("tags").start_object();
        for (key_21, value_22) in var_19 {
            {
                object_20.key(key_21).string(value_22);
            }
        }
        object_20.finish();
    }
    if let Some(var_23) = &input.use_dedicated_replication_server {
        object.key("useDedicatedReplicationServer").boolean(*var_23);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_job_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteJobInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_24) = &input.job_id {
        object.key("jobID").string(var_24);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_replication_configuration_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteReplicationConfigurationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_25) = &input.replication_configuration_template_id {
        object
            .key("replicationConfigurationTemplateID")
            .string(var_25);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_source_server_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSourceServerInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_26) = &input.source_server_id {
        object.key("sourceServerID").string(var_26);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_vcenter_client_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteVcenterClientInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_27) = &input.vcenter_client_id {
        object.key("vcenterClientID").string(var_27);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_job_log_items_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeJobLogItemsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_28) = &input.job_id {
        object.key("jobID").string(var_28);
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_29) = &input.next_token {
        object.key("nextToken").string(var_29);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_jobs_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeJobsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_30) = &input.filters {
        let mut object_31 = object.key("filters").start_object();
        crate::json_ser::serialize_structure_crate_model_describe_jobs_request_filters(
            &mut object_31,
            var_30,
        )?;
        object_31.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_32) = &input.next_token {
        object.key("nextToken").string(var_32);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_replication_configuration_templates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeReplicationConfigurationTemplatesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_33) = &input.next_token {
        object.key("nextToken").string(var_33);
    }
    if let Some(var_34) = &input.replication_configuration_template_i_ds {
        let mut array_35 = object
            .key("replicationConfigurationTemplateIDs")
            .start_array();
        for item_36 in var_34 {
            {
                array_35.value().string(item_36);
            }
        }
        array_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_source_servers_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSourceServersInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_37) = &input.filters {
        let mut object_38 = object.key("filters").start_object();
        crate::json_ser::serialize_structure_crate_model_describe_source_servers_request_filters(
            &mut object_38,
            var_37,
        )?;
        object_38.finish();
    }
    if input.max_results != 0 {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.max_results).into()),
        );
    }
    if let Some(var_39) = &input.next_token {
        object.key("nextToken").string(var_39);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_disconnect_from_service_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DisconnectFromServiceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_40) = &input.source_server_id {
        object.key("sourceServerID").string(var_40);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_finalize_cutover_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::FinalizeCutoverInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_41) = &input.source_server_id {
        object.key("sourceServerID").string(var_41);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_launch_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetLaunchConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_42) = &input.source_server_id {
        object.key("sourceServerID").string(var_42);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_replication_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetReplicationConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.source_server_id {
        object.key("sourceServerID").string(var_43);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_mark_as_archived_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::MarkAsArchivedInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_44) = &input.source_server_id {
        object.key("sourceServerID").string(var_44);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_retry_data_replication_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RetryDataReplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_45) = &input.source_server_id {
        object.key("sourceServerID").string(var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_cutover_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartCutoverInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.source_server_i_ds {
        let mut array_47 = object.key("sourceServerIDs").start_array();
        for item_48 in var_46 {
            {
                array_47.value().string(item_48);
            }
        }
        array_47.finish();
    }
    if let Some(var_49) = &input.tags {
        let mut object_50 = object.key("tags").start_object();
        for (key_51, value_52) in var_49 {
            {
                object_50.key(key_51).string(value_52);
            }
        }
        object_50.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_replication_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartReplicationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.source_server_id {
        object.key("sourceServerID").string(var_53);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_test_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartTestInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.source_server_i_ds {
        let mut array_55 = object.key("sourceServerIDs").start_array();
        for item_56 in var_54 {
            {
                array_55.value().string(item_56);
            }
        }
        array_55.finish();
    }
    if let Some(var_57) = &input.tags {
        let mut object_58 = object.key("tags").start_object();
        for (key_59, value_60) in var_57 {
            {
                object_58.key(key_59).string(value_60);
            }
        }
        object_58.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.tags {
        let mut object_62 = object.key("tags").start_object();
        for (key_63, value_64) in var_61 {
            {
                object_62.key(key_63).string(value_64);
            }
        }
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_terminate_target_instances_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TerminateTargetInstancesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.source_server_i_ds {
        let mut array_66 = object.key("sourceServerIDs").start_array();
        for item_67 in var_65 {
            {
                array_66.value().string(item_67);
            }
        }
        array_66.finish();
    }
    if let Some(var_68) = &input.tags {
        let mut object_69 = object.key("tags").start_object();
        for (key_70, value_71) in var_68 {
            {
                object_69.key(key_70).string(value_71);
            }
        }
        object_69.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_launch_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateLaunchConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_72) = &input.copy_private_ip {
        object.key("copyPrivateIp").boolean(*var_72);
    }
    if let Some(var_73) = &input.copy_tags {
        object.key("copyTags").boolean(*var_73);
    }
    if let Some(var_74) = &input.launch_disposition {
        object.key("launchDisposition").string(var_74.as_str());
    }
    if let Some(var_75) = &input.licensing {
        let mut object_76 = object.key("licensing").start_object();
        crate::json_ser::serialize_structure_crate_model_licensing(&mut object_76, var_75)?;
        object_76.finish();
    }
    if let Some(var_77) = &input.name {
        object.key("name").string(var_77);
    }
    if let Some(var_78) = &input.source_server_id {
        object.key("sourceServerID").string(var_78);
    }
    if let Some(var_79) = &input.target_instance_type_right_sizing_method {
        object
            .key("targetInstanceTypeRightSizingMethod")
            .string(var_79.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_replication_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReplicationConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_80) = &input.associate_default_security_group {
        object.key("associateDefaultSecurityGroup").boolean(*var_80);
    }
    if input.bandwidth_throttling != 0 {
        object.key("bandwidthThrottling").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.bandwidth_throttling).into()),
        );
    }
    if let Some(var_81) = &input.create_public_ip {
        object.key("createPublicIP").boolean(*var_81);
    }
    if let Some(var_82) = &input.data_plane_routing {
        object.key("dataPlaneRouting").string(var_82.as_str());
    }
    if let Some(var_83) = &input.default_large_staging_disk_type {
        object
            .key("defaultLargeStagingDiskType")
            .string(var_83.as_str());
    }
    if let Some(var_84) = &input.ebs_encryption {
        object.key("ebsEncryption").string(var_84.as_str());
    }
    if let Some(var_85) = &input.ebs_encryption_key_arn {
        object.key("ebsEncryptionKeyArn").string(var_85);
    }
    if let Some(var_86) = &input.name {
        object.key("name").string(var_86);
    }
    if let Some(var_87) = &input.replicated_disks {
        let mut array_88 = object.key("replicatedDisks").start_array();
        for item_89 in var_87 {
            {
                let mut object_90 = array_88.value().start_object();
                crate::json_ser::serialize_structure_crate_model_replication_configuration_replicated_disk(&mut object_90, item_89)?;
                object_90.finish();
            }
        }
        array_88.finish();
    }
    if let Some(var_91) = &input.replication_server_instance_type {
        object.key("replicationServerInstanceType").string(var_91);
    }
    if let Some(var_92) = &input.replication_servers_security_groups_i_ds {
        let mut array_93 = object
            .key("replicationServersSecurityGroupsIDs")
            .start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94);
            }
        }
        array_93.finish();
    }
    if let Some(var_95) = &input.source_server_id {
        object.key("sourceServerID").string(var_95);
    }
    if let Some(var_96) = &input.staging_area_subnet_id {
        object.key("stagingAreaSubnetId").string(var_96);
    }
    if let Some(var_97) = &input.staging_area_tags {
        let mut object_98 = object.key("stagingAreaTags").start_object();
        for (key_99, value_100) in var_97 {
            {
                object_98.key(key_99).string(value_100);
            }
        }
        object_98.finish();
    }
    if let Some(var_101) = &input.use_dedicated_replication_server {
        object
            .key("useDedicatedReplicationServer")
            .boolean(*var_101);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_replication_configuration_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReplicationConfigurationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.arn {
        object.key("arn").string(var_102);
    }
    if let Some(var_103) = &input.associate_default_security_group {
        object
            .key("associateDefaultSecurityGroup")
            .boolean(*var_103);
    }
    if input.bandwidth_throttling != 0 {
        object.key("bandwidthThrottling").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.bandwidth_throttling).into()),
        );
    }
    if let Some(var_104) = &input.create_public_ip {
        object.key("createPublicIP").boolean(*var_104);
    }
    if let Some(var_105) = &input.data_plane_routing {
        object.key("dataPlaneRouting").string(var_105.as_str());
    }
    if let Some(var_106) = &input.default_large_staging_disk_type {
        object
            .key("defaultLargeStagingDiskType")
            .string(var_106.as_str());
    }
    if let Some(var_107) = &input.ebs_encryption {
        object.key("ebsEncryption").string(var_107.as_str());
    }
    if let Some(var_108) = &input.ebs_encryption_key_arn {
        object.key("ebsEncryptionKeyArn").string(var_108);
    }
    if let Some(var_109) = &input.replication_configuration_template_id {
        object
            .key("replicationConfigurationTemplateID")
            .string(var_109);
    }
    if let Some(var_110) = &input.replication_server_instance_type {
        object.key("replicationServerInstanceType").string(var_110);
    }
    if let Some(var_111) = &input.replication_servers_security_groups_i_ds {
        let mut array_112 = object
            .key("replicationServersSecurityGroupsIDs")
            .start_array();
        for item_113 in var_111 {
            {
                array_112.value().string(item_113);
            }
        }
        array_112.finish();
    }
    if let Some(var_114) = &input.staging_area_subnet_id {
        object.key("stagingAreaSubnetId").string(var_114);
    }
    if let Some(var_115) = &input.staging_area_tags {
        let mut object_116 = object.key("stagingAreaTags").start_object();
        for (key_117, value_118) in var_115 {
            {
                object_116.key(key_117).string(value_118);
            }
        }
        object_116.finish();
    }
    if let Some(var_119) = &input.use_dedicated_replication_server {
        object
            .key("useDedicatedReplicationServer")
            .boolean(*var_119);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_source_server_replication_type_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSourceServerReplicationTypeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_120) = &input.replication_type {
        object.key("replicationType").string(var_120.as_str());
    }
    if let Some(var_121) = &input.source_server_id {
        object.key("sourceServerID").string(var_121);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_change_server_life_cycle_state_source_server_lifecycle(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ChangeServerLifeCycleStateSourceServerLifecycle,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_122) = &input.state {
        object.key("state").string(var_122.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_describe_jobs_request_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DescribeJobsRequestFilters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.job_i_ds {
        let mut array_124 = object.key("jobIDs").start_array();
        for item_125 in var_123 {
            {
                array_124.value().string(item_125);
            }
        }
        array_124.finish();
    }
    if let Some(var_126) = &input.from_date {
        object.key("fromDate").string(var_126);
    }
    if let Some(var_127) = &input.to_date {
        object.key("toDate").string(var_127);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_describe_source_servers_request_filters(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DescribeSourceServersRequestFilters,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_128) = &input.source_server_i_ds {
        let mut array_129 = object.key("sourceServerIDs").start_array();
        for item_130 in var_128 {
            {
                array_129.value().string(item_130);
            }
        }
        array_129.finish();
    }
    if let Some(var_131) = &input.is_archived {
        object.key("isArchived").boolean(*var_131);
    }
    if let Some(var_132) = &input.replication_types {
        let mut array_133 = object.key("replicationTypes").start_array();
        for item_134 in var_132 {
            {
                array_133.value().string(item_134.as_str());
            }
        }
        array_133.finish();
    }
    if let Some(var_135) = &input.life_cycle_states {
        let mut array_136 = object.key("lifeCycleStates").start_array();
        for item_137 in var_135 {
            {
                array_136.value().string(item_137.as_str());
            }
        }
        array_136.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_licensing(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Licensing,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_138) = &input.os_byol {
        object.key("osByol").boolean(*var_138);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_replication_configuration_replicated_disk(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReplicationConfigurationReplicatedDisk,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_139) = &input.device_name {
        object.key("deviceName").string(var_139);
    }
    if let Some(var_140) = &input.is_boot_disk {
        object.key("isBootDisk").boolean(*var_140);
    }
    if let Some(var_141) = &input.staging_disk_type {
        object.key("stagingDiskType").string(var_141.as_str());
    }
    if input.iops != 0 {
        object.key("iops").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.iops).into()),
        );
    }
    Ok(())
}

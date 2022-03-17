// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn endpoint_resolver() -> impl aws_endpoint::ResolveAwsEndpoint {
    aws_endpoint::PartitionResolver::new(
        aws_endpoint::Partition::builder()
            .id("aws")
            .region_regex(r#"^(us|eu|ap|sa|ca|me|af)\-\w+\-\d+$"#)
            .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                uri_template: "s3-control.{region}.amazonaws.com",
                protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                credential_scope: aws_endpoint::CredentialScope::builder().build(),
            })
            .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
            .endpoint(
                "ap-northeast-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-northeast-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-northeast-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-northeast-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-2")
                        .build(),
                },
            )
            .endpoint(
                "ap-northeast-3",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-northeast-3.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-northeast-3")
                        .build(),
                },
            )
            .endpoint(
                "ap-south-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-south-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-south-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-southeast-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-1")
                        .build(),
                },
            )
            .endpoint(
                "ap-southeast-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ap-southeast-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ap-southeast-2")
                        .build(),
                },
            )
            .endpoint(
                "ca-central-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.ca-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ca-central-1")
                        .build(),
                },
            )
            .endpoint(
                "ca-central-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control-fips.ca-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("ca-central-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-central-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.eu-central-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-central-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-north-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.eu-north-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-north-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.eu-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-1")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.eu-west-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-2")
                        .build(),
                },
            )
            .endpoint(
                "eu-west-3",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.eu-west-3.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("eu-west-3")
                        .build(),
                },
            )
            .endpoint(
                "sa-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.sa-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("sa-east-1")
                        .build(),
                },
            )
            .endpoint(
                "us-east-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.us-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .endpoint(
                "us-east-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control-fips.us-east-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-1")
                        .build(),
                },
            )
            .endpoint(
                "us-east-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.us-east-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-2")
                        .build(),
                },
            )
            .endpoint(
                "us-east-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control-fips.us-east-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-east-2")
                        .build(),
                },
            )
            .endpoint(
                "us-west-1",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.us-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-1")
                        .build(),
                },
            )
            .endpoint(
                "us-west-1-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control-fips.us-west-1.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-1")
                        .build(),
                },
            )
            .endpoint(
                "us-west-2",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.us-west-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-2")
                        .build(),
                },
            )
            .endpoint(
                "us-west-2-fips",
                aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control-fips.us-west-2.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder()
                        .region("us-west-2")
                        .build(),
                },
            )
            .build()
            .expect("invalid partition"),
        vec![
            aws_endpoint::Partition::builder()
                .id("aws-cn")
                .region_regex(r#"^cn\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.{region}.amazonaws.com.cn",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .endpoint(
                    "cn-north-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control.cn-north-1.amazonaws.com.cn",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("cn-north-1")
                            .build(),
                    },
                )
                .endpoint(
                    "cn-northwest-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control.cn-northwest-1.amazonaws.com.cn",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("cn-northwest-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso")
                .region_regex(r#"^us\-iso\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.{region}.c2s.ic.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-iso-b")
                .region_regex(r#"^us\-isob\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.{region}.sc2s.sgov.gov",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .build()
                .expect("invalid partition"),
            aws_endpoint::Partition::builder()
                .id("aws-us-gov")
                .region_regex(r#"^us\-gov\-\w+\-\d+$"#)
                .default_endpoint(aws_endpoint::partition::endpoint::Metadata {
                    uri_template: "s3-control.{region}.amazonaws.com",
                    protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                    signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                    credential_scope: aws_endpoint::CredentialScope::builder().build(),
                })
                .regionalized(aws_endpoint::partition::Regionalized::Regionalized)
                .endpoint(
                    "us-gov-east-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control.us-gov-east-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-gov-east-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control-fips.us-gov-east-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-east-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-gov-west-1",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .endpoint(
                    "us-gov-west-1-fips",
                    aws_endpoint::partition::endpoint::Metadata {
                        uri_template: "s3-control-fips.us-gov-west-1.amazonaws.com",
                        protocol: aws_endpoint::partition::endpoint::Protocol::Https,
                        signature_versions: aws_endpoint::partition::endpoint::SignatureVersion::V4,
                        credential_scope: aws_endpoint::CredentialScope::builder()
                            .region("us-gov-west-1")
                            .build(),
                    },
                )
                .build()
                .expect("invalid partition"),
        ],
    )
}

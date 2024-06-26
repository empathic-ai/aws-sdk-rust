// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_signing_jobs_output_next_token(
    input: &crate::operation::list_signing_jobs::ListSigningJobsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_signing_platforms_output_next_token(
    input: &crate::operation::list_signing_platforms::ListSigningPlatformsOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_signing_profiles_output_next_token(
    input: &crate::operation::list_signing_profiles::ListSigningProfilesOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

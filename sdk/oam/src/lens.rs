// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_attached_links_output_next_token(
    input: &crate::operation::list_attached_links::ListAttachedLinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_links_output_next_token(
    input: &crate::operation::list_links::ListLinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn reflens_list_sinks_output_next_token(
    input: &crate::operation::list_sinks::ListSinksOutput,
) -> std::option::Option<&std::string::String> {
    let input = match &input.next_token {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_attached_links_output_items(
    input: crate::operation::list_attached_links::ListAttachedLinksOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ListAttachedLinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_links_output_items(
    input: crate::operation::list_links::ListLinksOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ListLinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

pub(crate) fn lens_list_sinks_output_items(
    input: crate::operation::list_sinks::ListSinksOutput,
) -> std::option::Option<std::vec::Vec<crate::types::ListSinksItem>> {
    let input = match input.items {
        None => return None,
        Some(t) => t,
    };
    Some(input)
}

use crate::utils;

pub fn rest_client_processor(
    input_file: &str,
    output_file: &str,
    url: &str,
) -> Result<(), std::io::Error> {
    let content = utils::detect_methods_in_file(input_file, url)?;

    utils::write_file(output_file, &content)
}

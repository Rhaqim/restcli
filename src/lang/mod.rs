pub mod golang;
pub mod javascript;
pub mod python;
pub mod rust;

pub const COMMON_REGEX: &str = r"(?P<method>GET|POST|PUT|DELETE|PATCH|HEAD|OPTIONS|CONNECT|TRACE)\s+(?P<path>\/[a-zA-Z0-9\/\-\_]+)";
pub const COMMON_REGEX_2: &str = r"\b(?i)(POST|GET|DELETE|PATCH|PUT)\b.*?(/[\w/]*)";
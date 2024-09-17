use std::collections::HashMap;

use regex::Regex;

use crate::utils;

pub fn express_method_endpoints(file_path: &str) -> HashMap<String, Vec<String>> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    let re = Regex::new(r#"\b(app\.get|app\.post|app\.delete|app\.patch|app\.put)\(\"([^\"]+)\"\)"#)
        .unwrap();

    utils::method_endpoints(re, &content, false)
}

/// JavaScript file
/// Express framework
/// method is in the form of app.get, app.post, app.delete, app.patch, app.put
/// Express framework Regex::new(r#"\b(app\.get|app\.post|app\.delete|app\.patch|app\.put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Koa framework
/// method is in the form of router.get, router.post, router.delete, router.patch, router.put
/// Koa framework Regex::new(r#"\b(router\.get|router\.post|router\.delete|router\.patch|router\.put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Hapi framework
/// method is in the form of server.route
/// Hapi framework Regex::new(r#"\b(server\.route\(\"([^\"]+)\"\)"#).unwrap();
///
/// Restify framework
/// method is in the form of server.get, server.post, server.del, server.patch, server.put
/// Restify framework Regex::new(r#"\b(server\.get|server\.post|server\.del|server\.patch|server\.put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// NestJS framework
/// method is in the form of @Get, @Post, @Delete, @Patch, @Put
/// NestJS framework Regex::new(r#"\b(@Get|@Post|@Delete|@Patch|@Put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// NextJS framework
/// method is in the form of function GET, function POST, function DELETE, function PATCH, function PUT
/// NextJS framework Regex::new(r#"\b(function GET|function POST|function DELETE|function PATCH|function PUT)\(\"([^\"]+)\"\)"#).unwrap();
pub fn detect_methods_in_js_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re =
        Regex::new(r#"\b(app\.get|app\.post|app\.delete|app\.patch|app\.put)\(\"([^\"]+)\"\)"#)
            .unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, "GET", endpoint);

        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}

/// Java file
/// Spring framework
/// method is in the form of @GetMapping, @PostMapping, @DeleteMapping, @PatchMapping, @PutMapping
/// Spring framework Regex::new(r#"\b(@GetMapping|@PostMapping|@DeleteMapping|@PatchMapping|@PutMapping)\(\"([^\"]+)\"\)"#).unwrap();
///
/// JAX-RS framework
/// method is in the form of @GET, @POST, @DELETE, @PATCH, @PUT
/// JAX-RS framework Regex::new(r#"\b(@GET|@POST|@DELETE|@PATCH|@PUT)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Spark framework
/// method is in the form of get, post, delete, patch, put
/// Spark framework Regex::new(r#"\b(get|post|delete|patch|put)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Play framework
/// method is in the form of GET, POST, DELETE, PATCH, PUT
/// Play framework Regex::new(r#"\b(GET|POST|DELETE|PATCH|PUT)\(\"([^\"]+)\"\)"#).unwrap();
///
/// Micronaut framework
/// method is in the form of @Get, @Post, @Delete, @Patch, @Put
/// Micronaut framework Regex::new(r#"\b(@Get|@Post|@Delete|@Patch|@Put)\(\"([^\"]+)\"\)"#).unwrap();
pub fn detect_methods_in_java_file(file_path: &str, url: &str) -> Result<String, std::io::Error> {
    let content = utils::read_file(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(
        r#"\b(@GetMapping|@PostMapping|@DeleteMapping|@PatchMapping|@PutMapping)\(\"([^\"]+)\"\)"#,
    )
    .unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
        let mut curl_command = format!("\n{} {}{} HTTP/1.1\n", url, "GET", endpoint);

        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "No methods found in file",
        ))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(result)
    }
}

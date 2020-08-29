use crate::controller::http_request::HttpRequest;
use crate::controller::http_response::HttpResponse;
use crate::controller::http_response_code::HttpResponseCode;
use crate::controller::http_response_code::HttpResponseCodeTypes;


use crate::user;
use crate::services::user_service;

use crate::helper_functions;

use std::fs;



pub fn new_user_get_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{
    let mut contents = fs::read_to_string("register.html").unwrap();

    let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200));

    let key = String::from("Content-Type");
    let value = String::from("text/html; charset=UTF-8");
    response.add_header(&key, &value);

    let redirect = request.get_param(&String::from("redirect_uri"));

    let client_url = match redirect{
        None => String::from(""),
        Some(value) => value.to_string()
    };

    // Update marked fields with specific configuration
    contents = contents.replace("${message}", "Log in to Trec Apps!")
        .replace("${clientUrl}", client_url.as_str()) // Set it to nothing here as User is logging on Directly, but is used if loggingin for a client
        .replace("\"${authUrl}\"", "http://localhost:7878");

    

    response.set_body(contents);

    response
}

pub fn user_exists_get_mapping(request: &HttpRequest, conn:&oracle::Connection) -> HttpResponse
{
    let mut endpoint = request.get_endpoint();

    endpoint = endpoint.replace("UserExists/", "").replace("/", "");

    // Endpoint should now equal the username being requested

    let exists_result = user::user_exists(&endpoint, conn);

    match exists_result
    {
        Err(err) => {
            let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ServerErr500));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);
        
            response.set_body(err);
            response

        },
        Ok(res) => {
            let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);
            if res
            {
                response.set_body(String::from("TRUE"));
            }
            else
            {
                response.set_body(String::from("FALSE"));
            }
            response
        }
    }
}

pub fn validate_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{
    let auth_opt = request.get_header(&String::from("Authorization"));

    if auth_opt.is_none()
    {
        // Here, the user is unauthenticated

        let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr401));
        let key = String::from("Content-Type");
        let value = String::from("text/plain; charset=UTF-8");
        response.add_header(&key, &value);

        response.set_body(String::from("User Not Recognized"));

        return response;
    }

    // To-Do: Validate JWT Token


    // To-Do: Validate verification


    // Generate HTML Response


    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

pub fn send_email_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{




    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

pub fn new_user_post_mapping(request: &HttpRequest, conn: &oracle::Connection, salt: &oracle::Connection) -> HttpResponse
{
    let body = request.get_body_string();

    let body_vec = helper_functions::split_string_n(&body, " ", 2);

    let fields_str = body_vec.get(0);



    match fields_str
    {
        None => {
            let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr400));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);
            response.set_body(String::from("Expected Body"));
            response
        },
        Some(fields) => {
            let field_map = helper_functions::get_map_from_string(fields, "&", "=");

            if field_map.is_some()
            {
                user_service::enter_new_user(&field_map.unwrap(), conn, salt)
            }
            else
            {
                let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ServerErr500));
                let key = String::from("Content-Type");
                let value = String::from("text/plain; charset=UTF-8");
                response.add_header(&key, &value);
                response.set_body(String::from("failed to generate mapt of parameters"));
                response
            }
        }
    }

    
}

pub fn log_in_post_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{

    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

pub fn log_in_get_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{

    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

pub fn update_user_get_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{

    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

pub fn update_user_post_mapping(request: &HttpRequest, conn: &oracle::Connection) -> HttpResponse
{

    HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200))
}

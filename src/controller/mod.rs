use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

pub mod http_request;
pub mod http_response;
pub mod http_response_code;

use http_request::HttpRequest;
use http_response::HttpResponse;
use http_response_code::HttpResponseCode;
use http_response_code::HttpResponseCodeTypes;

// Declare actual controllers
mod oauth_controller;
mod user_controller;


pub fn handle_request(mut stream: TcpStream, salt: &Option<oracle::Connection>, user: &Option<oracle::Connection>)
{
    let mut buffer : String = String::from("");

    println!("In handle_request, about to read the stream!");

    let mut byteBuffer = [0;512];

    loop {
        let bytes_read = stream.read(&mut byteBuffer).unwrap();
        if bytes_read == 0
        {
            break;
        }
        buffer.push_str(std::str::from_utf8(&mut byteBuffer).unwrap());

        if bytes_read < 512
        {
            break;
        }
    }
    println!("Read the Stream!");

    println!("\n-----------\n{}\n-------------\n", buffer);


    let request = http_request::HttpRequest::parse(&buffer);

    match request
    {
        Err(err_detail) => {
            let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr400));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);
            response.set_body(err_detail);

            response.write_response(&stream);
        },
        Ok(request_obj) =>
        {
            let response = sort_controller(&request_obj, &salt.as_ref().unwrap(), &user.as_ref().unwrap());

            response.write_response(&stream);
        }
    }

    stream.flush();
}

fn sort_controller(request: &HttpRequest, salt: &oracle::Connection, user: & oracle::Connection) -> HttpResponse
{
    let end_point = request.get_endpoint();
    let method = request.get_method();

    println!("Sorting controller with endpoint '{}'", end_point);

    if end_point.eq(&String::from("/NewUser"))
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::new_user_get_mapping(request, user)
        }
        else if method.eq(&String::from("POST"))
        {
            user_controller::new_user_post_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else if end_point.eq(&String::from("/Validate"))
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::validate_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else if end_point.eq(&String::from("/SendEmail"))
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::send_email_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else if end_point.eq(&String::from("/LogIn"))
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::log_in_get_mapping(request, user)
        }
        else if method.eq(&String::from("POST"))
        {
            user_controller::log_in_post_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else if end_point.eq(&String::from("/UpdateUser"))
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::update_user_get_mapping(request, user)
        }
        else if method.eq(&String::from("POST"))
        {
            user_controller::update_user_post_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else if end_point.starts_with("/UserExists/")
    {
        if method.eq(&String::from("GET"))
        {
            user_controller::user_exists_get_mapping(request, user)
        }
        else
        {
            HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
        }
    }
    else 
    {
        HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr404))
    }
}
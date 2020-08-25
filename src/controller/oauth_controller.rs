
use crate::controller::http_request::HttpRequest;
use crate::controller::http_response::HttpResponse;
use crate::controller::http_response_code::HttpResponseCode;
use crate::controller::http_response_code::HttpResponseCodeTypes;
use crate::client::OauthTwoClient;

use std::fs;

pub fn authorize_mapping(request: &HttpRequest, conn: &mut oracle::Connection) -> HttpResponse
{

    let opt_client_id = request.get_param(&String::from("client_id"));

    if opt_client_id.is_none()
    {
        let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr401));
        let key = String::from("Content-Type");
        let value = String::from("text/plain; charset=UTF-8");
        response.add_header(&key, &value);
        response.set_body(String::from("Client Id not provided!"));

        return response;
    }

    let client_id = opt_client_id.unwrap();

    if !OauthTwoClient::is_id_available(&client_id, conn)
    {
        let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr401));
        let key = String::from("Content-Type");
        let value = String::from("text/plain; charset=UTF-8");
        response.add_header(&key, &value);
        response.set_body(String::from("Client Id not recognized!"));

        return response;
    }

    let res_client = OauthTwoClient::get_client_by_id(client_id, conn);
    let re = String::from("redirect_uri");
    let redirect = request.get_param(&re);

    match res_client
    {
        Err(no_client) => 
        {
            let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ServerErr500));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);

            response.set_body(no_client);

            return response;
        },
        Ok(_) => {},
    }

    if redirect.is_none()
    {
        let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ClientErr400));
            let key = String::from("Content-Type");
            let value = String::from("text/plain; charset=UTF-8");
            response.add_header(&key, &value);

            response.set_body(String::from("No Redirect Url Provided"));

            return response;
    }


    let mut response = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200));

    let key = String::from("Content-Type");
    let value = String::from("text/html; charset=UTF-8");
    response.add_header(&key, &value);

    


    // To-Do: Write html file and read from it
    let mut contents = fs::read_to_string("register.html").unwrap();
        // Update marked fields with specific configuration
        contents = contents.replace("${message}", "Log in to Trec Apps!")
        .replace("${clientUrl}", redirect.unwrap()) // Set it to nothing here as User is logging on Directly, but is used if loggingin for a client
        .replace("\"${authUrl}\"", "http://localhost:7878");

    response.set_body(contents);



    response
}
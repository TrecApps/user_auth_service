use std::collections::HashMap;

use crate::helper_functions;

pub struct HttpRequest{
    method: String,
    endpoint: String,
    parameters: HashMap<String, String>,
    http_version: String,
    headers: HashMap<String, String>,
    body: String
}

static METHODS: &'static [&'static str] = &["GET","POST"];

fn is_method(method: &String) -> bool
{
    for supported_method in METHODS.iter()
    {
        if method.eq(supported_method)
        {
            return true;
        }
    }
    return false;
}

impl HttpRequest
{
    pub fn get_param(&self, key: &String) -> Option<&String>
    {
        self.parameters.get(key)
    }

    pub fn get_endpoint(&self) -> String
    {
        self.endpoint.clone()
    }

    pub fn get_method(&self) -> String
    {
        self.method.clone()
    }

    pub fn get_header(&self, key: &String) -> Option<&String>
    {
        self.headers.get(key)
    }

    pub fn parse(contents: &String) -> Result<HttpRequest, String>
    {
        let first_vec = helper_functions::split_string_n(contents, " ", 4);


        if first_vec.len() != 4
        {
            return Err(String::from("Improperly formed http request, attempt to parse the first line did not yield required attributes"));
        }

        let method = first_vec.get(0).expect("Call to len did not work as expected");

        if !is_method(method)
        {
            return Err(String::from("Method Specified does not match our methods"));
        }

        let mappings = first_vec.get(1).expect("msg: &str");

        let mapping_vec = helper_functions::split_string_n(mappings, "?", 1);

        let endpoint_opt = mapping_vec.get(0);

        let mut endpoint = String::from("");

        match endpoint_opt
        {
            None => {return Err(String::from("Improperly set of parameters in http resquest"));},
            Some(end) => endpoint = String::from(end)
        }

        let empty_string = String::from("");

        let param_results = helper_functions::get_map_from_string(
            match mapping_vec.get(1)
            {
               None => &empty_string,
               Some(value) => value 
            }, "&", "=");

        let parameters :HashMap::<String, String>;

        match param_results
        {
            None => {return Err(String::from("Improperly set of parameters in http resquest"));},
            Some(value) => parameters = value
        }

        let version = first_vec.get(2).expect("msg: &str");
        
        let headers_body = helper_functions::split_string_n(first_vec.get(3).unwrap(), "\n\n", 2);

        let header_string = match headers_body.get(0)
        {
            None => String::from(""),
            Some(string) => string.clone()
        };

        let headers_opt = helper_functions::get_map_from_string(&header_string, " ", ":");

        if headers_opt.is_none()
        {
            return Err(String::from("Improperly set of headers in http resquest"));
        }

        let headers = headers_opt.unwrap();

        let body = match headers_body.get(1)
        {
            None => String::from(""),
            Some(b) => b.clone()
        };

        Ok(
            HttpRequest{
                method: String::from(method),
                endpoint,
                parameters,
                http_version: String::from(version),
                headers,
                body
            }
        )
    }
}
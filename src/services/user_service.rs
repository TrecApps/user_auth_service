
use std::collections::HashMap;
use crate::controller::http_response;
use crate::controller::http_response::HttpResponse;
use crate::controller::http_response_code::HttpResponseCode;
use crate::controller::http_response_code::HttpResponseCodeTypes;

use oracle::Connection;

use rand::Rng;

use dumb_crypto::scrypt::Scrypt;
use dumb_crypto::scrypt::ScryptError;
use passwords::hasher;

use crate::user;


static CHAR_ARRAY : [char; 62] = ['a', 'b', 'c','d','e','f','g','h','i','j','k','l',
    'm','n','o','p','q','r','s','t','u','v','w','x',
    'y','z','A','B','C','D','E','F','G','H','I','J',
    'K','L','M','N','O','P','Q','R','S','T','U','V',
    'W','X','Y','Z','0','1','2','3','4','5','6','7',
    '8','9'];

fn generate_random_string(length: usize) -> String
{
    let mut ret = String::from("");

    for num in 0..length
    {
        let random_char = CHAR_ARRAY[rand::thread_rng().gen_range(0, 62)];

        ret.push(random_char);
    }

    ret
}

pub fn enter_new_user(field_map: &HashMap<String, String>, conn: &oracle::Connection, salt: &oracle::Connection) -> HttpResponse
{
    let username_opt = field_map.get(&String::from("username"));

    if username_opt.is_none()
    {
        return http_response::get_client_respone_using_text(String::from("Needed username field in request"));
    }

    let email_opt = field_map.get(&String::from("email"));

    if email_opt.is_none()
    {
        return http_response::get_client_respone_using_text(String::from("Needed email field in request"));
    }

    let password_opt = field_map.get(&String::from("password"));

    if password_opt.is_none()
    {
        return http_response::get_client_respone_using_text(String::from("Needed password field in request"));
    }

    let password_reset_default = String::from("0");
    let inactivity_count_default = String::from("3");
    let max_logins_default = String::from("10");
    let lock_time_default = String::from("6");

    let password_reset = field_map.get(&String::from("password_reset")).unwrap_or(&password_reset_default);

    let inactivity_count = field_map.get(&String::from("inactivity_count")).unwrap_or(&inactivity_count_default);

    let max_logins = field_map.get(&String::from("max_logins")).unwrap_or(&max_logins_default);

    let lock_time = field_map.get(&String::from("lock_time")).unwrap_or(&lock_time_default);

    // Get the actual fields
    let password = password_opt.unwrap();

    let username = username_opt.unwrap();

    let email = email_opt.unwrap().replace("%40", "@");

    let raw_password_reset_res = password_reset.parse::<u8>();
    let raw_inactivity_count_res = inactivity_count.parse::<u8>();
    let raw_max_logins_res = max_logins.parse::<u8>();
    let raw_lock_time_res = lock_time.parse::<u8>();

    if raw_inactivity_count_res.is_err() || raw_lock_time_res.is_err() || raw_max_logins_res.is_err() || raw_password_reset_res.is_err()
    {
        return http_response::get_client_respone_using_text(String::from("One field was not a proper number"));
    }


    let raw_password_reset = raw_password_reset_res.unwrap();
    let raw_max_logins = raw_max_logins_res.unwrap();
    let raw_lock_time = raw_lock_time_res.unwrap();
    let raw_inactivity_count = raw_inactivity_count_res.unwrap();

    let b_salt = generate_random_string(32);
    let s_salt = generate_random_string(64);

    let b_password = get_bcrypt_password(password.to_string(), b_salt.to_string());
    let s_password = get_scrypt_password(password.to_string(), s_salt.to_string());

    if b_password.is_err()
    {
        return http_response::get_client_respone_using_text(b_password.unwrap_err());
    }

    if s_password.is_err()
    {
        return http_response::get_client_respone_using_text(s_password.unwrap_err());
    }

    let available_id = user::User::get_avaiable_id(conn);

    if available_id.is_err()
    {
        return http_response::get_client_respone_using_text(available_id.unwrap_err());
    }

    let mut new_user = user::User::new_secure(username.to_string(),
        email,
        String::from(""),
        s_password.unwrap(),
        b_password.unwrap(),
        0,
        String::from(""),
        0,
        raw_password_reset,
        raw_inactivity_count,
        false,
        raw_max_logins,
        raw_lock_time
    );
    let user_id = available_id.unwrap();
    new_user.set_id(user_id);

    if new_user.insert_new_user(conn)
    {
        // Table USER_SALTS: ID, S_SALT, B_SALT

        // To-Do: Add Salts to other table
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let mut insert_salt = 
            salt.prepare("insert into USER_SALTS (ID, S_SALT, B_SALT) values
            (:id, :s_salt, :b_salt)", &params).
            expect("Failed to create new salt insert sttement!");
        
        let insert_result = insert_salt.execute_named(&[("id", &user_id),
                ("s_salt", &s_salt),("b_salt", &b_salt)]);


        // To-Do: figure out the appropriate success response
        match insert_result
        {
            Ok(_) => HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::Success200)),
            Err(_) => {
                let mut res = HttpResponse::new(HttpResponseCode::new(HttpResponseCodeTypes::ServerErr500));
                let key = String::from("Content-Type");
                let value = String::from("text/plain; charset=UTF-8");
                res.add_header(&key, &value);
                res.set_body(String::from("Failed to Insert Salt Data to DB!"));
                res
            }
        }
    }
    else
    {
        http_response::get_client_respone_using_text(String::from("Failed to insert user to database"))
    }
}

fn get_scrypt_password(password: String, salt_str: String) -> Result<String, String>
{
    let scrypt = Scrypt::new(2, 256,2);

    let mut buff = [0;80];

    let res = scrypt.derive(password.as_bytes(), salt_str.as_bytes(), &mut buff);

    match res
    {
        Err(res_err) => {
            match res_err
            {
                ScryptError::RIsTooSmall => Err(String::from("RIsTooSmall")),
                ScryptError::NIsTooSmall => Err(String::from("NIsTooSmall")),
                ScryptError::NIsNotAPowerOfTwo => Err(String::from("NIsNotAPowerOfTwo")),
                ScryptError::PIsTooSmall =>Err(String::from("PIsTooSmall"))
            }
        },
        Ok(_) => {
            Ok(String::from_utf8_lossy(&buff).to_string())
        }
    }
}

fn get_bcrypt_password(password: String, salt_str: String) -> Result<String, String>
{
    let res = hasher::bcrypt(10, &salt_str, password.as_str());

    match res
    {
        Err(err_obj) => {
            Err(String::from(err_obj))
        },
        Ok(ok_obj) => {
            Ok(String::from_utf8_lossy(&ok_obj).to_string())
        }
    }
}
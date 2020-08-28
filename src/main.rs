extern crate threadpool;
extern crate oracle;

extern crate rand;

use std::net::TcpListener;
use std::env;

mod controller;
mod user;
mod client;
mod services;

pub mod err_string;

pub mod jwt_process;

mod helper_functions;

static mut MAIN_CONNECTION : Option<oracle::Connection> = None;
static mut SALT_CONNECTION : Option<oracle::Connection> = None;

fn main() {
    
    // Create oracle connections

    // First, retrieve the parameters from the Environment
    let db_url = env::var("DB_URL").expect("Environment Variable 'DB_URL' not set!");
    let db_url_2 = env::var("DB2_URL").expect("Environment Variable 'DB2_URL' not set!");

    let db_username = env::var("DB_USERNAME").expect("Environment Variable 'DB_USERNAME' not set!");
    let db_username_2 = env::var("DB2_USERNAME").expect("Environment Variable 'DB2_USERNAME' not set!");

    let db_password = env::var("DB_PASSWORD").expect("Environment Variable 'DB_PASSWORD' not set!");
    let db_password_2 = env::var("DB2_PASSWORD").expect("Environment Variable 'DB2_PASSWORD' not set!");

    unsafe
    {
    MAIN_CONNECTION = Some(oracle::Connection::connect(db_username, db_password, db_url).expect("Failed to Connect to the main Oracle Database"));

    SALT_CONNECTION = Some(oracle::Connection::connect(db_username_2, db_password_2, db_url_2)
        .expect("Failed to connect to the salt Oracle Database"));

    }
    // Retrieve a Listener that will listen on localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    

    // Get a thread-pool to pass requests off to
    let pool = threadpool::ThreadPool::new(4);

    println!("Created Threadpool, preparing to process requests!");

    // For every request made, send it to a thread under the handle_connection function
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Unwrapped the stream!");

        //stream.set_nonblocking(true).expect("set_nonblocking call failed");

        unsafe{
            pool.execute(||{
                controller::handle_request(stream, &SALT_CONNECTION, &MAIN_CONNECTION);
            });
        }   
    }

    pool.join();
}

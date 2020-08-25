extern crate threadpool;
extern crate oracle;

use std::net::TcpListener;

mod controller;
mod user;
mod client;

pub mod err_string;

pub mod jwt_process;

mod helper_functions;

static mut MAIN_CONNECTION : Option<oracle::Connection> = None;
static mut SALT_CONNECTION : Option<oracle::Connection> = None;

fn main() {
    
    // Create oracle connections

    unsafe
    {
    MAIN_CONNECTION = Some(oracle::Connection::connect("", "", "").expect("Failed to Connect to the main Oracle Database"));

    SALT_CONNECTION = Some(oracle::Connection::connect("username: U", "password: P", "connect_string: C")
        .expect("Failed to connect to the salt Oracle Database"));

    }
    // Retrieve a Listener that will listen on localhost:7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Get a thread-pool to pass requests off to
    let pool = threadpool::ThreadPool::new(4);

    // For every request made, send it to a thread under the handle_connection function
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        unsafe{
            pool.execute(||{
                controller::handle_request(stream, &SALT_CONNECTION, &MAIN_CONNECTION);
            });
        }   
    }

    pool.join();
}

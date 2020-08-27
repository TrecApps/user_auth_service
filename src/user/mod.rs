mod security_policy;

use oracle::{Connection, ResultSet, Row, Statement};

pub struct User
{
    id: usize,
    username: String,
    main_email: String,
    trec_email: String,
    s_password: String,
    b_password: String,
    unlock_time: u64,   // Time when the account can be unlocked (consider it unlocked if current time is higher)
    verify_code: String,
    verify_time: u64,   // if this is 0, consider the account verified

    // Levels of security to apply to this account
     password_month_reset: u8,          // how many months before a user needs to reset his/her password
     client_time_restrict: u8,          // Time, in 10 minutes, before the token should expire
     client_time_from_activity: bool,   // whether to update the token expiration (true) or not (false)
     max_login_attempts_per_hour: u8,   // How often to attempt a login before the account is locked
     login_lock_time: u8                // Time, in 10 minutes, to lock an account after max_login_attempts 
}

pub fn user_exists(username: &String, conn: &oracle::Connection) -> Result<bool, String>
{
    let params = [oracle::StmtParam::FetchArraySize(1)];
    let prepare = conn.prepare("select * from users where username = :username", &params);

    if prepare.is_err()
    {
        return Err(String::from("Failed to Prepare a query for retrieving user id!"));
    }

    let mut query = prepare.ok().expect("");
    let result = query.query_row_named(&[("username", username)]);

    if result.is_err()
    {
        Ok(false)
    }
    else
    {
        Ok(true)
    }
}

impl User
{
    fn new(username:String, main_email: String, trec_email: String, s_password: String,
         b_password: String, unlock_time: u64,verify_code: String, verify_time: u64) -> User
    {
        User {
            id:0,
            username,
            main_email,
            trec_email,
            s_password,
            b_password,
            unlock_time,
            verify_time,
            verify_code,
            password_month_reset: 0,
            client_time_restrict: 3,
            client_time_from_activity: true,
            max_login_attempts_per_hour: 10,
            login_lock_time: 5
        }
    }

    fn new_secure(username:String, main_email: String, trec_email: String, s_password: String,
        b_password: String, unlock_time: u64,verify_code: String, verify_time: u64,
        password_month_reset: u8, client_time_restrict: u8, client_time_from_activity: bool,
        max_login_attempts_per_hour: u8, login_lock_time: u8) -> User
   {
       User {
           id:0,
           username,
           main_email,
           trec_email,
           s_password,
           b_password,
           unlock_time,
           verify_time,
           verify_code,
           password_month_reset,
           client_time_restrict,
           client_time_from_activity,
           max_login_attempts_per_hour,
           login_lock_time
       }
   }

    fn get_user_by_id(conn: & oracle::Connection, id: usize) -> Result<User, String>
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let prepare = conn.prepare("select * from users where id = :id", &params);

        if prepare.is_err()
        {
            return Err(String::from("Failed to Prepare a query for retrieving user id!"));
        }

        let mut query = prepare.ok().expect("");
        let result = query.query_row_named(&[("id", &id)]);

        if result.is_err()
        {
            return Err(String::from("No User exists for provided id"));
        }

        let set = result.expect("msg: &str");
        Ok(
            User {
                id,
                username: set.get::<usize, String>(1).expect("db user username"),
                main_email: set.get::<usize, String>(2).expect("db user main_email"),
                trec_email: set.get::<usize, String>(3).expect("db user trec_email"),
                s_password: set.get::<usize, String>(4).expect("db user s_password"),
                b_password: set.get::<usize, String>(5).expect("db user b_password"),
                unlock_time: set.get::<usize, usize>(6).expect("db user unlock_time") as u64,
                verify_time: set.get::<usize, usize>(7).expect("db user verify_time") as u64,
                verify_code: set.get::<usize, String>(8).expect("db user verify_code"),
                password_month_reset: set.get::<usize, usize>(9).expect("db user verify_code") as u8,
                client_time_restrict: set.get::<usize, usize>(10).expect("db user verify_code") as u8,
                client_time_from_activity: if set.get::<usize, usize>(11).expect("db user verify_code") > 0  {true}else {false},
                max_login_attempts_per_hour: set.get::<usize, usize>(12).expect("db user verify_code") as u8,
                login_lock_time: set.get::<usize, usize>(13).expect("db user verify_code") as u8
            }
        )
    }

    fn update_user(&self, conn: &oracle::Connection) -> bool
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];

        let mut update_statement = 
            conn.prepare("Update users set
                username = :username,
                main_email = :main_email, 
                trec_email = trec_email 
                s_password = :s_password, 
                b_password = :b_password, 
                unlock_time = :unlock_time, 
                verify_code = :verify_code, 
                verify_time = :verify_time,
                password_month_reset = :password_month_reset, 
                client_time_restrict = :client_time_restrict, 
                client_time_from_activity = :client_time_from_activity, 
                max_login_attempts_per_hour = :max_login_attempts_per_hour, 
                login_lock_time = :login_lock_time
                where id = :id", &params).expect("Failed to prepare update statement for user!");

        let update_result = update_statement.execute_named(&[("username",&self.username),
        ("main_email",&self.main_email),
        ("trec_email",&self.trec_email),
        ("s_password",&self.s_password),
        ("b_password",&self.b_password),
        ("unlock_time",&self.unlock_time),
        ("verify_code",&self.verify_code),
        ("verify_time",&self.verify_time),
        ("password_month_reset",&self.password_month_reset),
        ("client_time_restrict",&self.client_time_restrict),
        ("client_time_from_activity",&self.client_time_from_activity),
        ("max_login_attempts_per_hour",&self.max_login_attempts_per_hour),
        ("login_lock_time",&self.login_lock_time),
        ("id", &self.id)
        ]);

        match update_result
        {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn insert_new_user(&self, conn: &oracle::Connection) -> bool
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let mut insert_statement =
            conn.prepare("insert into users (id, username, main_email, trec_email, s_password, b_password, unlock_time, verify_code, verify_time,
            password_month_reset, client_time_restrict, client_time_from_activity, max_login_attempts_per_hour, login_lock_time) values (
                :username, :main_email, :trec_email, :s_password, :b_password, :unlock_time, :verify_code, :verify_time,
                :password_month_reset, :client_time_restrict, :client_time_from_activity, :max_login_attempts_per_hour, :login_lock_time
            )", &params).expect("Failed to prepare insert statement for user!");

        let insert_result = insert_statement.execute_named(&[("id", &self.id),
                                        ("username",&self.username),
                                        ("main_email",&self.main_email),
                                        ("trec_email",&self.trec_email),
                                        ("s_password",&self.s_password),
                                        ("b_password",&self.b_password),
                                        ("unlock_time",&self.unlock_time),
                                        ("verify_code",&self.verify_code),
                                        ("verify_time",&self.verify_time),
                                        ("password_month_reset",&self.password_month_reset),
                                        ("client_time_restrict",&self.client_time_restrict),
                                        ("client_time_from_activity",&self.client_time_from_activity),
                                        ("max_login_attempts_per_hour",&self.max_login_attempts_per_hour),
                                        ("login_lock_time",&self.login_lock_time)
                                        ]);

        match insert_result
        {
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn get_avaiable_id(conn: &oracle::Connection) -> Result<usize, String>
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let prepare = conn.prepare("select max(id) from users", &params);

        if prepare.is_err()
        {
            return Err(String::from("Failed to Prepare a query for a user id!"));
        }

        let mut query = prepare.ok().expect("");
        let result = query.query(&[]);

        if result.is_err()
        {
            return Err(String::from("Failed to Execute a query for a user id!"));
        }

        let set = result.expect("msg: &str");
        let mut rows = Vec::<usize>::new();
        for row in set
        {
            let value = row.expect("Not proper row").get_as::<usize>().expect("value");
            rows.push(value);
        }

        match rows.first()
        {
            None => Err(String::from("Failed to execute query for a user id")),
            Some(value) => {
                if *value == std::usize::MAX
                {
                    Err(String::from("Reached max that a user id could be!"))
                }
                else
                {
                    Ok(value + 1)
                }
            }
        }

    }
}
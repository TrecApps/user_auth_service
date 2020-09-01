use oracle;
use mysql;

pub enum DbDriverType
{
    driver_oracle,
    driver_mysql
}

enum DbDriverObj
{
    driver_failed,
    driver_oracle { conn: oracle::Connection },
    driver_my_sql { conn: mysql::PooledConn }
}


pub struct DbDriver
{
    driver: DbDriverObj
}

impl DbDriver
{
    pub fn new(user: String, pass: String, url: String, db_type: DbDriverType) -> DbDriver
    {
        match db_type
        {
            DbDriverType::driver_oracle =>
            {
                let o_driver = oracle::Connection::connect(user, pass, url);

                match o_driver
                {
                    Err(_) => DbDriver{ driver: DbDriverObj::driver_failed },
                    Ok(c) => {
                        DbDriver{ driver: DbDriverObj::driver_oracle{conn: c} }
                    }
                }
            },
            DbDriverType::driver_mysql =>
            {
                let my_url = format!("mysql://{}:{}@{}", user, pass, url);

                let mysql_pool = mysql::Pool::new(my_url);

                let s_driver = mysql_pool.get_conn();

                match s_driver
                {
                    Err(_) => DbDriver{ driver: DbDriverObj::driver_failed },
                    Ok(c) => {
                        DbDriver{ driver: DbDriverObj::driver_my_sql{conn: c} }
                    }
                }
            }
        }
    }

    
}
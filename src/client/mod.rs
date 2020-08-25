
pub enum OauthTwoClientType
{
    ClientTypeWebService{client_secret: String},
    ClientTypeSinglePage,
    ClientTypeNativeApp,
    ClientTypeMobileApp
}

pub struct OauthTwoClient
{
    name: String,
    client_id: String,
    client_type: OauthTwoClientType,
    resources: String,
    scope: String
}

impl OauthTwoClient
{
    pub fn get_client_by_id(id: &String, conn: &oracle::Connection) -> Result<OauthTwoClient, String>
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let prepare = conn.prepare("select * from oauth_clients where id = :id", &params);

        if prepare.is_err()
        {
            return Err(String::from("Failed to Prepare a query for retrieving user id!"));
        }

        let mut query = prepare.ok().expect("");
        let result = query.query_row_named(&[("id", id)]);

        if result.is_err()
        {
            return Err(String::from("No User exists for provided id"));
        }

        let set = result.expect("msg: &str");

        let client_secret = set.get::<usize, String>(2).expect("failed to retrieve client secret field!");


        let client_type = match set.get::<usize, usize>(3).expect("failed to retrieve client type field")
        {
            0 => OauthTwoClientType::ClientTypeWebService{client_secret},
            1 => OauthTwoClientType::ClientTypeSinglePage,
            2 => OauthTwoClientType::ClientTypeNativeApp,
            3 => OauthTwoClientType::ClientTypeMobileApp,
            _ => {return Err(String::from("Error, unusual type value found for client"));}
        };



        Ok(
            OauthTwoClient{
                client_id: id.to_string(),
                name: set.get::<usize, String>(1).expect("failed to retrieve client name field!"),
                resources: set.get::<usize, String>(4).expect("failed to retrieve client resources field!"),
                scope: set.get::<usize, String>(5).expect("failed to retrieve client scope field!"),
                client_type
            }
        )
    }

    pub fn insert_new_client(&self, conn: &oracle::Connection) -> bool
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let mut insert_statement =
            conn.prepare("insert into oauth_clients (client_id, name, client_secret, client_type, resources, scope) values
                (:client_id, :name, :client_secret, :client_type, :resources, :scope)", &params).expect("msg: &str");

        let empty_string = String::from("");
        
        let (client_type, client_secret_) = match &self.client_type
        {
            OauthTwoClientType::ClientTypeWebService{client_secret} => (0, client_secret),
            OauthTwoClientType::ClientTypeSinglePage => (1, &empty_string),
            OauthTwoClientType::ClientTypeNativeApp => (2, &empty_string),
            OauthTwoClientType::ClientTypeMobileApp => (3, &empty_string)
        };

        let insert_result = insert_statement.execute_named(&[("client_id",&self.client_id),
                                                            ("name", &self.name),
                                                            ("client_secret",client_secret_),
                                                            ("client_type",&client_type),
                                                            ("resources",&self.resources),
                                                            ("scope",&self.scope)]);

        match insert_result
        {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn is_id_available(id: &String, conn: &oracle::Connection) -> bool
    {
        let params = [oracle::StmtParam::FetchArraySize(1)];
        let prepare = conn.prepare("select client_id from oauth_clients where client_id = :id", &params);

        let mut query = prepare.ok().expect("Failed to prepare query for client id");
        let result = query.query_named(&[("id", id)]).expect("Failed to query for client id");

        let mut vec_result = Vec::<usize>::new();

        for _ in result
        {
            vec_result.push(0);
        }

        vec_result.len() == 0
    }

    pub fn does_secret_match(&self, secret:&String) -> bool
    {
        match &self.client_type
        {
            OauthTwoClientType::ClientTypeWebService{client_secret} => client_secret.eq(secret),
            _ => false
        }
    }

    pub fn new(name: String, client_id: String, client_type: OauthTwoClientType, resources:String, scope:String)-> OauthTwoClient
    {
        OauthTwoClient{
            name,
            client_id,
            client_type,
            resources,
            scope
        }
    }
}
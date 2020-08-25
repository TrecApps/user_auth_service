/**
 * security_policy.rs
 * This File provides resources through which users can control how their accounts
 * are protected. Such strategies can range from how often they need to reset their passwords to 
 * how heavily hashed their passwords are, or how private they wish their information to be
 */

 pub struct SecurityPolicy
 {
     password_month_reset: u8,          // how many months before a user needs to reset his/her password
     client_time_restrict: u8,          // Time, in 10 minutes, before the token should expire
     client_time_from_activity: bool,   // whether to update the token expiration (true) or not (false)
     max_login_attempts_per_hour: u8,   // How often to attempt a login before the account is locked
     login_lock_time: u8                // Time, in 10 minutes, to lock an account after max_login_attempts 
 }
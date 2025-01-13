use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use scylla::{QueryRowsResult, Session};
use uuid::Uuid;
use anyhow::{Error, Result};
use chrono::NaiveDate;
use crate::security::passwords::{hash_password};

const ALLOWED_UPDATE_FIELDS: [&str; 7] = ["email", "password", "displayname", "status", "bannercolor", "backgroundcolor", "desc"];
const ALLOWED_STATUS: [u8; 4] = [0, 1, 2, 3];

#[derive(Debug)]
pub struct User {
    pub createdat: Option<NaiveDate>,
    pub user_id: Option<Uuid>,
    pub jwt: Option<Uuid>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub displayname: Option<String>,
    pub friends: Option<HashMap<Uuid, i8>>,
    pub badges: Option<Vec<Uuid>>,
    pub status: Option<i8>,
    pub bannercolor: Option<String>,
    pub backgroundcolor: Option<String>,
    pub isadmin: Option<bool>,
    pub desc: Option<String>,
}

// User implementation of functions that return user objects from accessible data

// Public trait UserFunc for User struct functions
pub  trait UserFunc: std::marker::Sized{
    async fn fill_info(self, session: &Arc<Session>) -> Result<Self>;
    async fn fetch_friends(self, session: &Arc<Session>) -> Result<Self>;
    async fn update(self, session: &Arc<Session>, change_field: &str, new_value: String) -> Result<Self>;
}
impl UserFunc for User {

    /// Filles up info about user besed on user id/email/jwt
    async fn fill_info(mut self, session: &Arc<Session>) -> Result<Self> {
        let res: QueryRowsResult;
        // Fetch data if ID is present
        if let Some(user_id) = self.user_id{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor, isadmin, desc FROM joltamp.users WHERE user_id = ? ALLOW FILTERING",
                                        (&user_id, )).await?.into_rows_result()?;
        }
        // Fetch data if JWT is present
        else if let Some(jwt) = self.jwt{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor, isadmin, desc FROM joltamp.users WHERE jwt = ? ALLOW FILTERING",
                                        (&jwt, )).await?.into_rows_result()?;
        }
        // Fetch data if Email is present
        else if let Some(email) = self.email{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor, isadmin, desc FROM joltamp.users WHERE email = ? ALLOW FILTERING",
                                        (&email, )).await?.into_rows_result()?;
        }
        // Return error if no data is provided
        else {
            return Err(Error::msg("Invalid"));
        }
        let (createdat, user_id, jwt, username, email, password, displayname, raw_friends, badges, status, bannercolor, backgroundcolor, isadmin, desc)
            = res.first_row::<(NaiveDate, Uuid, Uuid, String, String, String, String, HashMap<Uuid, i8>, Vec<Uuid>, i8, Option<String>, Option<String>, Option<bool>, Option<String>)>()?;

        self.createdat = Some(createdat);
        self.user_id = Some(user_id);
        self.jwt = Some(jwt);
        self.username = Some(username);
        self.email = Some(email);
        self.password = Some(password);
        self.displayname = Some(displayname);
        self.friends = Some(raw_friends);
        self.badges = Some(badges);
        self.status = Some(status);
        self.bannercolor = bannercolor;
        self.backgroundcolor = backgroundcolor;
        self.isadmin = isadmin;
        self.desc = desc;

        Ok(self)
    }
    async fn fetch_friends(self, session: &Arc<Session>) -> Result<Self> {
        Ok(self)
    }

    async fn update(mut self, session: &Arc<Session>, change_field: &str, mut new_value: String) -> Result<Self> {
        if let None = self.jwt {
            return Err(Error::msg("JWT is not set"));
        }

        println!("{}, {}", change_field, new_value);

        if ALLOWED_UPDATE_FIELDS.contains(&change_field) {
            if change_field == "password"{
                new_value = hash_password(&mut new_value).unwrap();
            }
            if change_field == "status" && !ALLOWED_STATUS.contains(&new_value.parse::<u8>()?) {
                return Err(Error::msg("Not allowed status!"));
            }
            if change_field == "email" && (!new_value.contains("@") || new_value.len() < 3){
                return Err(Error::msg("Invalid email"));
            }
            let res = session.query_unpaged(format!("UPDATE joltamp.users SET {} = ? WHERE username = ? AND user_id = ? AND createdat = ?", &change_field),
                                            (&new_value, &self.username, &self.user_id, &self.createdat)).await;
            if let Ok(_) = res {
                match change_field {
                    "email" => self.email = Some(new_value.to_string()),
                    "password" => self.password = Some(new_value.to_string()),
                    "displayname" => self.displayname = Some(new_value.to_string()),
                    "status" => self.status = Some(new_value.parse::<i8>()?),
                    "bannercolor" => self.bannercolor = Some(new_value.to_string()),
                    "backgroundcolor" => self.backgroundcolor = Some(new_value.to_string()),
                    _ => {
                        return Err(Error::msg("Action not allowed"));
                    }
                }
                Ok(self)
            }else{
                return Err(Error::msg("Update failed"));
            }
        }else{
            Err(Error::msg("Field not allowed"))
        }
    }
}
impl User {

    /// Creates user object from user id
    pub fn from_user_id(user_id: Uuid) -> User {
        User {
            createdat: None,
            user_id: Some(user_id),
            jwt: None,
            username: None,
            email: None,
            password: None,
            displayname: None,
            friends: None,
            badges: None,
            status: None,
            bannercolor: None,
            backgroundcolor: None,
            isadmin: None,
            desc: None,
        }
    }
    /// Creates user object from user jwt
    pub fn from_user_jwt(jwt: Uuid) -> User {
        User {
            createdat: None,
            user_id: None,
            jwt: Some(jwt),
            username: None,
            email: None,
            password: None,
            displayname: None,
            friends: None,
            badges: None,
            status: None,
            bannercolor: None,
            backgroundcolor: None,
            isadmin: None,
            desc: None,
        }
    }

    /// Creates user object from user email
    pub fn from_user_email(email: String) -> User {
        User {
            createdat: None,
            user_id: None,
            jwt: None,
            username: None,
            email: Some(email),
            password: None,
            displayname: None,
            friends: None,
            badges: None,
            status: None,
            bannercolor: None,
            backgroundcolor: None,
            isadmin: None,
            desc: None,
        }
    }
}
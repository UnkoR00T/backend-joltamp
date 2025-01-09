use std::collections::HashMap;
use std::sync::Arc;
use scylla::{QueryRowsResult, Session};
use uuid::Uuid;
use anyhow::{Error, Result};
use scylla::frame::value::CqlDate;

#[derive(Debug)]
pub struct User {
    pub createdat: Option<CqlDate>,
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
}

// User implementation of functions that return user objects from accessible data
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
        }
    }
}

// Public trait UserFunc for User struct functions
pub  trait UserFunc: std::marker::Sized{
    async fn fill_info(self, session: &Arc<Session>) -> Result<Self>;
}
impl UserFunc for User {
    
    /// Filles up info about user besed on user id/email/jwt
    async fn fill_info(mut self, session: &Arc<Session>) -> Result<Self> {
        let mut res: QueryRowsResult;
        if let Some(user_id) = self.user_id{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor FROM joltamp.users WHERE user_id = ? ALLOW FILTERING",
                                        (&user_id, )).await?.into_rows_result()?;
        }else if let Some(jwt) = self.jwt{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor FROM joltamp.users WHERE jwt = ? ALLOW FILTERING",
                                        (&jwt, )).await?.into_rows_result()?;
        }else if let Some(email) = self.email{
            res = session.query_unpaged("SELECT createdat, user_id, jwt, username, email, password, displayname, friends, badges, status, bannercolor, backgroundcolor FROM joltamp.users WHERE email = ? ALLOW FILTERING",
                                        (&email, )).await?.into_rows_result()?;
        }else {
            return Err(Error::msg("Invalid"));
        }
        let (createdat, user_id, jwt, username, email, password, displayname, raw_friends, badges, status, bannercolor, backgroundcolor)
            = res.first_row::<(CqlDate, Uuid, Uuid, String, String, String, String, HashMap<Uuid, i8>, Vec<Uuid>, i8, Option<String>, Option<String>)>()?;

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

        Ok(self)
    }
}
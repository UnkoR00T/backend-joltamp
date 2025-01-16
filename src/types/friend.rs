use std::collections::HashMap;
use std::sync::Arc;
use scylla::Session;
use uuid::Uuid;
use anyhow::Result;
use chrono::NaiveDate;
use serde::Serialize;
use crate::types::user::User;

#[derive(Serialize)]
pub struct Friend{
    pub friendstatus: u8,
    pub user_id: Uuid,
    pub username: Option<String>,
    pub badges: Option<Vec<Uuid>>,
    pub displayname: Option<String>,
    pub bannercolor: Option<String>,
    pub backgroundcolor: Option<String>,
    pub status: Option<i8>,
}

pub trait FriendFunc: std::marker::Sized {
    async fn fill_info(self, session: &Arc<Session>) -> Result<Self>;
}

impl FriendFunc for Friend {
    async fn fill_info(mut self, session: &Arc<Session>) -> Result<Self> {
        let res = session.query_unpaged("SELECT username, badges, displayname, bannercolor, backgroundcolor, status FROM joltamp.users WHERE user_id = ? ALLOW FILTERING",
                                    (&self.user_id, )).await?.into_rows_result()?;
        let (username, badges, displayname, bannercolor, backgroundcolor, status)
            = res.first_row::<(String, Option<Vec<Uuid>>, String, Option<String>, Option<String>, i8)>()?;
        self.username = Some(username);
        self.badges = badges;
        self.displayname = Some(displayname);
        self.bannercolor = bannercolor;
        self.backgroundcolor = backgroundcolor;
        self.status = Some(status);

        Ok(self)

    }
}

impl Friend {
    pub fn from_uuid(uuid: Uuid, status: i8) -> Friend{
        Friend {
            friendstatus: status as u8,
            user_id: uuid,
            username: None,
            badges: None,
            displayname: None,
            bannercolor: None,
            backgroundcolor: None,
            status: None,
        }
    }
    async fn from_user(self, user: User) -> Friend {
        Friend {
            friendstatus: 0,
            user_id: user.user_id.unwrap(),
            username: user.username,
            badges: user.badges,
            displayname: user.displayname,
            bannercolor: user.bannercolor,
            backgroundcolor: user.backgroundcolor,
            status: user.status,
        }
    }
}
use model::user::User;

#[derive(Deserialize,Serialize, Debug)]
pub struct SignupMsgs {
    pub status: i32,
    pub message : String,
}

#[derive(Deserialize,Serialize, Debug)]
pub struct SigninMsgs {
    pub status: i32,
    pub token: String,
    pub signin_user: User,
    pub message : String,
}
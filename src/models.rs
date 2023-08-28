use serde::{Serialize, Deserialize};
#[derive(Debug,Clone, Serialize, Deserialize)]
pub enum User{
    Human,
    Bot
}

impl Into<String> for User{
    fn into(self) -> String {
        match self{
            User::Bot => "Bot".to_string(),
            User::Human => "Human".to_string()
        }
    }
}

impl TryFrom<String> for User{
    type Error = String;
    fn try_from(s:String)->Result<Self,Self::Error>{
        match s.as_str(){
            "Human"=>Ok(User::Human),
            "Bot"=>Ok(User::Bot),
            _=>Err(format!("{} is not a valid user",s))
        }
    }
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Message{
    pub text:String,
    pub user:User
}
#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Conversation{
    pub messages:Vec<Message>,
    pub persona:String
}

impl Conversation{
    pub fn new(persona:String)->Self{
        Conversation{
            messages:Vec::new(),
            persona
        }
    }
}

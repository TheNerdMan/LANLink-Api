struct User{
    username: String,
    discord: String,
    steam: String,
    
}

pub fn create_user(username: String, discord: String, steam: String){
    User{
        username,
        discord,
        steam,
    }
}
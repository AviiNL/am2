pub struct Steam {
    pub username: String,
    pub password: String,
}

pub struct Jwt {
    pub secret: String,
}

pub struct Config {
    pub steam: Steam,
    pub jwt: Jwt,
}

pub static GET_HASH_AND_SALT: &str = "SELECT hash_and_salt, salt FROM usuario WHERE username = $1";
pub static GET_ALL_USUARIOS: &str = "SELECT * FROM usuario";
pub static GET_USUARIO_BY_ID: &str = "SELECT * FROM usuario WHERE usuario_id = $1";
pub static GET_USUARIO_BY_USERNAME: &str = "SELECT * FROM usuario WHERE username = $1";
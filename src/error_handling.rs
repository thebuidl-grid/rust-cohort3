use thiserror::Error;

#[derive(Error,Debug)]
enum ApiError{
    #[error("this `{0}` can not be found at the moment")]
    NotFound(String),

    #[error("this `{0}` api key need to be passed")]
    NoApiKey(String),

    #[error("No internet connection")]
    NoInternet
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("file not found: {0}")]
    FileNotFound(String),

    // #[error("parse error: {0}")]
    // ParseError(#[from] std::num::ParseIntError),

    // #[error("network error: status {status}, message: {message}")]
    // NetworkError { status: u32, message: String },
}

pub fn error(){

    let err = AppError::FileNotFound("config.toml".to_string());
    println!("{}",err); 
    let api_error = ApiError::NoApiKey("12345678".to_string());
    println!(" {:?}",api_error);
    // fn vikings() -> Option<String> { Some("vikings".to_string()) }
    // let value = option().or_else(vikings).unwrap();
    // println!("our value is {:?}",value);


    // let result_type = result();
    // println!("result type value is: {:?}",result_type);
    // panic!("opyrt")
}


fn option()->Option<String>{
    // Some("Hello world".to_string())
    None
}

fn result()->Result<String,String>{
    Ok("No Valid value".to_string())
}

fn result2()->Result<String,String>{
    let result = result()?;
    Ok(result)
}
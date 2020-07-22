use std::result;

pub enum ResultCode {

}

pub type Result<T> = result::Result<T, ResultCode>;
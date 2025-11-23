macro_rules! ok_or_return_err_as_ok {
    ($expr:expr) => {
        match $expr {
            Ok(v) => v,
            Err(e) => return Ok(e),
        }
    };
}

pub(crate) use ok_or_return_err_as_ok;

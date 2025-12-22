#[macro_export]
macro_rules! to_base_response {
    ($expr:expr) => {
        match $expr {
            Ok(body) => $crate::vojo::base_response::BaseResponse { code: 0, body },
            Err(_) => $crate::vojo::base_response::BaseResponse {
                code: -1,
                body: Default::default(),
            },
        }
    };
}

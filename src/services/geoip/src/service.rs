use micro::Future;

pub struct Service {
}

#[derive(Clone, Deserialize)]
pub struct LookupRequest {
    pub ip: String,
}

pub struct LookupResponse {
    pub country_code: Option<String>,
    pub latitude: f32,
    pub longitude: f32,
}

pub struct LookupError {
}

impl Service {
    pub fn new() -> Service {
        Service{}
    }

    pub fn lookup(&self, request: LookupRequest) -> Future<LookupResponse, LookupError> {
        Future::of(LookupResponse {
            country_code: Some("NL".to_owned()),
            latitude: 1.5,
            longitude: 0.75,
        })
    }
}

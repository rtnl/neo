#[derive(Copy, Clone, Debug)]
pub enum ResponseStatus {
    Ok,
    Error,
}

#[derive(Clone, Debug)]
pub struct Response {
    status: ResponseStatus,
}

impl Response {
    pub fn new(status: ResponseStatus) -> Self {
        Self { status }
    }

    pub fn get_status(&self) -> ResponseStatus {
        self.status
    }
}

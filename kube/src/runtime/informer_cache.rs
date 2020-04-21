use crate::{
    api::{Resource},
    client::Client,
}

struct InformerCache {
    client: Client,
}

impl InformerCache {
    pub fn new(client: Client) -> Self {
        InformerCache{
            client,
        }
    }
    fn get<T>(&self) -> Informer {}
}
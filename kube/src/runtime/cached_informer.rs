use crate::{
    api::{Api, ListParams, Meta},
    client::Client,
    runtime::Informer,
};

use futures::lock::Mutex;
use serde::de::DeserializeOwned;
use std::sync::Arc;

struct CachedInformer<K>
where
    K: Clone + DeserializeOwned + Meta,
{
    client: Client,
    informer: Mutex<Option<Arc<Informer<K>>>>,
}

impl<K> CachedInformer<K>
where
    K: Clone + DeserializeOwned + Meta,
{
    ///  Instantiates an informer for a given Kubernetes resource cached across multiple thread/users.
    pub fn new(client: Client) -> Self {
        CachedInformer {
            client,
            informer: Mutex::new(None),
        }
    }

    /// Fetches the cached informer, lazily creating it as necessary.
    async fn get(&self) -> Arc<Informer<K>> {
        let mut lock = self.informer.lock().await;

        if lock.is_some() {
            let informer = lock.take().unwrap();
            *lock = Some(informer.clone());
            return informer;
        }

        let api: Api<K> = Api::all(self.client.clone());
        let opts = ListParams::default();
        let informer = Arc::new(Informer::new(api).params(opts));

        lock.replace(informer.clone());
        informer
    }
}

pub fn get_kv_store(env: &worker::Env) -> worker::Result<worker::KvStore> {
    let kv_store_name: worker::Secret = env.var("KV_STORE_NAME")?;
    env.kv(&kv_store_name.to_string())
}

pub async fn get_cached_value(kv_store: &worker::KvStore, key: &str) -> Option<String> {
    match kv_store.get(key).text().await {
        Ok(optional_html) => optional_html,
        Err(_) => None,
    }
}

pub async fn write_cached_value(kv_store: &worker::KvStore, key: &str, value: &str) {
    match kv_store.put(key, value) {
        Ok(putter) => match putter.execute().await {
            Ok(_) => (),
            Err(e) => {
                worker::console_error!("KV write failed: {:?}", e);
            }
        },
        Err(e) => {
            worker::console_error!("KV write failed: {:?}", e);
        }
    };
}

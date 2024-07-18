use std::path::PathBuf;

use async_cni_rt::plugins::{CniDeserializable, PluginList};

#[tokio::test]
async fn t() {
    let pl = PluginList::from_file(PathBuf::from("/home/kanpov/Documents/test.conflist"))
        .await
        .unwrap();
    dbg!(pl);
}

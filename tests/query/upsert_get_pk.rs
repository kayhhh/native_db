use native_db::*;
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};
use shortcut_assert_fs::TmpFs;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[native_model(id = 1, version = 1)]
#[native_db]
struct Item {
    #[primary_key]
    id: u32,
    name: String,
}

#[test]
fn upsert_get() {
    let item = Item {
        id: 1,
        name: "test".to_string(),
    };

    let tf = TmpFs::new().unwrap();
    let mut models = Models::new();
    models.define::<Item>().unwrap();
    let db = Builder::new()
        .create(&models, tf.path("test").as_std_path())
        .unwrap();

    let rw = db.rw_transaction().unwrap();
    rw.upsert(item.clone()).unwrap();
    rw.commit().unwrap();

    let r = db.r_transaction().unwrap();
    let result_item = r.get().primary(1u32).unwrap().unwrap();
    assert_eq!(item, result_item);
}

#[test]
fn test_upsert_duplicate_key() {
    let tf = TmpFs::new().unwrap();

    let item_1 = Item {
        id: 1,
        name: "test".to_string(),
    };

    let mut models = Models::new();
    models.define::<Item>().unwrap();
    let db = Builder::new()
        .create(&models, tf.path("test").as_std_path())
        .unwrap();

    let rw = db.rw_transaction().unwrap();
    rw.upsert(item_1.clone()).unwrap();
    let result = rw.upsert(item_1.clone());
    assert!(result.is_ok());
}

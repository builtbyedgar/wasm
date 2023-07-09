use web_sys::prelude::*;

mod grocery_list {
  use web_sys::Storage;

  pub fn add_item(item: &str) {
      let window = web_sys::window().unwrap();
      let local_storage = window.local_storage().unwrap().unwrap();

      let list = match local_storage.get_item("list") {
          Some(items) => items,
          None => String::from("[]"),
      };

      let mut items: Vec<String> = serde_json::from_str(&list).unwrap();
      items.push(String::from(item));

      let new_list = serde_json::to_string(&items).unwrap();
      local_storage.set_item("list", &new_list).unwrap();
  }

  pub fn get_items() -> Vec<String> {
      let window = web_sys::window().unwrap();
      let local_storage = window.local_storage().unwrap().unwrap();

      let list = match local_storage.get_item("list") {
          Some(items) => items,
          None => String::from("[]"),
      };

      serde_json::from_str(&list).unwrap()
  }
}
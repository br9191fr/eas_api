// example from https://docs.rs/once_cell/1.8.0/once_cell/

use std::{sync::Mutex, collections::HashMap};
use std::borrow::Borrow;

use once_cell::sync::OnceCell;

fn global_data() -> &'static Mutex<HashMap<i32, String>> {
    static INSTANCE: OnceCell<Mutex<HashMap<i32, String>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert(13, "Spica".to_string());
        m.insert(74, "Hoyten".to_string());
        Mutex::new(m)
    })
}
fn main()  {
    let gd = global_data();
    let mut hm = gd.lock().unwrap();
    println!("{:?}", hm);
    let x = 13;
    println!("At {} gd contains {}",x,hm.borrow().get(&x).unwrap());
    let x = 74;
    println!("At {} gd contains {}",x,hm.borrow().get(&x).unwrap());
    let x = 13;
    println!("At {} gd contains {}",x,hm.borrow().get(&x).unwrap());
    drop(hm);
    hm = gd.lock().unwrap();
    println!("{:?}", hm);
}
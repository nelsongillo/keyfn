extern crate keyfn;

use keyfn::*;

fn main(){
    let mut storage = KeyStorage::new();

    let ctrl_a = KeyBind::new(
        keysym::XK_a,
        vec![Mod::Control],
        Trigger::Pressed,
        pressed,
    );
    
    storage.add(ctrl_a);

    storage.start();
}

fn pressed(){
    println!("Control + A pressed!");
}
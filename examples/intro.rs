extern crate keyfn;

use keyfn::*;

fn main(){
    let mut storage = KeyStorage::new();

    let ctrl_a = KeyBind::new(
        keysym::XK_a,
        vec![Mod::Control],
        Trigger::Pressed,
        ctrl_a_pressed,
    );

    let ctrl_alt_a = KeyBind::new(
        keysym::XK_a,
        vec![Mod::Control, Mod::Alt],
        Trigger::Pressed,
        ctrl_alt_a_pressed,
    );
    
    storage.add(ctrl_a);
    storage.add(ctrl_alt_a);

    storage.start();
}

fn ctrl_a_pressed(){
    println!("Control + A pressed!");
}

fn ctrl_alt_a_pressed(){
    println!("Control + Alt + A pressed!");
}
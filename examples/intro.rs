extern crate keyfn;

use keyfn::*;

fn main(){
     // create new KeyStorage
    let mut storage = KeyStorage::new();

    // Call crtl_a_pressed when Control + a is pressed
    let ctrl_a = KeyBind::new(
        keysym::XK_a,
        vec![Mod::Control],
        Trigger::Pressed,
        ctrl_a_pressed,
    );

    // Call crtl-alt_a_pressed when Control + Alt + a is pressed
    let ctrl_alt_a = KeyBind::new(
        keysym::XK_a,
        vec![Mod::Control, Mod::Alt],
        Trigger::Pressed,
        ctrl_alt_a_pressed,
    );
    
    // Add KeyBinds to storage
    storage.add(ctrl_a);
    storage.add(ctrl_alt_a);

    // start storage
    storage.start();
}

fn ctrl_a_pressed(){
    println!("Control + A pressed!");
}

fn ctrl_alt_a_pressed(){
    println!("Control + Alt + A pressed!");
}
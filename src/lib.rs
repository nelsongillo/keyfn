extern crate x11;

use x11::xlib;
use x11::xlib::{Display, Window, KeySym, XEvent};

use std::ptr;
use std::thread;
use std::os::raw::{c_int, c_uint};
use std::collections::HashMap;

pub use x11::keysym;
pub type FunctionCall = fn() -> ();

const IGNORED_MOD_MASK: c_uint = xlib::LockMask | xlib::Mod2Mask | xlib::Mod3Mask;

#[repr(u32)]
#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Mod{
    Alt =           xlib::Mod1Mask,
    NumLock =       xlib::Mod2Mask,
    ScrollLock =    xlib::Mod3Mask,
    Windows =       xlib::Mod4Mask,
    Mod5 =          xlib::Mod5Mask,
    Control =       xlib::ControlMask,
    Shift =         xlib::ShiftMask,
    CapsLock =      xlib::LockMask,
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub enum Trigger{
    Pressed,
    Released,
}

#[derive(Debug, Hash, PartialOrd, Ord, PartialEq, Eq)]
struct KeyBindMask{
    keycode:    KeySym,
    mod_mask:    c_uint,
}

#[derive(Debug)]
pub struct KeyBind{
    pub keycode:    KeySym,
    pub mods:       Vec<Mod>, 
    pub trigger:    Trigger,
    pub function:   FunctionCall,   
}

#[derive(Debug)]
pub struct KeyStorage{
    pressed:    HashMap<KeyBindMask, FunctionCall>,
    released:   HashMap<KeyBindMask, FunctionCall>,
    display:    *mut Display,
    root:       Window,
}

/*
*   Implementation
*/
impl KeyBind{
    pub fn new(
        keycode: u32,
        mut mods: Vec<Mod>,
        trigger: Trigger,
        function: FunctionCall,
    ) -> Self{

        mods.dedup();
        mods.sort();

        KeyBind{
            keycode: keycode as xlib::KeySym,
            mods: mods,
            trigger: trigger,
            function: function,
        }
    }
}

impl KeyBindMask{
    fn new(
        keycode: KeySym,
        mod_mask: c_uint,
    ) -> Self{
        KeyBindMask{
            keycode: keycode,
            mod_mask: mod_mask,
        }
    }
}

impl KeyStorage{
    pub fn new() -> Self{
        unsafe{
            let display = get_display();
            let root = get_root(display);

            KeyStorage{
                pressed: HashMap::new(),
                released: HashMap::new(),
                display: display,
                root: root,
            }
        }
    }

    pub fn add(
        &mut self,
        keybind: KeyBind
    ){
        unsafe{
            grab_key(self.display, self.root, &keybind);
        }

        if keybind.trigger == Trigger::Pressed {
            for mask in create_bind_mask(&keybind){
                self.pressed.insert(mask, keybind.function);
            }
        }   else{
            for mask in create_bind_mask(&keybind){
                self.released.insert(mask, keybind.function);
            }
        }
    }

    pub fn action(
        &mut self,
        event: &mut XEvent
    ){
        let key = get_keysym_from_keycode(event.as_mut());    
        let xkeyevent = unsafe { event.key };

        if xkeyevent.type_ == xlib::KeyPress {
            
            if let Some(call) = self.pressed.get(&KeyBindMask::new(key, xkeyevent.state)){
                let call = call.to_owned();
                thread::spawn(move || {
                    call();
                });
            }
        } else if xkeyevent.type_ == xlib::KeyRelease {

            if let Some(call) = self.released.get(&KeyBindMask::new(key, xkeyevent.state)){
                let call = call.to_owned();
                thread::spawn(move || {
                    call();
                });
            }
        }
    }

    pub fn start(&mut self) {
        let mut event = xlib::XEvent { pad: [0; 24] };
        loop {
            unsafe {
                xlib::XNextEvent(self.display, &mut event);
            }
            self.action(&mut event);
        }
    }
}

/*
*   Utility
*/
unsafe fn grab_key(
    display: *mut Display,
    root: Window,
    key: &KeyBind
){
    let keycode = xlib::XKeysymToKeycode(display, key.keycode) as c_int;

    for mask in create_mod_mask(&key.mods) {
        xlib::XGrabKey(display,
                       keycode as c_int,
                       mask,
                       root,
                       true as c_int,
                       xlib::GrabModeAsync,
                       xlib::GrabModeAsync);
    }
}

fn get_keysym_from_keycode(press: &mut xlib::XKeyEvent) -> xlib::KeySym {
    unsafe {
        xlib::XLookupKeysym(press as *mut _, 0) 
    }
}

fn create_mod_mask(mods: &Vec<Mod>) -> Vec<c_uint> {
    let mut mod_mask;

    if mods.is_empty(){
        mod_mask = xlib::AnyModifier;
    } else {
        mod_mask = 0;

        for mask in mods {
            mod_mask |= *mask as u32;
        }
    }

    let mut ignored_mask = 0;
    let mut out = Vec::new();

    while ignored_mask <= IGNORED_MOD_MASK {
        if (ignored_mask & !IGNORED_MOD_MASK) > 0 {
            ignored_mask += 1;
            continue;
        }

        out.push(mod_mask | ignored_mask);
        ignored_mask += 1;
    }

    return out;
}

fn create_bind_mask(keybind: &KeyBind) -> Vec<KeyBindMask> {
    let mut out = Vec::new();

    for mask in create_mod_mask(&keybind.mods){
        out.push(KeyBindMask::new(keybind.keycode, mask));
    }

    return out;
}

unsafe fn get_display() -> *mut xlib::Display {
    xlib::XOpenDisplay(ptr::null())
}

unsafe fn get_root(display: *mut xlib::Display) -> xlib::Window {
    xlib::XDefaultRootWindow(display)
}
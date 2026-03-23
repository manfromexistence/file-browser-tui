fb_macro::mod_flat!(handle tty);

#[cfg(windows)]
fb_macro::mod_flat!(windows);

pub static TTY: fb_shared::RoCell<Tty> = fb_shared::RoCell::new();

pub fn init() {
	TTY.with(<_>::default);
}

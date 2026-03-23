use std::sync::atomic::Ordering;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use tracing::warn;
use yazi_actor::Ctx;
use yazi_config::keymap::Key;
use yazi_macro::{act, emit, succ};
use yazi_shared::{data::Data, event::{ActionCow, Event, NEED_RENDER}};
use yazi_widgets::input::InputMode;

use crate::{Executor, Router, app::App};

pub(super) struct Dispatcher<'a> {
	app: &'a mut App,
}

impl<'a> Dispatcher<'a> {
	#[inline]
	pub(super) fn new(app: &'a mut App) -> Self { Self { app } }

	#[inline]
	pub(super) fn dispatch(&mut self, event: Event) -> Result<()> {
		let result = match event {
			Event::Call(action) => self.dispatch_call(action),
			Event::Seq(actions) => self.dispatch_seq(actions),
			Event::Render(partial) => self.dispatch_render(partial),
			Event::Key(key) => self.dispatch_key(key),
			Event::Mouse(mouse) => self.dispatch_mouse(mouse),
			Event::Resize => self.dispatch_resize(),
			Event::Focus => self.dispatch_focus(),
			Event::Paste(str) => self.dispatch_paste(str),
			Event::Timer => self.dispatch_timer(),
		};

		if let Err(err) = result {
			warn!("Event dispatch error: {err:?}");
		}
		Ok(())
	}

	#[inline]
	fn dispatch_call(&mut self, action: ActionCow) -> Result<Data> {
		Executor::new(self.app).execute(action)
	}

	#[inline]
	fn dispatch_seq(&mut self, mut actions: Vec<ActionCow>) -> Result<Data> {
		if let Some(last) = actions.pop() {
			self.dispatch_call(last)?;
		}
		if !actions.is_empty() {
			emit!(Seq(actions));
		}
		succ!();
	}

	#[inline]
	fn dispatch_render(&mut self, partial: bool) -> Result<Data> {
		if partial {
			_ = NEED_RENDER.compare_exchange(0, 2, Ordering::Relaxed, Ordering::Relaxed);
		} else {
			NEED_RENDER.store(1, Ordering::Relaxed);
		}
		succ!()
	}

	#[inline]
	fn dispatch_key(&mut self, key: KeyEvent) -> Result<Data> {
		use crossterm::event::KeyCode;
		use crate::tui::input::InputAction;
		
		// If in animation mode, handle navigation keys but allow typing
		if self.app.bridge.chat_state.animation_mode {
			let animations = crate::tui::AnimationType::all();
			
			// Handle navigation keys for animation carousel
			match key.code {
				KeyCode::Left => {
					// Previous animation
					if self.app.bridge.chat_state.current_animation_index == 0 {
						self.app.bridge.chat_state.current_animation_index = animations.len() - 1;
					} else {
						self.app.bridge.chat_state.current_animation_index -= 1;
					}
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Right => {
					// Next animation (but not Enter - Enter submits input)
					self.app.bridge.chat_state.current_animation_index = 
						(self.app.bridge.chat_state.current_animation_index + 1) % animations.len();
					self.app.bridge.chat_state.animation_start_time = Some(Instant::now());
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {
					// For other keys, fall through to handle input
					// This allows typing while viewing animations
				}
			}
		}
		
		// Global menu navigation keys - work when menu is visible on ANY screen
		if self.app.bridge.chat_state.show_tachyon_menu {
			match key.code {
				KeyCode::Up | KeyCode::Char('k') => {
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Down | KeyCode::Char('j') => {
					self.app.bridge.chat_state.menu.select_next_menu_item();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageUp => {
					self.app.bridge.chat_state.menu.page_up(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::PageDown => {
					self.app.bridge.chat_state.menu.page_down(10);
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Home | KeyCode::Char('g') => {
					self.app.bridge.chat_state.menu.jump_to_top();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::End | KeyCode::Char('G') => {
					self.app.bridge.chat_state.menu.jump_to_bottom();
					// Apply theme preview if in theme submenu
					if let Some(theme_name) = self.app.bridge.chat_state.menu.get_highlighted_theme_name() {
						self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Char('t') | KeyCode::Char('T') => {
					// Toggle light/dark mode when in theme submenu
					if self.app.bridge.chat_state.menu.current_submenu == Some(0) {
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				KeyCode::Enter => {
					// Check if toggle mode button is selected
					if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
						// Toggle the theme mode
						self.app.bridge.chat_state.toggle_theme_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					
					// Check if toggle recording button is selected
					if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
						// Toggle the recording mode
						self.app.bridge.chat_state.menu.toggle_recording_mode();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					
					// Get the current theme name before selecting
					let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();
					
					// Select current menu item (enter submenu or execute action)
					let _should_close = !self.app.bridge.chat_state.menu.select_current_item();
					
					// If we were in theme submenu and selected a theme, just close the menu
					// (theme is already applied from navigation/hover)
					if theme_name.is_some() {
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
					}
					
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				KeyCode::Esc | KeyCode::Backspace => {
					// Go back to main menu if in submenu, otherwise close menu
					if self.app.bridge.chat_state.menu.current_submenu.is_some() {
						self.app.bridge.chat_state.menu.go_back_to_main();
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					} else {
						// Close menu
						self.app.bridge.chat_state.menu_is_closing = true;
						self.app.bridge.chat_state.menu.pick_closing_effect();
						self.app.bridge.chat_state.show_tachyon_menu = false;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
				}
				_ => {}
			}
		}
		
		// Global '0' key handler - toggle menu overlay on ANY screen
		if key.code == KeyCode::Char('0') {
			if self.app.bridge.chat_state.show_tachyon_menu {
				// Closing menu - pick random closing animation
				self.app.bridge.chat_state.menu_is_closing = true;
				self.app.bridge.chat_state.menu.pick_closing_effect();
				self.app.bridge.chat_state.show_tachyon_menu = false;
			} else {
				// Opening menu - pick random opening animation
				self.app.bridge.chat_state.menu_is_closing = false;
				self.app.bridge.chat_state.show_tachyon_menu = true;
				self.app.bridge.chat_state.menu.pick_opening_effect();
			}
			
			NEED_RENDER.store(1, Ordering::Relaxed);
			succ!()
		}
		
		// Handle chat input when in Chat mode or FilePicker mode (chat input is visible)
		if self.app.bridge.mode == crate::tui::AppMode::Chat 
			|| self.app.bridge.mode == crate::tui::AppMode::FilePicker 
		{
			// Handle scrolling when messages exist and input is empty
			if !self.app.bridge.chat_state.messages.is_empty() 
				&& self.app.bridge.chat_state.input.content.is_empty() 
			{
				match key.code {
					KeyCode::Up => {
						self.app.bridge.chat_state.chat_scroll_offset = 
							self.app.bridge.chat_state.chat_scroll_offset.saturating_sub(1);
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					KeyCode::Down => {
						self.app.bridge.chat_state.chat_scroll_offset += 1;
						NEED_RENDER.store(1, Ordering::Relaxed);
						succ!()
					}
					_ => {}
				}
			}
			// Route key to chat input
			let action = self.app.bridge.chat_state.input.handle_key(key);
			
			match action {
				InputAction::Submit(msg) => {
					// Add message to chat - this exits animation mode
					self.app.bridge.chat_state.add_user_message(msg);
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::Exit => {
					// Show farewell train animation
					crate::tui::exit_animation::show_train_farewell();
					// Exit the application
					std::process::exit(0);
				}
				InputAction::Changed => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				InputAction::PreviousHistory | InputAction::NextHistory => {
					// TODO: Implement history navigation
					succ!()
				}
				InputAction::None => {
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
			}
		}
		// Route to yazi's normal key handling
		Router::new(self.app).route(Key::from(key))?;
		succ!();
	}

	#[inline]
	fn dispatch_mouse(&mut self, mouse: MouseEvent) -> Result<Data> {
		use crossterm::event::MouseEventKind;
		
		// Handle menu mouse events globally when menu is visible
		if self.app.bridge.chat_state.show_tachyon_menu {
			match mouse.kind {
				MouseEventKind::Moved => {
					// Handle hover - always process and render if state changed
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, false) {
						// Apply theme preview if hovering over a theme
						if let Some(theme_name) = self.app.bridge.chat_state.menu.get_hovered_theme_name() {
							self.app.bridge.chat_state.apply_theme(&theme_name, self.app.bridge.chat_state.theme_mode);
						}
					}
					NEED_RENDER.store(1, Ordering::Relaxed);
				}
				MouseEventKind::Down(_) => {
					// Handle click - select and potentially enter submenu
					if self.app.bridge.chat_state.menu.handle_mouse(mouse.column, mouse.row, true) {
						// Check if toggle mode button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_mode_selected() {
							// Toggle the theme mode
							self.app.bridge.chat_state.toggle_theme_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}
						
						// Check if toggle recording button is clicked
						if self.app.bridge.chat_state.menu.is_toggle_recording_selected() {
							// Toggle the recording mode
							self.app.bridge.chat_state.menu.toggle_recording_mode();
							NEED_RENDER.store(1, Ordering::Relaxed);
							succ!()
						}
						
						// Get the current theme name before selecting
						let theme_name = self.app.bridge.chat_state.menu.get_selected_theme_name();
						
						// Item was clicked - now select it (enter submenu or execute)
						let _should_close = !self.app.bridge.chat_state.menu.select_current_item();
						
						// If we were in theme submenu and clicked a theme, just close the menu
						// (theme is already applied from hover)
						if theme_name.is_some() {
							self.app.bridge.chat_state.menu_is_closing = true;
							self.app.bridge.chat_state.menu.pick_closing_effect();
							self.app.bridge.chat_state.show_tachyon_menu = false;
						}
						
						NEED_RENDER.store(1, Ordering::Relaxed);
					}
				}
				MouseEventKind::ScrollUp => {
					// Scroll up (previous items)
					self.app.bridge.chat_state.menu.select_prev_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				MouseEventKind::ScrollDown => {
					// Scroll down (next items)
					self.app.bridge.chat_state.menu.select_next_menu_item();
					NEED_RENDER.store(1, Ordering::Relaxed);
					succ!()
				}
				_ => {}
			}
		}
		
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:mouse, cx, mouse)
	}

	#[inline]
	fn dispatch_resize(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:resize, cx, crate::Root::reflow as fn(_) -> _)
	}

	#[inline]
	fn dispatch_focus(&mut self) -> Result<Data> {
		let cx = &mut Ctx::active(&mut self.app.core, &mut self.app.term);
		act!(app:focus, cx)
	}

	#[inline]
	fn dispatch_paste(&mut self, str: String) -> Result<Data> {
		if self.app.core.input.visible {
			let input = &mut self.app.core.input;
			if input.mode() == InputMode::Insert {
				input.type_str(&str)?;
			} else if input.mode() == InputMode::Replace {
				input.replace_str(&str)?;
			}
		}
		succ!();
	}

	#[inline]
	fn dispatch_timer(&mut self) -> Result<Data> {
		// Timer tick for animations - just trigger a render
		// The effects are time-based and will automatically show updated colors
		
		// Update chat state (process LLM responses)
		self.app.bridge.chat_state.update();
		
		// Update splash font cycling (every 3 seconds)
		if self.app.bridge.chat_state.animation_mode 
			&& self.app.bridge.chat_state.last_font_change.elapsed() >= Duration::from_secs(3)
		{
			let animations = crate::tui::AnimationType::all();
			let current_anim = animations[self.app.bridge.chat_state.current_animation_index];
			if current_anim == crate::tui::AnimationType::Splash {
				self.app.bridge.chat_state.splash_font_index = 
					(self.app.bridge.chat_state.splash_font_index + 1) % 113; // 113 valid fonts
				self.app.bridge.chat_state.last_font_change = Instant::now();
			}
		}
		
		// Update Menu timing
		let elapsed = self.app.bridge.chat_state.last_frame_instant.elapsed();
		self.app.bridge.chat_state.menu.update(elapsed);
		self.app.bridge.chat_state.last_frame_instant = Instant::now();
		
		NEED_RENDER.store(1, Ordering::Relaxed);
		succ!();
	}
}

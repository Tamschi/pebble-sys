//! Documentation for this crate is work in progress.
//!
//! For now, please see the C API documentation at <https://developer.rebble.io/developer.pebble.com/docs/c/index.html> for more information.

#![no_std]
#![doc(html_root_url = "https://docs.rs/pebble-sys/0.0.1")]
#![feature(extern_types)]
#![warn(clippy::pedantic)]
#![allow(clippy::match_bool)]
// Matching the SDK documentation.
#![allow(clippy::module_name_repetitions)]

use core::panic::PanicInfo;
use foundation::logging::app_log;
use standard_c::memory::c_str;

pub mod prelude {
	pub use super::standard_c::prelude::*;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	unsafe {
		let panic = &*("### PANIC ###\0" as *const str as *const _ as *const c_str);
		let todo = &*("TODO: Output trace somehow.\0" as *const str as *const _ as *const c_str);
		app_log(1, panic, -1, todo);
	}
	loop {}
}

extern "C" {
	/// Marker type for unsized newtypes.
	type ExternData;
}

pub mod foundation {
	pub mod app {
		extern "C" {
			pub fn app_event_loop();
		}
	}

	pub mod logging {
		use crate::standard_c::memory::{c_str, int};

		//TODO: Enum AppLogLevel

		extern "C" {
			pub fn app_log(
				log_level: u8,
				src_filename: &c_str,
				src_line_number: int,
				fmt: &c_str,
				...
			);
		}
	}

	pub mod resources {
		use crate::standard_c::memory::void;

		#[derive(Copy, Clone)]
		#[repr(transparent)]
		pub struct ResHandle(*const void);

		extern "C" {
			pub fn resource_get_handle(resource_id: u32) -> ResHandle;
			pub fn resource_size(h: ResHandle) -> usize;
			pub fn resource_load(h: ResHandle, buffer: *mut u8, max_length: usize) -> usize;
			pub fn resource_load_byte_range(
				h: ResHandle,
				start_offset: u32,
				buffer: *mut u8,
				num_bytes: usize,
			) -> usize;
		}
	}
}

pub mod graphics {
	pub mod graphics_types {
		#[repr(C)]
		pub struct GPoint {
			pub x: i16,
			pub y: i16,
		}

		#[repr(C)]
		pub struct GRect {
			pub origin: GPoint,
			pub size: GSize,
		}

		#[repr(C)]
		pub struct GSize {
			pub w: i16,
			pub h: i16,
		}

		#[repr(C)]
		pub union GColor8 {
			pub argb: u8,
		}

		pub type GColor = GColor8;

		extern "C" {
			pub type GBitmap;
			pub type GBitmapSequence;
			pub type GContext;
		}

		pub mod color_definitions {
			use super::GColor8;

			macro_rules! colors {
				($name:ident = $value:literal $(, $further_name:ident = $further_value:literal)*$(,)?) => {
					pub const $name: GColor8 = GColor8 {
						argb: $value,
					};
					$(colors!($further_name = $further_value);)*
				};
			}

			colors! {
				BLUE_MOON = 0b_11_00_01_11,
				MELON = 0b_11_11_10_10,
				YELLOW = 0b_11_11_11_00,
			}
		}
	}
}

pub mod user_interface {
	pub mod clicks {
		use crate::standard_c::memory::void;

		#[repr(C)] //TODO
		pub enum ButtonId {
			_A, //TODO
		}

		#[repr(transparent)]
		pub struct ClickRecognizerRef(*mut void);
		pub type ClickHandler = extern "C" fn(recognizer: ClickRecognizerRef, context: *mut void);
		pub type ClickConfigProvider = extern "C" fn(context: *mut void);
	}

	pub mod layers {
		use super::window::Window;
		use crate::{
			graphics::graphics_types::{GContext, GPoint, GRect},
			standard_c::memory::void,
		};
		use core::ptr::NonNull;

		pub type LayerUpdateProc = extern "C" fn(layer: NonNull<Layer>, NonNull<GContext>);

		extern "C" {
			pub type Layer;

			pub fn layer_create(frame: GRect) -> *mut Layer;
			pub fn layer_create_with_data(frame: GRect, data_size: usize) -> *mut Layer;
			pub fn layer_destroy(layer: &'static mut Layer);
			pub fn layer_mark_dirty(layer: NonNull<Layer>);
			pub fn layer_set_update_proc(
				layer: NonNull<Layer>,
				update_proc: Option<LayerUpdateProc>, //TODO: Check if this is legal!
			);
			pub fn layer_set_frame(layer: NonNull<Layer>, frame: GRect);
			pub fn layer_get_frame(layer: NonNull<Layer>) -> GRect;
			pub fn layer_set_bounds(layer: NonNull<Layer>, bounds: GRect);
			pub fn layer_get_bounds(layer: NonNull<Layer>) -> GRect;
			pub fn layer_convert_point_to_screen(layer: NonNull<Layer>, point: GPoint) -> GPoint;
			pub fn layer_convert_rect_to_screen(layer: NonNull<Layer>, rect: GRect) -> GRect;
			pub fn layer_get_window(layer: NonNull<Layer>) -> *mut Window;
			pub fn layer_remove_from_parent(child: NonNull<Layer>);
			pub fn layer_remove_child_layers(parent: NonNull<Layer>);
			pub fn layer_add_child(parent: NonNull<Layer>, child: NonNull<Layer>);
			pub fn layer_insert_below_sibling(
				layer_to_insert: NonNull<Layer>,
				below_sibling_layer: NonNull<Layer>,
			);
			pub fn layer_insert_above_sibling(
				layer_to_insert: NonNull<Layer>,
				above_sibling_layer: NonNull<Layer>,
			);
			pub fn layer_set_hidden(layer: NonNull<Layer>, hidden: bool);
			pub fn layer_get_hidden(layer: NonNull<Layer>) -> bool;
			pub fn layer_set_clips(layer: NonNull<Layer>, clips: bool);
			pub fn layer_get_clips(layer: NonNull<Layer>) -> bool;
			pub fn layer_get_data(layer: NonNull<Layer>) -> NonNull<void>;

			//TODO: #define GRect layer_get_unobstructed_bounds(const Layer* layer);
		}
	}

	pub mod vibes {
		use core::marker::PhantomData;

		#[repr(C)]
		pub struct VibePattern<'a> {
			/// Pointer to an array of segment durations in on(off on)*off? order, up to 10_000ms each.
			/// There must be at least one duration!
			pub durations: *const u32,
			/// Length of the array.
			pub num_segments: u32,
			pub phantom: PhantomData<&'a u32>,
		}

		extern "C" {
			pub fn vibes_cancel();
			pub fn vibes_short_pulse();
			pub fn vibes_long_pulse();
			pub fn vibes_double_pulse();
			pub fn vibes_enqueue_custom_pattern(pattern: VibePattern);
		}
	}

	pub mod window {
		use super::{
			clicks::{ButtonId, ClickConfigProvider, ClickHandler},
			layers::Layer,
		};
		use crate::{graphics::graphics_types::GColor8, standard_c::memory::void};
		use core::ptr::NonNull;

		#[repr(C)]
		pub struct WindowHandlers {
			pub load: Option<WindowHandler>,
			pub appear: Option<WindowHandler>,
			pub disappear: Option<WindowHandler>,
			pub unload: Option<WindowHandler>,
		}

		pub type WindowHandler = extern "C" fn(window: &mut Window);

		extern "C" {
			pub type Window;

			pub fn window_create() -> Option<&'static mut Window>;
			pub fn window_destroy(window: &'static mut Window);
			pub fn window_set_click_config_provider(
				window: &mut Window,
				click_config_provider: Option<ClickConfigProvider>,
			);
			pub fn window_set_click_config_provider_with_context(
				window: &mut Window,
				click_config_provider: Option<ClickConfigProvider>,
				context: *mut void,
			);
			pub fn window_get_click_config_provider(window: &Window)
				-> Option<ClickConfigProvider>;
			pub fn window_get_click_config_context(window: &Window) -> *mut void;
			pub fn window_set_window_handlers(window: &mut Window, handlers: WindowHandlers);

			// The watch is single-threaded and everything's on the heap, so this *should* be fine.
			pub fn window_get_root_layer(window: &Window) -> &mut Layer;

			pub fn window_set_background_color(window: &mut Window, background_color: GColor8);
			pub fn window_is_loaded(window: &mut Window) -> bool;
			pub fn window_set_user_data(window: &mut Window, data: *mut void);
			pub fn window_get_user_data(window: &Window) -> *mut void;
			pub fn window_single_click_subscribe(button_id: ButtonId, handler: ClickHandler);
			pub fn window_single_repeating_click_subscribe(
				button_id: ButtonId,
				repeat_interval_ms: u16,
				handler: ClickHandler,
			);
			pub fn window_multi_click_subscribe(
				button_id: ButtonId,
				min_clicks: u8,
				max_clicks: u8,
				timeout: u16,
				last_click_only: bool,
				handler: ClickHandler,
			);
			pub fn window_long_click_subscribe(
				button_id: ButtonId,
				delay_ms: u16,
				down_handler: ClickHandler,
				up_handler: ClickHandler,
			);
			pub fn window_raw_click_subscribe(
				button_id: ButtonId,
				down_handler: ClickHandler,
				up_handler: ClickHandler,
				context: Option<NonNull<void>>,
			);
			pub fn window_set_click_context(button_id: ButtonId, context: *mut void);
		}

		pub mod number_window {
			//! A ready-made window prompting the user to pick a number.
			//!
			//! TODO: Images

			use super::Window;
			use crate::{
				standard_c::memory::{c_str, void},
				ExternData,
			};
			use core::{
				marker::PhantomData,
				ops::{Deref, DerefMut},
			};

			/// [`NumberWindow`] callbacks.
			#[repr(C)]
			pub struct NumberWindowCallbacks {
				/// Called as the value is incremented.
				pub incremented: Option<NumberWindowCallback>,
				/// Called as the value is decremented.
				pub decremented: Option<NumberWindowCallback>,
				/// Called as the value is confirmed, i.e. as the SELECT button is clicked.
				pub selected: Option<NumberWindowCallback>,
			}

			/// A [`NumberWindow`] callback.
			pub type NumberWindowCallback =
				for<'a> extern "C" fn(number_window: &'a mut NumberWindow<'a>, context: &mut void);

			/// Limited-lifetime foreign type. See [module](./index.html) documentation.  
			/// [`Deref`] and [`DerefMut`] towards [`Window`] (but destroying it as such might leak memory).
			///
			/// [`Deref`]: https://doc.rust-lang.org/stable/core/ops/trait.Deref.html
			/// [`DerefMut`]: https://doc.rust-lang.org/stable/core/ops/trait.DerefMut.html
			/// [`Window`]: ../foreigntype.Window.html
			#[repr(transparent)]
			pub struct NumberWindow<'a>(PhantomData<&'a ()>, ExternData);

			extern "C" {
				pub fn number_window_create<'a>(
					label: &'a c_str,
					callbacks: NumberWindowCallbacks,
					callback_context: &'static mut void,
				) -> Option<&'a mut NumberWindow<'a>>;

				pub fn number_window_destroy(number_window: &'static mut NumberWindow);
				pub fn number_window_set_label<'a>(
					number_window: &mut NumberWindow<'a>,
					label: &'a c_str,
				);
				pub fn number_window_set_max(number_window: &mut NumberWindow, max: i32);
				pub fn number_window_set_min(number_window: &mut NumberWindow, min: i32);
				pub fn number_window_set_value(number_window: &mut NumberWindow, value: i32);
				pub fn number_window_set_step_size(number_window: &mut NumberWindow, step: i32);
				pub fn number_window_get_value(number_window: &NumberWindow) -> i32;
				pub fn number_window_get_window<'a>(
					number_window: &'a NumberWindow<'a>,
				) -> &'a Window;
				#[allow(clashing_extern_declarations)]
				#[link_name = "number_window_get_window"]
				pub fn number_window_get_window_mut<'a>(
					number_window: &'a mut NumberWindow<'a>,
				) -> &'a mut Window;
			}

			impl<'a> Deref for NumberWindow<'a> {
				type Target = Window;

				fn deref(&self) -> &Self::Target {
					unsafe { &*(self as *const _ as *const Self::Target) }
				}
			}

			impl<'a> DerefMut for NumberWindow<'a> {
				fn deref_mut(&mut self) -> &mut Self::Target {
					unsafe { &mut *(self as *mut _ as *mut Self::Target) }
				}
			}
		}
	}

	pub mod window_stack {
		use super::window::Window;
		use core::ptr::NonNull;

		extern "C" {
			pub fn window_stack_push(window: &'static mut Window, animated: bool);
			pub fn window_stack_pop(animated: bool) -> Option<NonNull<Window>>;
			pub fn window_stack_pop_all(animated: bool);
			pub fn window_stack_remove(window: &mut Window, animated: bool) -> bool;
			pub fn window_stack_get_top_window() -> Option<NonNull<Window>>;
			pub fn window_stack_contains_window(window: &mut Window) -> bool;
		}
	}
}

pub mod standard_c {
	pub mod prelude {
		pub use super::memory::prelude::*;
	}

	pub mod memory {
		#![allow(non_camel_case_types)]

		use core::convert::TryFrom;

		pub mod prelude {
			pub use super::{
				CastUncheckedExt, CastUncheckedMutExt, OptionCastUncheckedMutExt, UpcastExt,
				UpcastMutExt,
			};
		}

		pub type int = i32;

		extern "C" {
			pub type c_str;

			/// `void` can be safely passed back across the FFI as `&void` while [`core::ffi::c_void`] cannot.
			/// ([`c_void`] is NOT [unsized]!)
			///
			/// [`core::ffi::c_void`]: https://doc.rust-lang.org/stable/core/ffi/enum.c_void.html
			/// [`c_void`]: https://doc.rust-lang.org/stable/core/ffi/enum.c_void.html
			/// [unsized]: https://doc.rust-lang.org/stable/core/marker/trait.Sized.html
			pub type void;

			pub fn malloc(size: usize) -> Option<&'static mut void>;
			pub fn calloc(count: usize, size: usize) -> Option<&'static mut void>;
			pub fn realloc(ptr: *mut void, size: usize) -> Option<&'static mut void>;
			pub fn free(ptr: &'static mut void);
			pub fn memcmp(ptr1: &void, ptr2: &void, n: usize) -> int;
			pub fn memcpy(dest: &mut void, src: &void, n: usize) -> *mut void;
			pub fn memmove(dest: *mut void, src: *const void, n: usize) -> *mut void;
			pub fn memset(dest: &mut void, c: int, n: usize) -> *mut void;
		}

		impl<'a, T> From<&'a mut T> for &'a mut void {
			fn from(src: &'a mut T) -> Self {
				unsafe { &mut *(src as *mut _ as *mut void) }
			}
		}

		impl<'a, T> From<&'a T> for &'a void {
			fn from(src: &'a T) -> Self {
				unsafe { &*(src as *const _ as *const void) }
			}
		}

		pub trait CastUncheckedExt<'a> {
			/// Casts a mutable untyped heap reference ([`&void]) into a typed one.
			///
			/// # Safety
			///
			/// Horribly unsafe if T doesn't point to an **initialised** instance of T.
			unsafe fn cast_unchecked<T>(self) -> &'a T;
		}

		pub trait CastUncheckedMutExt<'a> {
			/// Casts a mutable untyped heap reference ([`&mut void]) into a typed one.
			///
			/// # Safety
			///
			/// Horribly unsafe if T doesn't point to an **initialised** instance of T.
			unsafe fn cast_unchecked_mut<T>(self) -> &'a mut T;
		}

		pub trait OptionCastUncheckedMutExt<'a> {
			/// Casts a mutable untyped heap reference ([`&mut void]) into a typed one.
			///
			/// # Safety
			///
			/// Horribly unsafe if T doesn't point to an **initialised** instance of T.
			unsafe fn cast_unchecked_mut<T>(self) -> Option<&'a mut T>;
		}

		pub trait UpcastExt<'a> {
			type Output;

			fn upcast(self) -> Self::Output;
		}

		pub trait UpcastMutExt<'a> {
			type Output;

			fn upcast_mut(self) -> Self::Output;
		}

		impl<'a> CastUncheckedExt<'a> for &'a void {
			unsafe fn cast_unchecked<T>(self) -> &'a T {
				&*(self as *const _ as *const T)
			}
		}

		impl<'a> CastUncheckedMutExt<'a> for &'a mut void {
			unsafe fn cast_unchecked_mut<T>(self) -> &'a mut T {
				&mut *(self as *mut _ as *mut T)
			}
		}

		impl<'a> OptionCastUncheckedMutExt<'a> for Option<&'a mut void> {
			unsafe fn cast_unchecked_mut<T>(self) -> Option<&'a mut T> {
				self.map(|void_ref| &mut *(void_ref as *mut _ as *mut T))
			}
		}

		impl<'a, T> UpcastMutExt<'a> for Option<&'a mut T> {
			type Output = Option<&'a mut void>;

			fn upcast_mut(self) -> Self::Output {
				self.map(|t_ref| t_ref.into())
			}
		}

		impl<'a, T> UpcastExt<'a> for &'a T {
			type Output = &'a void;

			fn upcast(self) -> Self::Output {
				self.into()
			}
		}

		impl<'a, T> UpcastMutExt<'a> for &'a mut T {
			type Output = &'a mut void;

			fn upcast_mut(self) -> Self::Output {
				self.into()
			}
		}

		impl c_str {
			/// Interprets a zero-terminated Rust [`prim@str`] as [`c_str`].
			///
			/// # Errors
			///
			/// Iff `text` does not end with `'\0'`.
			pub fn ref_from_str(text: &str) -> Result<&Self, ()> {
				match text.ends_with('\0') {
					true => Ok(unsafe { &*(text as *const _ as *const c_str) }),
					false => Err(()),
				}
			}
		}

		impl<'a> TryFrom<&'a str> for &'a c_str {
			type Error = ();

			fn try_from(value: &'a str) -> Result<Self, Self::Error> {
				match value.ends_with('\0') {
					true => Ok(unsafe { &*(value as *const _ as *const c_str) }),
					false => Err(()),
				}
			}
		}
	}
}

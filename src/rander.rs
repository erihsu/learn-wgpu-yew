use gloo::console::log;
use once_cell::sync::OnceCell;

use web_sys::HtmlCanvasElement;
use yew_canvas::WithRander;

use crate::wgpu_state::State;
#[derive(Clone, PartialEq)]
pub(super) struct Rander();

static mut WGPU_STATE: OnceCell<State> = OnceCell::new();

impl WithRander for Rander {
    fn rand(self, canvas: &HtmlCanvasElement) {
        let canvas = canvas.clone();
        let (height, width) = (canvas.height(), canvas.width());

        unsafe {
            if WGPU_STATE.get().is_none() {
                wasm_bindgen_futures::spawn_local(async move {
                    let state = State::new(&canvas).await;
                    WGPU_STATE.get_or_init(|| state);
                    WGPU_STATE.get().unwrap().render().unwrap();
                })
            }

            if !WGPU_STATE.get().is_none() {
                WGPU_STATE.get_mut().unwrap().resize(width, height);

                WGPU_STATE.get().unwrap().render().unwrap();
            }
        }
    }
}

/// A state that can render using some backend.
pub trait RenderableState {

    type ErrType;

    /// A function where API-specific code will be called, and manages the flow of rendering.
    fn api_loop(&mut self, redraw_request: bool) -> Result<(), Self::ErrType>{
        panic!("No API has been defined using this RenderableState.")
    }
}
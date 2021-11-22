pub enum ProxyEvent {
    /// Call the `update()` function for the given renderer ID
    UPDATE(usize),

    // /// Forcibly render the program/renderer (not reccomended for GUI/Games, more for specific data oriented use-cases)
    // RENDER(usize),

    /// Request the conductor to close.
    CLOSE_REQUEST
}


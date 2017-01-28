//! TODO: Command queues

#[derive(Clone, Copy, Debug)]
pub enum Primitive {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    QuadList,
}

#[derive(Debug)]
pub enum Command {
    /// Batches together many commands on the same thread, maintaining their order.
    Batch(Vec<Command>),
    /// Locks all other command processing threads for the duration of this command
    Exclusive(Box<Command>),

    BindProgram(()),
    BindVao,
    SetViewport,
    GenerateMipmap,
    Clear,
    Draw(Primitive),
    DrawIndexed(Primitive),
}
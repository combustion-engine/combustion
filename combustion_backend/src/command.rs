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

#[derive(Clone, Copy, Debug)]
pub enum Command {
    BindProgram,
    BindVao,
    SetViewport,
    GenerateMipmap,
    Clear,
    Draw(Primitive),
    DrawIndexed(Primitive),
}
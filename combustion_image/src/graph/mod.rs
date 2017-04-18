pub mod line;
pub mod function;
pub mod dots;
pub mod shapes;

pub trait Graphable {
    type PlotData;

    fn plot(x: u32, y: u32, i: f64, Self::PlotData);
}
use brt::grid_shapes::{Circle, Line, Rectangle};

#[derive(Default, Clone)]
pub enum SelectionType {
    #[default]
    Single,
    Circle(Circle),
    Line(Line),
    RectangleBorder(Rectangle),
    RectangleFilled(Rectangle),
}

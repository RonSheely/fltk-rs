// This shows usage of the TableExt::find_cell() method to create a grid-like widget

use fltk::{prelude::*, *};

struct Grid {
    table: table::Table,
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new(0, 0, 0, 0, None)
    }
}

impl Grid {
    pub fn new<S: Into<Option<&'static str>>>(x: i32, y: i32, w: i32, h: i32, label: S) -> Self {
        let mut table = table::Table::new(x, y, w, h, label);
        table.set_frame(enums::FrameType::NoBox);
        table.set_scrollbar_size(-1);
        table.end();
        Self { table }
    }

    pub fn default_fill() -> Self {
        let g = Grid::default();
        Self { table: g.table.size_of_parent().center_of_parent() }
    }

    pub fn set_layout(&mut self, rows: i32, cols: i32) {
        self.table.set_rows(rows);
        self.table.set_cols(cols);
        let parent = self.table.parent().unwrap();
        self.table.set_row_height_all(parent.h() / rows);
        self.table.set_col_width_all(parent.w() / cols);
    }

    pub fn add_widget<W: WidgetExt>(&mut self, widget: &mut W, row: i32, col: i32, row_span: i32, col_span: i32) {
        if let Some((x, y, w, h)) = self.table.find_cell(table::TableContext::Cell, row, col) {
            widget.resize(x, y, w * row_span, h * col_span);
            self.table.add(widget);
        }
    }

    pub fn debug(&mut self, flag: bool) {
        if flag {
            self.table.draw_cell(move |_, ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::Cell => {
                    draw::set_draw_color(enums::Color::Red);
                    draw::draw_rect(x, y, w, h);
                    draw::draw_text2(&format!("{},{}", row, col), x, y, w, h, enums::Align::Center);
                },
                _ => (),
            });
        }
    }
}

fn main() {
    let a = app::App::default();
    let mut win = window::Window::default().with_size(500, 400);
    let mut grid = Grid::default_fill();
    grid.set_layout(10, 5);
    grid.debug(false); // set to true to see cell outlines
    win.end();

    grid.add_widget(&mut frame::Frame::default().with_label("Employee Form"), 0, 1, 3, 1);
    grid.add_widget(&mut frame::Frame::default().with_label("Name"), 2, 1, 1, 1);
    let mut name = input::Input::default();
    grid.add_widget(&mut name, 2, 3, 1, 1);
    grid.add_widget(&mut frame::Frame::default().with_label("Age"), 4, 1, 1, 1);
    let mut age = input::IntInput::default();
    grid.add_widget(&mut age, 4, 3, 1, 1);
    grid.add_widget(&mut frame::Frame::default().with_label("Occupation"), 6, 1, 1, 1);
    let mut occupation = input::Input::default();
    grid.add_widget(&mut occupation, 6, 3, 1, 1);
    let mut btn = button::Button::default().with_label("Submit");
    grid.add_widget(&mut btn, 8, 2, 1, 1);
    
    win.show();

    btn.set_callback(move |_| {
        println!("Name: {}", name.value());
        println!("Age: {}", age.value());
        println!("Occupation: {}", occupation.value());
    });

    a.run().unwrap();
}
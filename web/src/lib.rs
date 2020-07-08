use wasm_bindgen::prelude::*;

const SIZE: usize = 7;

macro_rules! to_position {
    ($value:expr) => {
        match $value {
            0 => "10",
            1 => "30",
            2 => "50",
            3 => "70",
            4 => "90",
            5 => "110",
            6 => "130",
            _ => unreachable!(),
        }
    };
}

#[derive(Copy, Clone, Debug)]
enum Cell {
    None,
    One,
    Two,
}

struct Board {
    cells: [[Cell; SIZE]; SIZE],
}

struct BoardIter<'a> {
    board: &'a Board,
    index: usize,
}

impl<'a> std::iter::IntoIterator for &'a Board {
    type Item = Cell;
    type IntoIter = BoardIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BoardIter {
            board: self,
            index: 0,
        }
    }
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [[Cell::None; SIZE]; SIZE],
        }
    }
}

impl std::iter::Iterator for BoardIter<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > SIZE * SIZE {
            None
        } else {
            Some(self.board.cells[self.index % SIZE][self.index / SIZE])
        }
    }
}

impl<'a> dodrio::Render<'a> for Board {
    fn render(&self, ctx: &mut dodrio::RenderContext<'a>) -> dodrio::Node<'a> {
        use dodrio::{builder::*, bumpalo::collections::Vec};

        let mut cells = Vec::with_capacity_in(SIZE * SIZE, ctx.bump);
        for row in 0..SIZE {
            for col in 0..SIZE {
                let cell = self.cells[row][col];
                cells.push(
                    circle(ctx.bump)
                        .attributes([
                            attr(
                                "class",
                                match cell {
                                    Cell::None => "none",
                                    Cell::One => "one",
                                    Cell::Two => "two",
                                },
                            ),
                            attr("cy", to_position!(row)),
                            attr("cx", to_position!(col)),
                            attr(
                                "r",
                                match cell {
                                    Cell::None => "4",
                                    _ => "8",
                                },
                            ),
                        ])
                        .finish(),
                )
            }
        }

        div(ctx.bump)
            .attributes([attr("class", "container")])
            .child(
                svg(ctx.bump)
                    .attributes([attr("viewBox", "0 0 140 140")])
                    .children(cells)
                    .finish(),
            )
            .finish()
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("should initialize logging OK");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let board = Board::new();

    let vdom = dodrio::Vdom::new(&body, board);

    vdom.forget();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

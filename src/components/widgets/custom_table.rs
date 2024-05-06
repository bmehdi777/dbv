use ratatui::{
    layout::{Position, Size},
    prelude::*,
    widgets::*,
};

#[derive(Debug, Clone, Default)]
pub struct CustomTable<'a> {
    rows: Vec<Vec<String>>,
    header: Vec<String>,
    block: Block<'a>,
    header_style: Style,
    header_block_style: Style,
    highlight_style: Style,
    style: Style,
}

impl<'a> CustomTable<'a> {
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = block;
        self
    }
    pub fn header(mut self, header: Vec<String>) -> Self {
        self.header = header;
        self
    }
    pub fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }
    pub fn header_style(mut self, style: Style) -> Self {
        self.header_style = style;
        self
    }
    pub fn header_block_style(mut self, style: Style) -> Self {
        self.header_block_style = style;
        self
    }

    fn render_header(&self, area: Rect, buf: &mut Buffer) {
        let header_block = Block::default()
            .style(self.header_style)
            .borders(Borders::BOTTOM)
            .border_style(self.header_block_style);

        // make the constraints match the number of item in header : if there is more than 4 (or 3
        // ?) then make it MAX 4 (or 3?)
        // else make it fill by the number of item
        let constraints = [Constraint::Fill(1), Constraint::Fill(1)];
        let rects = Layout::horizontal(constraints)
            .flex(layout::Flex::Center)
            .split(Rect::new(area.x + 1, area.y + 1, area.width, area.height));

        for (index, header_title) in self.header.iter().enumerate() {
            let line = Line::from(Span::from(header_title).style(self.header_style));
            line.render(*rects.get(index).unwrap(), buf);
        }

        header_block.render(Rect::new(area.x, area.y, area.width - 2, 3), buf);
    }
}

impl StatefulWidget for CustomTable<'_> {
    type State = CustomTableState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        buf.set_style(area, self.style);
        self.render_header(area, buf);
        self.block.render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTableState {
    // (x,y)
    pub offset: (usize, usize),
}

use ratatui::{
    prelude::*,
    widgets::*,
};

const MAX_ELEMENT_PER_ROW: usize = 4;

#[derive(Debug, Clone, Default)]
pub struct CustomTable<'a> {
    _rows: Vec<Vec<String>>,
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

    fn render_header(&self, area: Rect, buf: &mut Buffer, state: &mut CustomTableState) {
        let header_block = Block::default()
            .style(self.header_style)
            .borders(Borders::BOTTOM)
            .border_style(self.header_block_style);

        let constraints = if self.header.len() > MAX_ELEMENT_PER_ROW {
            vec![
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
                Constraint::Fill(1),
            ]
        } else {
            self.header
                .iter()
                .map(|_| Constraint::Fill(1))
                .collect::<Vec<_>>()
        };

        let rects = Layout::horizontal(&constraints)
            .flex(layout::Flex::Center)
            .split(Rect::new(area.x + 1, area.y + 1, area.width, area.height));
        for index in 0..constraints.len() {
            let header_title = self.header.get(state.offset_header + index).unwrap();
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
        self.render_header(area, buf, state);
        self.block.render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct CustomTableState {
    // (x,y)
    pub offset_header: usize,
    pub header_length: usize,
    pub content_length: usize,
}

impl CustomTableState {
    pub fn new(header_length: usize, content_length: usize) -> Self {
        CustomTableState {
            offset_header: 0,
            header_length,
            content_length,
        }
    }
    pub fn content_length(mut self, content_length: usize) -> Self {
        self.content_length = content_length;
        self
    }
    pub fn header_length(mut self, header_length: usize) -> Self {
        self.header_length = header_length;
        self
    }

    pub fn next_col(&mut self) {
        self.offset_header = self
            .offset_header
            .saturating_add(1)
            .min(self.header_length.saturating_sub(MAX_ELEMENT_PER_ROW));
    }
    pub fn prev_col(&mut self) {
        self.offset_header = self.offset_header.saturating_sub(1);
    }
}

use ratatui::widgets;

pub struct Context {
    pub list: Vec<BlockListItem>,
    pub list_cursor: Option<usize>,
    pub entry_analyzed :bool,
    pub input: String,
}

pub struct BlockListItem {
    pub name: String,
    pub start_address: fireball::core::Address,
    pub analyzed: bool,
    pub selected: bool,
}

impl BlockListItem {
    pub fn new(name: String, address: fireball::core::Address) -> Self {
        BlockListItem {
            name,
            start_address: address,
            analyzed: false,
            selected: false,
        }
    }
    pub fn list_item(&self) -> widgets::ListItem {
        use widgets::ListItem;

        let mut style = ratatui::style::Style::default();
        if self.analyzed {
            style = style.fg(ratatui::style::Color::Green);
        }
        if self.selected {
            style = style.fg(ratatui::style::Color::Blue);
        }
        ListItem::new(self.name.as_str()).style(style)
    }
}

impl Context {
    pub fn new() -> Self {
        Context {
            list: Vec::new(),
            list_cursor: None,
            entry_analyzed: false,
            input: String::new(),
        }
    }
    pub fn list_get_mut_or_insert(
        &mut self,
        start_address: fireball::core::Address,
    ) -> &mut BlockListItem {
        let pos = self
            .list
            .iter()
            .position(|x| x.start_address == start_address);

        if let Some(pos) = pos {
            &mut self.list[pos]
        } else {
            let list_item =
                BlockListItem::new(start_address.get_virtual_address_str(), start_address);
            self.list.push(list_item);
            self.list.last_mut().unwrap()
        }
    }
}

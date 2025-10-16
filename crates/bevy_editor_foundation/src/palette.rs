use bevy::prelude::*;

/// Generic selection palette for editor tooling.
///
/// Crates can provide type aliases to specialize the palette for their own
/// entity or asset types without redefining the resource.
#[derive(Resource, Debug, Clone)]
pub struct EditorPalette<T> {
    /// Currently selected item in the palette, if any.
    pub selected: Option<T>,
}

impl<T> Default for EditorPalette<T> {
    fn default() -> Self {
        Self { selected: None }
    }
}

impl<T> EditorPalette<T> {
    /// Clears the selection.
    pub fn clear(&mut self) {
        self.selected = None;
    }

    /// Sets the selection to the provided item.
    pub fn select(&mut self, item: T) {
        self.selected = Some(item);
    }
}

#[cfg(test)]
mod tests {
    use super::EditorPalette;

    #[test]
    fn palette_selects_and_clears() {
        let mut palette = EditorPalette::<u32>::default();
        assert!(palette.selected.is_none());

        palette.select(42);
        assert_eq!(palette.selected, Some(42));

        palette.clear();
        assert!(palette.selected.is_none());
    }
}

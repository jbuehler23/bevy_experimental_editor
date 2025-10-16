use bevy::prelude::*;
use std::collections::HashSet;

/// Tracks the set of currently selected entities within the editor world.
#[derive(Resource, Default, Debug, Clone)]
pub struct Selection {
    /// Selected entities, stored as a hash set for quick membership checks.
    pub selected: HashSet<Entity>,
}

impl Selection {
    /// Clears all selections.
    pub fn clear(&mut self) {
        self.selected.clear();
    }

    /// Selects only the provided entity.
    pub fn select(&mut self, entity: Entity) {
        self.selected.clear();
        self.selected.insert(entity);
    }

    /// Toggles selection state for the given entity.
    pub fn toggle(&mut self, entity: Entity) {
        if !self.selected.remove(&entity) {
            self.selected.insert(entity);
        }
    }

    /// Returns true if the entity is currently selected.
    pub fn is_selected(&self, entity: Entity) -> bool {
        self.selected.contains(&entity)
    }

    /// Returns true if no entities are selected.
    pub fn is_empty(&self) -> bool {
        self.selected.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Selection;
    use bevy::prelude::Entity;

    #[test]
    fn selection_tracks_entities() {
        let mut selection = Selection::default();
        let entity_a = Entity::from_raw(1);
        let entity_b = Entity::from_raw(2);

        selection.select(entity_a);
        assert!(selection.is_selected(entity_a));
        assert!(!selection.is_selected(entity_b));

        selection.toggle(entity_b);
        assert!(selection.is_selected(entity_b));

        selection.clear();
        assert!(selection.is_empty());
    }
}

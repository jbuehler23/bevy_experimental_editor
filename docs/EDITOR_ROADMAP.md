# Bevy Experimental Editor - Development Roadmap

This document outlines the development plan for the Bevy Experimental Editor, organized into phases from MVP to feature-complete.

**Current Status**: Phase 1.2 - Core Editor Infrastructure

---

## Phase 1: Core Editor Infrastructure

**Goal**: Build the foundational editor systems that everything else depends on.

### 1.1 Undo/Redo System - COMPLETED
- [x] Command pattern implementation
- [x] History tracking with limits
- [x] Keyboard shortcuts (Ctrl+Z, Ctrl+Shift+Z, Ctrl+Y)
- [x] Command merging for continuous operations
- [x] Integration with all editor operations

**Status**: Fully implemented with `EditorCommand` trait and `EditorHistory` resource.

### 1.2 Enhanced Gizmos - COMPLETED
- [x] Move gizmo with axis constraints (Q key)
- [x] Rotate gizmo (W key)
- [x] Scale gizmo with corner/edge handles (E key)
- [x] Visual mode indicator in viewport
- [x] Click-anywhere-to-drag within selection bounds
- [x] Undo/redo integration for all transformations

**Status**: Fully implemented with three-mode gizmo system.

### 1.3 Viewport System - IN PROGRESS
- [x] Basic 2D camera with pan and zoom
- [ ] Multi-viewport support (Scene/Game/Inspector views)
- [ ] Viewport rendering pipeline
- [ ] Camera controls (orthographic/perspective switching)
- [ ] Grid snapping toggle
- [x] Viewport gizmo overlay (top-left orientation widget)

**Status**: Basic viewport works, needs multi-view support.

### 1.4 Component System - TODO
- [ ] Generic component registry
- [ ] Component reflection for inspector
- [ ] Custom component UI widgets
- [ ] Component search and filtering
- [ ] Drag-and-drop component addition
- [ ] Component presets/templates

---

## Phase 2: Asset Pipeline

**Goal**: Handle all asset types the editor needs to work with.

### 2.1 Asset Browser Enhancements
- [x] Thumbnail generation for images/scenes
- [x] Asset preview panel - kind of
- [ ] Asset metadata editor
- [ ] Search and filtering
- [ ] Folder organization
- [ ] Recent assets panel
- [ ] Favorites/bookmarks

### 2.2 Asset Import Pipeline
- [ ] Drag-and-drop file import
- [ ] Batch import processing
- [ ] Import settings per asset type
- [ ] Asset hot-reloading
- [ ] Asset dependency tracking
- [ ] External file watchers

### 2.3 Material Editor
- [ ] Visual material graph
- [ ] Shader preview
- [ ] Material library
- [ ] PBR parameter editing
- [ ] Texture slot management
- [ ] Material instances

### 2.4 Audio System
- [ ] Audio asset browser
- [ ] Audio preview player
- [ ] Spatial audio visualization
- [ ] Audio source placement
- [ ] Volume/pitch controls

---

## Phase 3: Physics & Collision

**Goal**: Integrate physics simulation and collision editing.

### 3.1 Collision Editor Enhancement
- [ ] Visual collision shape editing
- [ ] Multiple colliders per entity
- [ ] Collision layers and masks
- [ ] Trigger volumes
- [ ] Collision shape library

### 3.2 Physics Integration
- [ ] Integrate with Avian
- [ ] Rigidbody component editor
- [ ] Physics simulation toggle
- [ ] Joint/constraint editor
- [ ] Physics debug visualization

### 3.3 Raycasting Tools
- [ ] Visual raycast tool
- [ ] Raycast debugging
- [ ] Hit information display

---

## Phase 4: Prefabs & Scenes

**Goal**: Robust scene and prefab management system.

### 4.1 Scene Management
- [ ] Multi-scene editing
- [ ] Scene hierarchy viewer
- [ ] Scene loading/unloading
- [ ] Scene merging
- [ ] Scene templates

### 4.2 Prefab System
- [ ] Create prefabs from entities
- [ ] Prefab instances with overrides
- [ ] Nested prefabs
- [ ] Prefab variants
- [ ] Apply/revert instance changes
- [ ] Prefab library browser

### 4.3 Entity Templates
- [ ] Replace enum-based templates with trait system
- [ ] Custom template plugins
- [ ] Template categories
- [ ] Template search
- [ ] Template preview

---

## Phase 5: Scripting Integration

**Goal**: Allow gameplay logic in the editor.

### 5.1 Script System
- [ ] Choose scripting approach (Rhai/Lua/WASM)
- [ ] Script editor with syntax highlighting
- [ ] Script hot-reloading
- [ ] Script debugging
- [ ] Script console

### 5.2 Behavior Components
- [ ] Custom component scripts
- [ ] Event handlers
- [ ] Script templates
- [ ] Script library

### 5.3 Visual Scripting (Optional)
- [ ] Node-based visual scripting
- [ ] Blueprint-style graphs
- [ ] Custom node types

---

## Phase 6: Particle Systems

**Goal**: Visual effects creation and editing.

### 6.1 Particle Editor
- [ ] Particle system component
- [ ] Emission shapes (point, sphere, cone, etc.)
- [ ] Particle properties (color, size, lifetime, velocity)
- [ ] Particle curves/gradients
- [ ] Texture animation
- [ ] Particle preview
- [ ] Particle presets

### 6.2 Effect Sequences
- [ ] Timeline-based effect editor
- [ ] Multiple particle systems per effect
- [ ] Effect triggers

---

## Phase 7: UI Builder

**Goal**: Visual UI/UX design tools.

### 7.1 UI Editor
- [ ] Visual UI layout editor
- [ ] UI hierarchy tree
- [ ] Anchor/pivot controls
- [ ] UI component library (buttons, panels, text, etc.)
- [ ] Layout presets (flex, grid)
- [ ] UI preview mode

### 7.2 UI Styling
- [ ] Style editor
- [ ] Theme system
- [ ] Font management
- [ ] Icon library

---

## Phase 8: Build & Play Mode

**Goal**: Test games directly in the editor.

### 8.1 Play Mode
- [ ] In-editor play mode
- [ ] Play/pause/step controls
- [ ] Runtime object inspection
- [ ] Hot reload during play
- [ ] Performance profiler

### 8.2 Build System
- [ ] Build configuration UI
- [ ] Multi-platform export
- [ ] Build progress tracking
- [ ] Build logs
- [ ] Custom build hooks

---

## Phase 9: Advanced Features

**Goal**: Professional-grade editor features.

### 9.1 Animation System
- [ ] Animation clip editor
- [ ] Keyframe timeline
- [ ] Animation preview
- [ ] Animation blending
- [ ] Animation events

### 9.2 Terrain Editor (Optional)
- [ ] Heightmap editing
- [ ] Terrain painting
- [ ] Vegetation placement
- [ ] Terrain layers

### 9.3 Lighting Tools
- [ ] Light component editor
- [ ] Light probes
- [ ] Baking settings
- [ ] Lightmap preview
- [ ] HDR/tone mapping controls

### 9.4 Camera Tools
- [ ] Camera preview
- [ ] Camera paths
- [ ] Depth of field visualization
- [ ] Frustum visualization

---

## Phase 10: Polish & UX

**Goal**: Make the editor feel professional and complete.

### 10.1 User Experience
- [ ] Keyboard shortcut customization
- [ ] Layout saving/loading
- [ ] Panel docking improvements
- [ ] Theme system (dark/light)
- [ ] Accessibility features

### 10.2 Documentation
- [ ] In-editor help system
- [ ] Tooltips everywhere
- [ ] Tutorial system
- [ ] Example projects
- [ ] API documentation

### 10.3 Performance
- [ ] Large scene optimization
- [ ] Asset streaming
- [ ] Editor profiling
- [ ] Memory optimization

### 10.4 Quality of Life
- [ ] Recent files menu
- [ ] Auto-save
- [ ] Crash recovery
- [ ] Editor preferences
- [ ] Plugin system

---

## Post-1.0: Future Considerations

### Multiplayer Editing
- [ ] Collaborative editing
- [ ] Change synchronization
- [ ] Conflict resolution

### Advanced Tools
- [ ] Custom editor extensions API
- [ ] Editor plugin marketplace
- [ ] Version control integration
- [ ] Cloud asset library

### AI Integration
- [ ] AI-assisted asset generation
- [ ] Smart prefab suggestions
- [ ] Code generation

---

## Release Milestones

### MVP Release
- Core infrastructure (Phase 1)
- Basic asset pipeline (Phase 2)
- Physics integration (Phase 3)
- Functional for simple 2D/3D projects

### Alpha Release
- All MVP features
- Prefabs and scenes (Phase 4)
- Basic scripting (Phase 5)
- Ready for early adopters

### Beta Release
- All Alpha features
- Particle systems (Phase 6)
- UI builder (Phase 7)
- Play mode (Phase 8)
- Feature-complete for most use cases

### 1.0 Release
- All Beta features
- Advanced features (Phase 9)
- Polish and documentation (Phase 10)
- Production-ready

---

## Current Focus

**Current Phase**: Completing Phase 1.3 (Viewport System) and 1.4 (Component System)

**Next Up**: Phase 2.1 (Asset Browser Enhancements)

**Blockers**: None

**Recent Completions**:
- Undo/Redo system with command pattern
- Three-mode gizmo system (Move/Rotate/Scale)
- Gizmo-transform undo integration
- Visual mode indicator
- Repository refactoring to generic editor

---

## Contributing

Each phase builds on the previous, so it's recommended to complete phases in order.

For questions or suggestions, please open an issue on the repository.

#!/usr/bin/env python3
"""
Creates a simple test tileset PNG for the Eryndor editor.
16x16 pixel tiles in a 16x16 grid (256x256 total).
"""

from PIL import Image, ImageDraw

# Tileset parameters
TILE_SIZE = 16
GRID_SIZE = 16
TILESET_WIDTH = TILE_SIZE * GRID_SIZE
TILESET_HEIGHT = TILE_SIZE * GRID_SIZE

# Create image
img = Image.new('RGBA', (TILESET_WIDTH, TILESET_HEIGHT), (0, 0, 0, 0))
draw = ImageDraw.Draw(img)

# Color palette (16 distinct colors)
colors = [
    (0, 0, 0),       # Black
    (255, 255, 255), # White
    (255, 0, 0),     # Red
    (0, 255, 0),     # Green
    (0, 0, 255),     # Blue
    (255, 255, 0),   # Yellow
    (255, 0, 255),   # Magenta
    (0, 255, 255),   # Cyan
    (128, 0, 0),     # Dark Red
    (0, 128, 0),     # Dark Green
    (0, 0, 128),     # Dark Blue
    (128, 128, 0),   # Olive
    (128, 0, 128),   # Purple
    (0, 128, 128),   # Teal
    (192, 192, 192), # Silver
    (128, 128, 128), # Gray
]

# Draw tiles
for row in range(GRID_SIZE):
    for col in range(GRID_SIZE):
        x = col * TILE_SIZE
        y = row * TILE_SIZE

        # Pick color based on position
        color_idx = (row + col) % len(colors)
        color = colors[color_idx]

        # Draw filled rectangle
        draw.rectangle(
            [x, y, x + TILE_SIZE - 1, y + TILE_SIZE - 1],
            fill=color + (255,),
            outline=(64, 64, 64, 255),
            width=1
        )

        # Draw tile number (small, centered)
        tile_id = row * GRID_SIZE + col
        text = str(tile_id)

        # Simple text positioning (center)
        text_x = x + TILE_SIZE // 2 - len(text) * 2
        text_y = y + TILE_SIZE // 2 - 3

        # Draw text shadow
        draw.text((text_x + 1, text_y + 1), text, fill=(0, 0, 0, 128))
        # Draw text
        draw.text((text_x, text_y), text, fill=(255, 255, 255, 255))

# Save
img.save('crates/eryndor-editor/assets/tilesets/test_tileset.png')
print("Created test_tileset.png (256x256, 16x16 tiles)")

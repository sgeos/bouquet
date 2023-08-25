#![allow(dead_code)]

use alloc::{string::String, vec::Vec};

pub type BouquetFloat = f32;
pub type BouquetIndex = usize;
pub type BouquetInteger = i32;

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: BouquetFloat,
    pub y: BouquetFloat,
}

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: BouquetFloat,
    pub y: BouquetFloat,
    pub z: BouquetFloat,
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: BouquetFloat,
    pub g: BouquetFloat,
    pub b: BouquetFloat,
    pub a: BouquetFloat,
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub id: BouquetIndex, // Platform-specific texture ID or handle
    pub size: Vec2,
}

#[derive(Clone, Debug)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: BouquetFloat,
    pub layer: BouquetIndex,
}

#[derive(Clone, Debug)]
pub struct Sprite {
    pub texture: Texture,
    pub transform: Transform,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub content: String,
    pub position: Vec2,
    pub color: Color,
    //pub font: Font, // This can be another struct or an enum
    pub size: BouquetFloat,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub sprites: Vec<Sprite>,
    pub texts: Vec<Text>,
    // More elements as needed
}

#[derive(Clone, Debug)]
pub struct Camera {
    pub position: Vec2,
    pub zoom: BouquetFloat,
}

pub trait Renderer {
    fn clear(&self, color: Color);
    fn draw_sprite(&self, sprite: &Sprite);
    fn draw_text(&self, text: &Text);
    fn present(&self); // Flushes any queued draw calls to the screen
}

/*
enum UIComponent {
    Button { text: Text, background: Sprite, action: Box<dyn Fn()> },
    Label { text: Text },
    // ... other components ...
}
*/

/*** Usage
 *
let renderer: Box<dyn Renderer> = ...; // Initialize platform-specific renderer

renderer.clear(Color::new(0.0, 0.0, 0.0, 1.0));

// Render the game world
for sprite in game_scene.sprites.iter() {
    renderer.draw_sprite(sprite);
}

// Render HUD/UI
for text in hud.texts.iter() {
    renderer.draw_text(text);
}

renderer.present();
 */

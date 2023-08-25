use {
    alloc::string::String,
    bouquet_core::rendering::{Color, Renderer, Sprite, Text},
    fermium::prelude::*,
};

pub struct FermiumRenderer {
    pub window: *mut SDL_Window,
    pub renderer: *mut SDL_Renderer,
    // ... any other resources like texture caches, font systems, etc.
}

impl FermiumRenderer {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, String> {
        // Initialize SDL2
        unsafe { assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0) };

        // Create window
        let window = unsafe {
            SDL_CreateWindow(
                format!("{title}\0").as_ptr() as *const _,
                SDL_WINDOWPOS_CENTERED,
                SDL_WINDOWPOS_CENTERED,
                width,
                height,
                (SDL_WINDOW_SHOWN).0,
            )
        };
        assert!(!window.is_null());

        // Create renderer
        let default_driver = -1;
        let renderer =
            unsafe { SDL_CreateRenderer(window, default_driver, SDL_RENDERER_ACCELERATED.0) };
        assert!(!renderer.is_null());

        Ok(FermiumRenderer {
            window,
            renderer,
            // ... other initializations as necessary
        })
    }
}

impl Renderer for FermiumRenderer {
    fn clear(&self, color: Color) {
        unsafe {
            SDL_SetRenderDrawColor(
                self.renderer,
                (color.r * 255.0) as u8,
                (color.g * 255.0) as u8,
                (color.b * 255.0) as u8,
                (color.a * 255.0) as u8,
            );
            SDL_RenderClear(self.renderer);
        }
    }

    fn draw_sprite(&self, sprite: &Sprite) {
        // You'd need to have loaded the sprite's texture into memory.
        // Here's a simplified example of drawing a texture.
        unsafe {
            let dst_rect = SDL_Rect {
                // Convert sprite's transform into destination rectangle for rendering.
                // This is simplified; you may need to adjust for scaling, rotation, etc.
                x: sprite.transform.position.x as i32,
                y: sprite.transform.position.y as i32,
                w: sprite.texture.size.x as i32,
                h: sprite.texture.size.y as i32,
            };
            SDL_RenderCopy(
                self.renderer,
                sprite.texture.id as *mut _,
                core::ptr::null(),
                &dst_rect,
            );
        }
    }

    fn draw_text(&self, _text: &Text) {
        // Fermium does not provide direct text rendering. You'd need to use something like SDL_ttf.
        // After you have the text rendered to a texture, you can draw it similar to a sprite.
        // For now, just a placeholder:
        // TODO: Implement text rendering via a TTF solution or bitmap fonts.
    }

    fn present(&self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }
}

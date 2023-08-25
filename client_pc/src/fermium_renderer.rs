use {
    alloc::{string::String, vec::Vec},
    bouquet_core::rendering::{Color, Renderer, Sprite, Text},
    fermium::prelude::*,
    libm::ceilf,
    rusttype::{point, Font, Scale},
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

    fn draw_text(&self, text: &Text) {
        // buffer extend is a fudge factor to handle glyphs with negative
        // bounding box values, these values may need to be increased
        let buffer_extend_x = 4;
        let buffer_extend_y = 4.0;
        let font_data = include_bytes!("../../data/fonts/hakidame.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        let scale = Scale::uniform(text.size);
        let v_metrics = font.v_metrics(scale);

        let glyphs: Vec<_> = font
            .layout(&text.content, scale, point(0.0, v_metrics.ascent))
            .collect();

        let glyphs_height = ceilf(v_metrics.ascent - v_metrics.descent + buffer_extend_y) as u32;
        let glyphs_width = {
            let min_x = glyphs
                .first()
                .map(|g| g.pixel_bounding_box().unwrap().min.x)
                .unwrap();
            let max_x = glyphs
                .last()
                .map(|g| g.pixel_bounding_box().unwrap().max.x)
                .unwrap();
            (max_x - min_x + buffer_extend_x) as u32
        };

        let mut buffer: Vec<u8> = vec![0; (glyphs_width * glyphs_height * 4) as usize];

        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                let x_offset = bounding_box.min.x.max(0) as u32;
                let y_offset = bounding_box.min.y.max(0) as u32;
                glyph.draw(|x, y, v| {
                    let base = ((x + x_offset) + (y + y_offset) * glyphs_width) as usize * 4;
                    buffer[base] = (text.color.r * 255.0) as u8;
                    buffer[base + 1] = (text.color.g * 255.0) as u8;
                    buffer[base + 2] = (text.color.b * 255.0) as u8;
                    buffer[base + 3] = (v * 255.0) as u8;
                });
            }
        }

        let texture = unsafe {
            let tex = SDL_CreateTexture(
                self.renderer,
                SDL_PIXELFORMAT_RGBA32.0,
                SDL_TEXTUREACCESS_STATIC.0,
                glyphs_width as i32,
                glyphs_height as i32,
            );
            SDL_UpdateTexture(
                tex,
                core::ptr::null(),
                buffer.as_ptr() as *const _,
                (glyphs_width * 4) as i32,
            );
            tex
        };

        let dst_rect = SDL_Rect {
            x: text.position.x as i32,
            y: text.position.y as i32,
            w: glyphs_width as i32,
            h: glyphs_height as i32,
        };

        unsafe {
            SDL_SetTextureBlendMode(texture, SDL_BLENDMODE_BLEND);
            SDL_RenderCopy(self.renderer, texture, core::ptr::null(), &dst_rect);
            SDL_DestroyTexture(texture); // Important: Free the texture after using
        }
    }

    fn present(&self) {
        unsafe {
            SDL_RenderPresent(self.renderer);
        }
    }
}

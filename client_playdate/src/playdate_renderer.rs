use {
    anyhow::Error,
    crankstart::{
        geometry::ScreenPoint,
        graphics::{Bitmap, Graphics, LCDColor, LCDSolidColor},
    },
    euclid::{point2, size2, Rect},
};

pub struct PlaydateRenderer {}

impl PlaydateRenderer {
    pub fn render_box(
        width: i32,
        height: i32,
        line_width: i32,
        position: ScreenPoint,
    ) -> Result<(), Error> {
        let graphics = Graphics::get();

        let lw = line_width;
        let bi = width / 4; // base increment
        let xi = 2 * bi; // x increment
        let yi = 1 * bi; // y increment
        let xe = xi - 1; // x end
        let ye = yi - 1; // x end
        let loop_max = match height - width / 2 < 0 {
            false => height - width / 2,
            true => 1,
        };
        let mut x = position.x; // x movable position offset
        let mut y = position.y; // y movable position offset

        // fill background with polygons
        // top
        graphics.fill_triangle(
            point2(x + 0, y + yi),
            point2(x + xi, y + 0),
            point2(x + xi, y + yi),
            LCDColor::Solid(LCDSolidColor::kColorBlack),
        )?;
        x += xi;
        graphics.fill_triangle(
            point2(x + 0, y + 0),
            point2(x + xi, y + yi),
            point2(x + 0, y + yi),
            LCDColor::Solid(LCDSolidColor::kColorBlack),
        )?;
        // mid
        x = position.x;
        y += yi;
        graphics.fill_rect(
            Rect {
                origin: point2(x, y),
                size: size2(2 * xi, loop_max),
            },
            LCDColor::Solid(LCDSolidColor::kColorBlack),
        )?;
        // bottom
        y += loop_max;
        graphics.fill_triangle(
            point2(x + 0, y + 0),
            point2(x + xi, y + 0),
            point2(x + xi, y + yi),
            LCDColor::Solid(LCDSolidColor::kColorBlack),
        )?;
        x += xi;
        graphics.fill_triangle(
            point2(x + 0, y + 0 * yi),
            point2(x + xi, y + 0 * yi),
            point2(x + 0, y + yi),
            LCDColor::Solid(LCDSolidColor::kColorBlack),
        )?;

        // top left
        x = position.x;
        y = position.y;
        graphics.draw_line(
            point2(x, y + ye),
            point2(x + xe, y),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        // mid left
        y += yi;
        // left vertical lines
        graphics.draw_line(
            point2(x, y),
            point2(x, y + loop_max),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        graphics.draw_line(
            point2(x + xe, y + yi),
            point2(x + xe, y + yi + loop_max),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        // left fill
        for y in (y..y + loop_max).step_by(2) {
            graphics.draw_line(
                point2(x, y),
                point2(x + xe, y + ye),
                lw,
                LCDColor::Solid(LCDSolidColor::kColorWhite),
            )?;
        }
        // bottom left
        y += loop_max;
        graphics.draw_line(
            point2(x, y),
            point2(x + xe, y + ye),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;

        // top right
        x += xi;
        y = position.y;
        graphics.draw_line(
            point2(x, y),
            point2(x + xe, y + ye),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        // mid right
        y += yi;
        // right vertical lines
        graphics.draw_line(
            point2(x + xe, y),
            point2(x + xe, y + loop_max),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        graphics.draw_line(
            point2(x, y + yi),
            point2(x, y + yi + loop_max),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;
        // right fill
        for y in (y..y + loop_max).step_by(2) {
            graphics.draw_line(
                point2(x, y + ye),
                point2(x + xe, y),
                0,
                LCDColor::Solid(LCDSolidColor::kColorWhite),
            )?;
        }
        y += loop_max;
        // bottom right
        graphics.draw_line(
            point2(x, y + ye),
            point2(x + xe, y),
            lw,
            LCDColor::Solid(LCDSolidColor::kColorWhite),
        )?;

        Ok(())
    }

    pub fn render_box_bitmap(width: i32, height: i32, line_width: i32) -> Result<Bitmap, Error> {
        let graphics = Graphics::get();
        let bitmap = graphics.new_bitmap(
            size2(width, height),
            LCDColor::Solid(LCDSolidColor::kColorClear),
        )?;
        //graphics.push_context(bitmap);
        Self::render_box(width, height, line_width, point2(0, 0))?;
        //graphics.pop_context();
        Ok(bitmap)
    }
}

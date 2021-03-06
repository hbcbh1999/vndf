use nalgebra::{
    cast,

    Norm,
    ToHomogeneous,

    Iso3,
    Mat4,
    Vec2,
    Vec3,
};

use client::graphics::base::Graphics;
use client::graphics::draw::ShapeDrawer;
use client::graphics::transforms::Transforms;
use shared::color;
use shared::util::angle_of;


pub const GRID_UNIT: u32 = 100;

pub struct GridDrawer {
    scaling_factor: f32,
    symbol_drawer: ShapeDrawer,
}

impl GridDrawer {
    pub fn new(graphics: &mut Graphics,
               scaling_factor: f32,)
               -> GridDrawer {
        GridDrawer {
            scaling_factor: scaling_factor,
            symbol_drawer: ShapeDrawer::line(graphics),
        }
    }

    pub fn draw(&mut self,
                zoom: f32,
                win_size: &Vec2<f32>,
                transforms: &Transforms,
                graphics: &mut Graphics,) {

        let mut grid_unit = GRID_UNIT;

        if zoom > 100.0 { grid_unit = GRID_UNIT*100; }
        else if zoom > 50.0 { grid_unit = GRID_UNIT*50; }
        else if zoom > 10.0 { grid_unit = GRID_UNIT*10; }


        // draw horizontal lines
        let x_zoom = win_size[0]*zoom;
        for x in 0..(x_zoom / grid_unit as f32) as u32 {
            let y = x*grid_unit;
            let transform = transforms.symbol_to_screen(
                cast(Vec2::new(-1.0 * x_zoom, y as f32)));

            self.draw_line(
                Vec2::new(x_zoom,0.0),
                transform,
                graphics,
                );

            // draw inverse
            let transform = transforms.symbol_to_screen(
                cast(Vec2::new(-1.0 * x_zoom, -1.0*y as f32)));

            self.draw_line(
                Vec2::new(x_zoom,0.0),
                transform,
                graphics,
            );
        }

        // draw vertical lines
        let y_zoom = win_size[1]*zoom;
        for y in 0..(y_zoom / grid_unit as f32) as u32 {
            let x = y*grid_unit;
            let transform = transforms.symbol_to_screen(
                cast(Vec2::new(x as f32, -1.0 * y_zoom)));

            self.draw_line(
                Vec2::new(0.0,y_zoom),
                transform,
                graphics,
            );

            // draw inverse lines, since range iter cannot handle anything but positives
            let transform = transforms.symbol_to_screen(
                cast(Vec2::new(-1.0*x as f32, -1.0 * y_zoom)));

            self.draw_line(
                Vec2::new(0.0,y_zoom),
                transform,
                graphics,
            );
        }
    }

    fn draw_line(
        &mut self,
        vec: Vec2<f32>,
        transform: Mat4<f32>,
        graphics : &mut Graphics,
    ) {
        let line_rotation = Iso3::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(
                0.0,
                0.0,
                angle_of(vec),
            ),
        );
        self.symbol_drawer.draw(
            vec.norm() * self.scaling_factor * 50.0,
            color::Colors::orange(),
            transform * line_rotation.to_homogeneous(),
            graphics,
        );
    }
}

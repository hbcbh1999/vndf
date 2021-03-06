use nalgebra::Vec2;

use shared::color::Color;


pub type EntityId = u64;


#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Body {
    pub position: Vec2<f64>,
    pub velocity: Vec2<f64>,
    pub force   : Vec2<f64>,
    pub mass    : f64,
}

#[derive(Clone, Debug, Eq, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Broadcast {
    pub sender : EntityId,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Maneuver {
    pub ship_id: EntityId,
    pub data   : ManeuverData,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ManeuverData {
    pub start_s   : f64,
    pub duration_s: f64,
    pub angle     : f64,
    pub thrust    : f64, // 0.0 = 0%, 1.0 = 100%
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Planet {
    pub position: Vec2<f64>,
    pub radius  : f64,
    pub mass    : f64,
    pub color   : Color,
}

#[derive(Clone, Copy, Debug, PartialEq, RustcDecodable, RustcEncodable)]
pub struct Ship;

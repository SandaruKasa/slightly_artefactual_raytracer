use std::sync::Arc;

use super::*;

pub trait Upcast: Sync + Send {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a;
}
impl<T: Object> Upcast for T {
    fn upcast<'a>(self: Arc<Self>) -> Arc<dyn Object + 'a>
    where
        Self: 'a,
    {
        self
    }
}

pub trait Object: Upcast + Sync + Send {
    fn get_color(&self, pos: Point) -> Color;
    fn get_normal(&self, pos: Point) -> Vector;
    fn get_material(&self, pos: Point) -> Material;
    fn is_schematic(&self) -> bool {
        false
    }
}

pub trait MarchingObject: Object {
    fn get_sdf(&self, pos: Point) -> f64;

    //SDF derivative
    fn sdf_drv(&self, pos: Point, delta: Vector) -> f64 {
        self.get_sdf(pos + delta) - self.get_sdf(pos - delta)
    }

    fn _get_normal(&self, pos: Point) -> Vector {
        Vector {
            x: self.sdf_drv(pos, BASIS[0]),
            y: self.sdf_drv(pos, BASIS[1]),
            z: self.sdf_drv(pos, BASIS[2]),
        }
    }
}

pub trait TracingObject: Object {
    fn find_intersection(&self, ray: Ray) -> Option<f64>;
}

pub trait MetaTracingObject: Sync + Send {
    fn get_color(&self, pos: Point) -> Color;
    fn get_material(&self, pos: Point) -> Material;
    fn build_objects(self: Arc<Self>) -> Vec<TracingObjectType>;
}

pub trait LightSource: Sync + Send {
    fn _get_light_dir(&self, pos: Point) -> Vector;
    fn _get_brightness(&self, pos: Point) -> f64;

    fn get_dist(&self, pos: Point) -> f64;
    fn get_color(&self, pos: Point) -> Color;

    fn build_schematic_objects(self: Arc<Self>) -> Vec<TracingObjectType>;

    fn get_light_dir(&self, scene: &Scene, pos: Point) -> Option<Vector> {
        let dir = self._get_light_dir(pos);
        let dist = self.get_dist(pos);
        if scene.compute_shadow_ray(Ray::new(pos, -dir), dist) {
            None
        } else {
            Some(dir)
        }
    }
    fn get_brightness(&self, pos: Point) -> f64 {
        let dist = self.get_dist(pos);
        self._get_brightness(pos) / (dist * dist)
    }
}

pub type ObjectType = Arc<dyn Object>;
pub type MarchingObjectType = Arc<dyn MarchingObject>;
pub type TracingObjectType = Arc<dyn TracingObject>;
pub type MetaTracingObjectType = Arc<dyn MetaTracingObject>;
pub type LightSourceType = Arc<dyn LightSource>;

//! Defines an instance of some piece of geometry in the scene, instances
//! can re-use loaded geometry but apply different transformations and materials
//! to them

use std::sync::Arc;
use std::string::ToString;

use geometry::{Intersection, Boundable, BBox, BoundableGeom, DifferentialGeometry};
use material::Material;
use linalg;

/// An instance of geometry in the scene that only receives light
pub struct Receiver {
    /// The geometry that's being instanced.
    geom: Arc<BoundableGeom + Send + Sync>,
    /// The material being used by this instance.
    pub material: Arc<Material + Send + Sync>,
    /// The transform to world space
    transform: linalg::Transform,
    /// Tag to identify the instance
    pub tag: String,
}

impl Receiver {
    /// Create a new instance of some geometry in the scene
    pub fn new(geom: Arc<BoundableGeom + Send + Sync>, material: Arc<Material + Send + Sync>,
               transform: linalg::Transform, tag: &str) -> Receiver {
        Receiver { geom: geom, material: material, transform: transform, tag: tag.to_string() }
    }
    /// Test the ray for intersection against this insance of geometry.
    /// returns Some(Intersection) if an intersection was found and None if not.
    /// If an intersection is found `ray.max_t` will be set accordingly
    pub fn intersect(&self, ray: &mut linalg::Ray) -> Option<(DifferentialGeometry, &Material)> {
        let mut local = self.transform.inv_mul_ray(ray);
        let mut dg = match self.geom.intersect(&mut local) {
            Some(dg) => dg,
            None => return None,
        };
        ray.max_t = local.max_t;
        dg.p = self.transform * dg.p;
        dg.n = self.transform * dg.n;
        dg.ng = self.transform * dg.ng;
        dg.dp_du = self.transform * dg.dp_du;
        dg.dp_dv = self.transform * dg.dp_dv;
        Some((dg, &*self.material))
    }
}

impl Boundable for Receiver {
    fn bounds(&self) -> BBox {
        self.transform * self.geom.bounds()
    }
}

/// An instance of geometry in the scene that receives and emits light
/// TODO: This is currently just a placeholder, emissive geometry isn't
/// currently implemented. This is why it's identical to `Receiver` :P
pub struct Emitter {
    /// The geometry that's being instanced.
    /// TODO: We could make this an `Option` and then represent point lights
    /// as an Emitter with no geometry!
    geom: Arc<BoundableGeom + Send + Sync>,
    /// The material being used by this instance.
    pub material: Arc<Material + Send + Sync>,
    /// The transform to world space
    transform: linalg::Transform,
    /// Tag to identify the instance
    pub tag: String,
}

// TODO: It may look like we repeat a lot of code here but that won't be the case after I
// actually implement the emitter and unify point lights within this design.
impl Emitter {
    /// Create a new instance of some geometry in the scene
    pub fn new(geom: Arc<BoundableGeom + Send + Sync>, material: Arc<Material + Send + Sync>,
               transform: linalg::Transform, tag: &str) -> Emitter {
        Emitter { geom: geom, material: material, transform: transform, tag: tag.to_string() }
    }
    /// Test the ray for intersection against this insance of geometry.
    /// returns Some(Intersection) if an intersection was found and None if not.
    /// If an intersection is found `ray.max_t` will be set accordingly
    pub fn intersect(&self, ray: &mut linalg::Ray) -> Option<(DifferentialGeometry, &Material)> {
        let mut local = self.transform.inv_mul_ray(ray);
        let mut dg = match self.geom.intersect(&mut local) {
            Some(dg) => dg,
            None => return None,
        };
        ray.max_t = local.max_t;
        dg.p = self.transform * dg.p;
        dg.n = self.transform * dg.n;
        dg.ng = self.transform * dg.ng;
        dg.dp_du = self.transform * dg.dp_du;
        dg.dp_dv = self.transform * dg.dp_dv;
        Some((dg, &*self.material))
    }
}

impl Boundable for Emitter {
    fn bounds(&self) -> BBox {
        self.transform * self.geom.bounds()
    }
}

/// Defines an instance of some geometry with its own transform and material
pub enum Instance {
    Emitter(Emitter),
    Receiver(Receiver),
}

impl Instance {
    /// Create an instance of the geometry in the scene that will only receive light.
    pub fn receiver(geom: Arc<BoundableGeom + Send + Sync>, material: Arc<Material + Send + Sync>,
               transform: linalg::Transform, tag: &str) -> Instance {
        Instance::Receiver(Receiver::new(geom, material, transform, tag))
    }
    /// Create an instance of the geometry in the scene that will emit and receive light
    pub fn emitter(geom: Arc<BoundableGeom + Send + Sync>, material: Arc<Material + Send + Sync>,
               transform: linalg::Transform, tag: &str) -> Instance {
        Instance::Emitter(Emitter::new(geom, material, transform, tag))
    }
    /// Test the ray for intersection against this insance of geometry.
    /// returns Some(Intersection) if an intersection was found and None if not.
    /// If an intersection is found `ray.max_t` will be set accordingly
    pub fn intersect(&self, ray: &mut linalg::Ray) -> Option<Intersection> {
        let hit = match self {
            &Instance::Emitter(ref e) => e.intersect(ray),
            &Instance::Receiver(ref r) => r.intersect(ray),
        };
        match hit {
            Some((dg, mat)) => Some(Intersection::new(dg, self, mat)),
            None => None,
        }
    }
}

impl Boundable for Instance {
    fn bounds(&self) -> BBox {
        match self {
            &Instance::Emitter(ref e) => e.bounds(),
            &Instance::Receiver(ref r) => r.bounds(),
        }
    }
}


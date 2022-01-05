use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::hittable::
{
	Hittable,
	HittableList
};
use crate::materials::
{
	Material,
	dielectric::Dielectric,
	diffuse_light::DiffuseLight,
	isotropic::Isotropic,
	lambertian::Lambertian,
	metal::Metal,
};
use crate::math::vec::
{
	Vec2u,
	Vec3d,
};
use crate::objects::
{
	cone::Cone,
	constant_medium::ConstantMedium,
	cube::Cube,
	cylinder::Cylinder,
	moving_sphere::MovingSphere,
	obj::Obj,
	rect::
	{
		XYRect,
		XZRect,
		YZRect,
	},
	rotate::
	{
		RotateY,
	},
	sphere::Sphere,
	stl::Stl,
	translate::Translate,
	triangle::Triangle,
};
use crate::textures::
{
	Texture,
	checker_texture::CheckerTexture,
	image_texture::ImageTexture,
	noise_texture::NoiseTexture,
	solid_color::SolidColor,
};

use rand::Rng;

use std::sync::Arc;

use std::collections::HashMap;

use std::str::FromStr;

pub struct Scene
{
	camera: Camera,
	textures: HashMap::<String, Arc::<dyn Texture>>,
	materials: HashMap::<String, Arc::<dyn Material>>,
	objects: HittableList,
}

impl Scene
{
	pub fn new(camera: Camera) -> Self
	{
		Self{camera, textures: HashMap::new(), materials: HashMap::new(), objects: HittableList::new()}
	}

	pub fn camera(&self) -> &Camera
	{
		&self.camera
	}
	
	pub fn objects(&self) -> &HittableList
	{
		&self.objects
	}

	pub fn texture(&self, name: String) -> Arc::<dyn Texture>
	{
		match self.textures.get(&name)
		{
			None => panic!("can't find texture {}", name),
			Some(texture) => texture.clone(),
		}
	}

	pub fn insert_texture(&mut self, name: String, texture: Arc::<dyn Texture>)
	{
		self.textures.insert(name, texture);
	}

	pub fn material(&self, name: String) -> Arc::<dyn Material>
	{
		match self.materials.get(&name)
		{
			None => panic!("can't find material {}", name),
			Some(material) => material.clone(),
		}
	}

	pub fn insert_material(&mut self, name: String, material: Arc::<dyn Material>)
	{
		self.materials.insert(name, material);
	}

	pub fn insert_object(&mut self, object: Arc::<dyn Hittable>)
	{
		self.objects.push(object);
	}

	fn parse_xml_attr_f64(node: &roxmltree::Node, name: &str) -> f64
	{
		let attribute = node.attribute(name).expect("no Value attribute found");
		return f64::from_str(attribute).expect("invalid float value");
	}

	fn parse_xml_attr_u32(node: &roxmltree::Node, name: &str) -> u32
	{
		let attribute = node.attribute(name).expect("no Value attribute found");
		return u32::from_str(attribute).expect("invalid float value");
	}

	fn parse_xml_f64(node: &roxmltree::Node) -> f64
	{
		return Self::parse_xml_attr_f64(node, "value");
	}

	fn parse_xml_vec3d(node: &roxmltree::Node) -> Vec3d
	{
		Vec3d::new(Self::parse_xml_attr_f64(node, "x"), Self::parse_xml_attr_f64(node, "y"), Self::parse_xml_attr_f64(node, "z"))
	}

	fn parse_xml_vec2u(node: &roxmltree::Node) -> Vec2u
	{
		Vec2u::new(Self::parse_xml_attr_u32(node, "x"), Self::parse_xml_attr_u32(node, "y"))
	}

	fn parse_xml_camera(node: &roxmltree::Node) -> Camera
	{
		let position = Self::parse_xml_vec3d(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Position").expect("no Position node found for Camera"));
		let direction = Self::parse_xml_vec3d(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Direction").expect("no Direction node found for Camera"));
		let fov = Self::parse_xml_f64(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Fov").expect("no Fov node found for Camera"));
		let size = Self::parse_xml_vec2u(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Size").expect("no Size node found for Camera"));
		let aperture = Self::parse_xml_f64(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Aperture").expect("no Aperture node found for Camera"));
		let focus_dist = Self::parse_xml_f64(&node.children().find(|node| node.is_element() && node.tag_name().name() == "FocusDistance").expect("no FocusDistance node found for Camera"));
		Camera::new(position, direction, Vec3d::new(0.0, 1.0, 0.0), fov, size, aperture, focus_dist)
	}

	fn parse_xml_texture(&mut self, node: &roxmltree::Node)
	{
		if let Some(name) = node.attribute("name")
		{
			return match node.tag_name().name()
			{
				/*"CheckerTexture" => self.insert_texture(name.to_string(), Arc::new(CheckerTexture::from(&self, node))),
				"ImageTexture"   => self.insert_texture(name.to_string(), Arc::new(ImageTexture::from(&self, node))),
				"NoiseTexture"   => self.insert_texture(name.to_string(), Arc::new(NoiseTexture::from(&self, node))),
				"SolidColor"     => self.insert_texture(name.to_string(), Arc::new(SolidColor::from(&self, node))),*/
				_                => panic!("unknown material: {}", node.tag_name().name()),
			};
		}

		panic!("no name given for texture {:?}", node);
	}

	fn parse_xml_textures(&mut self, node: &roxmltree::Node)
	{
		node.children().for_each(|child|
		{
			match child.node_type()
			{
				roxmltree::NodeType::Element => self.parse_xml_texture(&child),
				roxmltree::NodeType::Comment => (),
				roxmltree::NodeType::Text => (),
				_ => panic!("unexpected Textures node {:?}", child),
			}
		});
	}

	fn parse_xml_material(&mut self, node: &roxmltree::Node)
	{
		if let Some(name) = node.attribute("name")
		{
			match node.tag_name().name()
			{
				/*"Dielectric"   => self.insert_material(name.to_string(), Arc::new(Dielectric::from(&self, node))),
				"DiffuseLight" => self.insert_material(name.to_string(), Arc::new(DiffuseLight::from(&self, node))),
				"Isotropic"    => self.insert_material(name.to_string(), Arc::new(Isotropic::from(&self, node))),
				"Lambertian"   => self.insert_material(name.to_string(), Arc::new(Lambertian::from(&self, node))),*/
				_              => panic!("unknown material: {}", node.tag_name().name()),
			}
		}

		panic!("no name given for material {:?}", node);
	}

	fn parse_xml_materials(&mut self, node: &roxmltree::Node)
	{
		node.children().for_each(|child|
		{
			match child.node_type()
			{
				roxmltree::NodeType::Element => self.parse_xml_material(&child),
				roxmltree::NodeType::Comment => (),
				roxmltree::NodeType::Text => (),
				_ => panic!("unexpected Materials node {:?}", child),
			}
		});
	}

	fn parse_xml_object(&mut self, node: &roxmltree::Node)
	{
		self.insert_object(match node.tag_name().name()
		{
			/*"Group"          => Arc::new(BvhNode::from(&self, node)),
			"Cone"           => Arc::new(Cone::from(&self, node)),
			"ConstantMedium" => Arc::new(ConstantMedium::from(&self, node)),
			"Cube"           => Arc::new(Cube::from(&self, node)),
			"Cylinder"       => Arc::new(Cylinder::from(&self, node)),
			"MovingSphere"   => Arc::new(MovingSphere::from(&self, node)),
			"Obj"            => Arc::new(Obj::from(&self, node)),
			"XYRect"         => Arc::new(XYRect::from(&self, node)),
			"XZRect"         => Arc::new(XZRect::from(&self, node)),
			"YZRect"         => Arc::new(YZRect::from(&self, node)),
			"RotateY"        => Arc::new(RotateY::from(&self, node)),
			"Sphere"         => Arc::new(Sphere::from(&self, node)),
			"Stl"            => Arc::new(Stl::from(&self, node)),
			"Translate"      => Arc::new(Translate::from(&self, node)),
			"Triangle"       => Arc::new(Triangle::from(&self, node)),*/
			_                => panic!("unknown object: {}", node.tag_name().name()),
		});
	}

	fn parse_xml_objects(&mut self, node: &roxmltree::Node)
	{
		node.children().for_each(|child|
		{
			match child.node_type()
			{
				roxmltree::NodeType::Element => self.parse_xml_object(&child),
				roxmltree::NodeType::Comment => (),
				roxmltree::NodeType::Text => (),
				_ => panic!("unexpected Objects node {:?}", child),
			}
		});
	}

	pub fn from_file(filename: &str) -> Self
	{
		let content = std::fs::read_to_string(filename).expect("can't read file");
		if let Result::Ok(doc) = roxmltree::Document::parse(&content)
		{
			let mut ret = None;

			doc.root().children().for_each(|node|
			{
				match node.node_type()
				{
					roxmltree::NodeType::Element =>
					{
						match node.tag_name().name()
						{
							"Scene" =>
							{
								let mut scene = Scene::new(Self::parse_xml_camera(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Camera").expect("no Camera node found")));

								scene.parse_xml_textures(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Textures").expect("no Textures node found"));
								scene.parse_xml_materials(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Materials").expect("no Materials node found"));
								scene.parse_xml_objects(&node.children().find(|node| node.is_element() && node.tag_name().name() == "Objects").expect("node Objects node found"));
								ret = Some(scene);
							},
							_ => panic!("unexpected node: {}", node.tag_name().name()),
						}
					},
					roxmltree::NodeType::Comment => (),
					_ => panic!("not supported node {:?}", node),
				}
			});

			match ret
			{
				Some(ret) => return ret,
				None => panic!("no Scene found"),
			}
		}
		panic!("no scene found in file");
	}

	pub fn random_scene() -> Self
	{
		let lookfrom = Vec3d::new(-5.0, 20.0, 5.0);
		let lookat = Vec3d::new(-2.0, 0.0, 3.0);
		let vup = Vec3d::new(0.0, 1.0, 0.0);
		let dist_to_focus = 10.0;
		let aperture = 0.01;
		let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, Vec2u::new(1920, 1080), aperture, dist_to_focus, 0.0, 1.0);

		let mut rng = rand::thread_rng();
		let mut world = HittableList::new();
		let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

		let ground_material: Arc::<dyn Material> = Arc::new(Lambertian::new(
			Arc::new(CheckerTexture::new(
				Arc::new(SolidColor::new(Vec3d::new(0.2, 0.3, 0.1))),
				Arc::new(SolidColor::new(Vec3d::new(0.9, 0.9, 0.9)))
		))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));

		for a in -11..11
		{
			for b in -11..11
			{
				let center = Vec3d::new(a as f64 + 0.9 * rng.gen_range(0.0..1.0), 0.2, b as f64 + 0.9 * rng.gen_range(0.0..1.0));

				let tmp = center - Vec3d::new(4.0, 0.2, 0.0);
				if f64::sqrt(Vec3d::dot(tmp, tmp)) > 0.9
				{
					let choose_mat = rng.gen_range(0.0..1.0);
					if choose_mat < 0.2
					{
						let albedo = Arc::new(SolidColor::new(Vec3d::random(0.0, 1.0) * Vec3d::random(0.0, 1.0) * 1.0));
						let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(albedo));
						let center2 = center + Vec3d::new(0.0, rng.gen_range(0.0..0.5), 0.0);
						objects.push(Arc::new(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, material.clone())));
					}
					else if choose_mat < 0.3
					{
						let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
						objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
					}
					else if choose_mat < 0.6
					{
						let albedo = Arc::new(NoiseTexture::new(Vec3d::random(0.0, 1.0) * Vec3d::random(0.0, 1.0) * 50.0, 4.0));
						let material: Arc::<dyn Material> = Arc::new(DiffuseLight::new(albedo));
						objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
					}
					else if choose_mat < 0.8
					{
						let albedo = Vec3d::random(0.5, 1.0);
						let fuzz = rng.gen_range(0.0..0.5);
						let material: Arc::<dyn Material> = Arc::new(Metal::new(Arc::new(SolidColor::new(albedo)), fuzz));
						objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
					}
					else
					{
						let material: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
						objects.push(Arc::new(Sphere::new(center, 0.2, material.clone())));
					}
				}
			}
		}

		let m1: Arc::<dyn Material> = Arc::new(Dielectric::new(1.5));
		objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, 1.0, 0.0), 1.0, m1.clone())));

		let m2: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.4, 0.2, 0.1)))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(-4.0, 1.0, 0.0), 1.0, m2.clone())));

		let m3: Arc::<dyn Material> = Arc::new(Metal::new(Arc::new(SolidColor::new(Vec3d::new(0.7, 0.6, 0.5))), 0.0));
		objects.push(Arc::new(Sphere::new(Vec3d::new(4.0, 1.0, 0.0), 1.0, m3.clone())));

		let m4: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::new(0.5, 0.5, 0.8) * 10.0))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(10.0, 10.0, 10.0), 5.0, m4.clone())));

		objects.push(Arc::new(Obj::new("cessna.obj", Vec3d::new(-3.0, 1.0, 3.0), Vec3d::newv(1.0 / 10.0))));
		objects.push(Arc::new(Stl::new("frostmourne.stl", Vec3d::new(0.0, -1.0, 7.0), Vec3d::newv(1.0 / 15.0), m2)));

		world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

		Self{camera, textures: HashMap::new(), materials: HashMap::new(), objects: world}
	}

	pub fn simple_light_scene() -> Self
	{
		let lookfrom = Vec3d::new(26.0, 3.0, 6.0);
		let lookat = Vec3d::new(0.0, 2.0, 0.0);
		let vup = Vec3d::new(0.0, 1.0, 0.0);
		let dist_to_focus = 10.0;
		let aperture = 0.01;
		let camera = Camera::with_time(lookfrom, lookat, vup, 20.0, Vec2u::new(1920, 1080), aperture, dist_to_focus, 0.0, 1.0);

		let mut world = HittableList::new();
		let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

		let pertext: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(Vec3d::one(), 4.0))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, -1000.0, 0.0), 1000.0, pertext.clone())));
		objects.push(Arc::new(Sphere::new(Vec3d::new(0.0,     2.0, 0.0),    2.0, pertext.clone())));

		let difflight: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::newv(4.0)))));
		objects.push(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight.clone())));

		world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

		Self{camera, textures: HashMap::new(), materials: HashMap::new(), objects: world}
	}

	pub fn cornell_box() -> Self
	{
		let lookfrom = Vec3d::new(278.0, 278.0, -800.0);
		let lookat = Vec3d::new(278.0, 278.0, 0.0);
		let vup = Vec3d::new(0.0, 1.0, 0.0);
		let dist_to_focus = 10.0;
		let aperture = 0.01;
		let camera = Camera::with_time(lookfrom, lookat, vup, 40.0, Vec2u::new(1000, 1000), aperture, dist_to_focus, 0.0, 1.0);

		let mut scene = Self::new(camera);
		let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

		scene.insert_texture("red".to_string(),   Arc::new(SolidColor::new(Vec3d::new(0.64, 0.05, 0.05))));
		scene.insert_texture("white".to_string(), Arc::new(SolidColor::new(Vec3d::new(0.73, 0.73, 0.73))));
		scene.insert_texture("green".to_string(), Arc::new(SolidColor::new(Vec3d::new(0.12, 0.45, 0.15))));
		scene.insert_texture("light".to_string(), Arc::new(SolidColor::new(Vec3d::new(7.0 , 7.0 , 7.0 ))));

		scene.insert_material("red".to_string()  , Arc::new(  Lambertian::new(scene.texture("red".to_string()))));
		scene.insert_material("white".to_string(), Arc::new(  Lambertian::new(scene.texture("white".to_string()))));
		scene.insert_material("green".to_string(), Arc::new(  Lambertian::new(scene.texture("green".to_string()))));
		scene.insert_material("light".to_string(), Arc::new(DiffuseLight::new(scene.texture("light".to_string()))));

		objects.push(Arc::new(YZRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, scene.material("green".to_string()))));
		objects.push(Arc::new(YZRect::new(  0.0, 555.0,   0.0, 555.0,   0.0, scene.material("red".to_string()))));
		objects.push(Arc::new(XZRect::new(113.0, 443.0, 127.0, 432.0, 554.0, scene.material("light".to_string()))));
		objects.push(Arc::new(XZRect::new(  0.0, 555.0,   0.0, 555.0,   0.0, scene.material("white".to_string()))));
		objects.push(Arc::new(XZRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, scene.material("white".to_string()))));
		objects.push(Arc::new(XYRect::new(  0.0, 555.0,   0.0, 555.0, 555.0, scene.material("white".to_string()))));

		let mut box1: Arc::<dyn Hittable> = Arc::new(Cube::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(165.0, 330.0, 165.0), scene.material("white".to_string())));
		box1 = Arc::new(RotateY::new(box1.clone(), 15.0));
		box1 = Arc::new(Translate::new(box1.clone(), Vec3d::new(265.0, 0.0, 295.0)));

		objects.push(Arc::new(ConstantMedium::new(box1.clone(), 0.01, Arc::new(SolidColor::new(Vec3d::zero())))));

		let mut box2: Arc::<dyn Hittable> = Arc::new(Cube::new(Vec3d::new(0.0, 0.0, 0.0), Vec3d::new(165.0, 165.0, 165.0), scene.material("white".to_string())));
		box2 = Arc::new(RotateY::new(box2.clone(), -18.0));
		box2 = Arc::new(Translate::new(box2.clone(), Vec3d::new(130.0, 0.0, 65.0)));
		objects.push(Arc::new(ConstantMedium::new(box2.clone(), 0.01, Arc::new(SolidColor::new(Vec3d::one())))));

		scene.insert_object(Arc::new(BvhNode::new(objects, 0.0, 1.0)));
		scene
	}

	pub fn chapter2() -> Self
	{
		let lookfrom = Vec3d::new(478.0, 278.0, -600.0);
		let lookat = Vec3d::new(278.0, 278.0, 0.0);
		let vup = Vec3d::new(0.0, 1.0, 0.0);
		let dist_to_focus = 10.0;
		let aperture = 0.01;
		let camera = Camera::with_time(lookfrom, lookat, vup, 40.0, Vec2u::new(1000, 1000), aperture, dist_to_focus, 0.0, 1.0);

		let mut rng = rand::thread_rng();
		let mut world = HittableList::new();
		let mut objects: Vec::<Arc::<dyn Hittable>> = Vec::new();

		let ground = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.48, 0.83, 0.53)))));
		let mut boxes1: Vec::<Arc::<dyn Hittable>> = Vec::new();
		let boxes_per_side = 20;
		for i in 0..boxes_per_side
		{
			for j in 0..boxes_per_side
			{
				let w = 100.0;
				let x0 = -1000.0 + i as f64 * w;
				let z0 = -1000.0 + j as f64 * w;
				let y0 = 0.0;
				let x1 = x0 + w;
				let y1 = rng.gen_range(1.0..101.0);
				let z1 = z0 + w;

				boxes1.push(Arc::new(Cube::new(Vec3d::new(x0, y0, z0), Vec3d::new(x1, y1, z1), ground.clone())));
			}
		}
		objects.push(Arc::new(BvhNode::new(boxes1, 0.0, 1.0)));

		let light: Arc::<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(SolidColor::new(Vec3d::newv(7.0)))));
		objects.push(Arc::new(XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, light.clone())));

		let center1 = Vec3d::new(400.0, 400.0, 200.0);
		let center2 = center1 + Vec3d::new(30.0, 0.0, 0.0);
		let moving_sphere_material: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.7, 0.3, 0.1)))));
		objects.push(Arc::new(MovingSphere::new(center1, center2, 0.0, 1.0, 50.0, moving_sphere_material.clone())));
		objects.push(Arc::new(Sphere::new(Vec3d::new(260.0, 140.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5)))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Arc::new(SolidColor::new(Vec3d::new(0.8, 0.8, 0.9))), 1.0)))));

		/*let b1: Arc::<dyn Hittable> = Arc::new(Sphere::new(Vec3d::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
		objects.push(b1.clone());
		objects.push(Arc::new(ConstantMedium::new(b1.clone(), 0.2, Arc::new(SolidColor::new(Vec3d::new(0.2, 0.4, 0.9))))));
		let b2: Arc::<dyn Hittable> = Arc::new(Sphere::new(Vec3d::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
		objects.push(Arc::new(ConstantMedium::new(b2.clone(), 0.0001, Arc::new(SolidColor::new(Vec3d::new(1.0, 1.0, 1.0))))));*/

		objects.push(Arc::new(Sphere::new(Vec3d::new(400.0, 200.0, 400.0), 100.0, Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg")))))));
		objects.push(Arc::new(Sphere::new(Vec3d::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new(Arc::new(NoiseTexture::new(Vec3d::new(1.0, 1.0, 1.0), 0.1)))))));

		let mut boxes2: Vec::<Arc::<dyn Hittable>> = Vec::new();
		let white: Arc::<dyn Material> = Arc::new(Lambertian::new(Arc::new(SolidColor::new(Vec3d::new(0.73, 0.73, 0.73)))));
		for _ in 0..1000
		{
			boxes2.push(Arc::new(Sphere::new(Vec3d::random(0.0, 165.0), 10.0, white.clone())));
		}
		objects.push(Arc::new
		(
			Translate::new
			(
				Arc::new(RotateY::new
				(
					Arc::new(BvhNode::new(boxes2, 0.0, 1.0)),
					15.0
				)),
				Vec3d::new(-100.0, 270.0, 395.0)
			)
		));

		world.push(Arc::new(BvhNode::new(objects, 0.0, 1.0)));

		Self{camera, textures: HashMap::new(), materials: HashMap::new(), objects: world}
	}
}

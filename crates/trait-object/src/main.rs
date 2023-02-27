#![warn(clippy::all, clippy::nursery)]

use std::{any::Any, marker::PhantomData};

use uuid::Uuid;

pub struct ObjectID<T> {
    canvas_id: Uuid,
    id: usize,
    _phantom: PhantomData<T>,
}

impl<T> ObjectID<T> {
    fn new(canvas_id: Uuid, id: usize) -> Self {
        Self {
            canvas_id,
            id,
            _phantom: Default::default(),
        }
    }
}

pub struct Canvas {
    id: Uuid,
    objects: Vec<Option<Box<dyn Draw>>>,
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn draw_all(&self) {
        for object in self.objects.iter().flatten() {
            object.draw();
        }
    }

    pub fn add<T>(&mut self, object: T) -> ObjectID<T>
    where
        T: Draw + 'static,
    {
        self.objects.push(Some(Box::new(object) as Box<dyn Draw>));
        ObjectID::new(self.id, self.objects.len() - 1)
    }

    pub fn get_ref<T>(&self, object: &ObjectID<T>) -> &T
    where
        T: Draw + 'static,
    {
        assert_eq!(object.canvas_id, self.id,);
        self.objects
            .get(object.id)
            .unwrap()
            .as_ref()
            .unwrap()
            .as_ref()
            .as_any()
            .downcast_ref()
            .unwrap()
    }

    pub fn get_mut<T>(&mut self, object: &ObjectID<T>) -> &mut T
    where
        T: Draw + 'static,
    {
        assert_eq!(object.canvas_id, self.id,);
        self.objects
            .get_mut(object.id)
            .unwrap()
            .as_mut()
            .unwrap()
            .as_mut()
            .as_any_mut()
            .downcast_mut()
            .unwrap()
    }

    pub fn remove<T>(&mut self, object: ObjectID<T>) -> Box<T>
    where
        T: Draw + 'static,
    {
        assert_eq!(object.canvas_id, self.id,);
        let obj = self.objects.get_mut(object.id).unwrap().take().unwrap();
        assert!(obj.as_ref().as_any().is::<T>());
        unsafe {
            let raw = Box::into_raw(obj);
            Box::from_raw(raw as *mut T)
        }
    }
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            objects: Default::default(),
        }
    }
}

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
where
    T: 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Draw: AsAny {
    fn draw(&self);
}

pub struct Triangle {
    pub size: f32,
}

impl Triangle {
    const fn new(size: f32) -> Self {
        Self { size }
    }
}

impl Draw for Triangle {
    fn draw(&self) {
        println!("Triangle (size: {})", self.size);
    }
}

pub struct Sqaure {
    pub w: f32,
    pub h: f32,
}

impl Sqaure {
    const fn new(w: f32, h: f32) -> Self {
        Self { w, h }
    }
}

impl Draw for Sqaure {
    fn draw(&self) {
        println!("Square ({} x {})", self.w, self.h);
    }
}

fn main() {
    let mut canvas = Canvas::new();

    let triangle = canvas.add(Triangle::new(1.5));
    let square = canvas.add(Sqaure::new(2.5, 3.5));

    canvas.draw_all();

    println!("--------------------------------------------");

    canvas.get_mut(&triangle).size = 2.0;
    canvas.get_mut(&square).w = 10.2;

    canvas.draw_all();

    println!("--------------------------------------------");

    let triangle = canvas.remove(triangle);
    assert_eq!(triangle.size, 2.0);

    canvas.draw_all();
}

use std::any::{Any, AnyRefExt, AnyMutRefExt};

#[deriving(Show)]
pub struct Entity {
    components: Vec<Box<Any>>
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            components: Vec::new()
        }
    }

    pub fn add<T: 'static>(&mut self, component: Box<T>) {
        self.components.push(component as Box<Any>);
    }

    pub fn get<'a, T: 'static>(&'a self) -> Option<&'a T> {
        for component in self.components.iter() {
            match component.as_ref::<T>() {
                Some(c) => return Some(c),
                None => {}
            }
        }
        None
    }

    pub fn get_mut<'a, T: 'static>(&'a mut self) -> Option<&'a mut T> {
        for component in self.components.mut_iter() {
            match component.as_mut::<T>() {
                Some(c) => return Some(c),
                None => {}
            }
        }
        None
    }
}

#[test]
fn test_get() {
    struct Component {
        pub field: int
    }

    let mut e = Entity::new();
    e.add(box Component { field: 0; });
    let c: &Component = e.get();
    assert!(!c.is_none());
}

#[test]
fn test_get_mut() {
    struct Component {
        pub field: int
    }

    let mut e = Entity::new();
    e.add(box Component { field: 0; });
    let mut c: &Component = e.get_mut();
    assert!(!c.is_none());
}
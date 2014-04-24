
use collections::HashMap;
use std::any::Any;
use std::intrinsics::TypeId;

pub trait Component {
	// the component is a bit flag that uniquely
	// identifies the component among other components
	// TODO: make this global (in the World struct?)
	// using a u64 for type means the maximum number of
	// unique components is 64 (maybe 65 if 0 is also used)
	fn get_component_type(&self) -> u64;
	// fn start(&mut self);
	// fn update(&mut self);
}

#[deriving(Eq, Hash, Show)]
pub struct ComponentType(u64);

// TODO: could just override add to do bit shifting

impl BitAnd<ComponentType, ComponentType> for ComponentType {
    fn bitand(&self, rhs: &ComponentType) -> ComponentType {
        let ComponentType(left) = *self;
        let ComponentType(right) = *rhs;
        ComponentType(left & right)
    }
}

impl BitOr<ComponentType, ComponentType> for ComponentType {
    fn bitor(&self, rhs: &ComponentType) -> ComponentType {
        let ComponentType(left) = *self;
        let ComponentType(right) = *rhs;
        ComponentType(left | right)
    }
}

impl BitXor<ComponentType, ComponentType> for ComponentType {
    fn bitxor(&self, rhs: &ComponentType) -> ComponentType {
        let ComponentType(left) = *self;
        let ComponentType(right) = *rhs;
        ComponentType(left ^ right)
    }
}

impl Shl<int, ComponentType> for ComponentType {
    fn shl(&self, rhs: &int) -> ComponentType {
        let ComponentType(value) = *self;
        ComponentType(value << *rhs)
    }
}

impl Shr<int, ComponentType> for ComponentType {
    fn shr(&self, rhs: &int) -> ComponentType {
        let ComponentType(value) = *self;
        ComponentType(value >> *rhs)
    }
}

// TODO: change u64 references to ComponentType ones
pub struct ComponentTypeManager {
    current_id: ComponentType,
    types: HashMap<TypeId, ComponentType>
}

impl ComponentTypeManager {
    pub fn new() -> ComponentTypeManager {
        ComponentTypeManager {
            current_id: ComponentType(1u64),
            types: HashMap::new()
        }
    }

    pub fn get_type_for<T: Component + 'static>(&mut self) -> ComponentType {
        let id = self.types.find_or_insert(TypeId::of::<T>(), self.current_id);
        if self.current_id == *id {
            self.current_id = self.current_id << 1;
        }
        *id
    }

    pub fn get_type_for_instance<T: Component + 'static>(&mut self, component: ~T) -> ComponentType {
        let id = self.types.find_or_insert(component.get_type_id(), self.current_id);
        if self.current_id == *id {
            self.current_id = self.current_id << 1;
        }
        *id
    }
}

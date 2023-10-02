use std::any::Any;

pub trait ComponentArray {
    fn resize_with_none(&mut self, size: usize);
    fn unset(&mut self, index: usize);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentArray for Vec<Option<T>> {
    fn resize_with_none(&mut self, size: usize) {
        self.resize_with(size, || None);
    }

    fn unset(&mut self, index: usize) {
        self[index] = None;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
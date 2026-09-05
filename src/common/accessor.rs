
pub trait Getter<T> {
    fn get(&self) -> &T;
}

pub trait Setter<T> {
    fn set(&mut self, new_value: T);
}

pub struct Accessor<T> {
    value: T,
}

impl<T> Accessor<T> {
    pub fn new(value: T) -> Self {
        Accessor {value: value}
    }
}

impl<T> Getter<T> for Accessor<T> {
    fn get(&self) -> &T {
        return &self.value;
    }
}

impl<T> Setter<T> for Accessor<T> {
    fn set(&mut self, new_value: T) {
        self.value = new_value;
    }
}


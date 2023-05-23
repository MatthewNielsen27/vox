use std::marker::{PhantomData, Tuple};

pub struct TaggedAs<Tag, T> {
    pub value: T,
    _phantom: PhantomData<Tag>,
}

impl<T, Tag> From<T> for TaggedAs<Tag, T> {
    fn from(value: T) -> Self {
        Self { value, _phantom: PhantomData }
    }
}

impl<T, Tag> TaggedAs<Tag, T> {
    pub fn new<J: Into<T>>(j: J) -> Self {
        Self { value: j.into(), _phantom: PhantomData }
    }

    pub fn apply(&mut self, f: &dyn Fn(&mut T)) {
        f(&mut self.value)
    }
}

pub trait Taggable<T> {
    fn into_tagged<Tag>(self) -> TaggedAs<Tag, T>;
}

impl<T> Taggable<T> for T {
    fn into_tagged<Tag>(self) -> TaggedAs<Tag, T> {
        self.into()
    }
}

pub fn tag<Tag, T>(t: T) -> TaggedAs<Tag, T> {
    t.into()
}

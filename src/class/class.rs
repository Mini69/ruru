use std::convert::From;

use binding::class::{define_class, new_instance, define_method, define_singleton_method};
use binding::global::rb_cObject;
use types::{Callback, Value};
use util::create_arguments;

use super::any_object::AnyObject;
use super::traits::RawObject;

pub struct Class {
    value: Value
}

impl Class {
    // TODO: replace rb_cObject with optional superclass
    pub fn new(name: &str) -> Self {
        Class {
            value: define_class(name, rb_cObject)
        }
    }

    pub fn new_instance(&self, arguments: Vec<AnyObject>) -> AnyObject {
        let (argc, argv) = create_arguments(arguments);
        let instance = new_instance(self.value(), argc, argv);

        AnyObject::from(instance)
    }

    pub fn define_method<T: RawObject>(&self, name: &str, callback: Callback<T>) {
        define_method::<T>(self.value, name, callback);
    }

    pub fn define_singleton_method<T: RawObject>(&self, name: &str, callback: Callback<T>) {
        define_singleton_method(self.value, name, callback);
    }
}

impl From<Value> for Class {
    fn from(value: Value) -> Self {
        Class {
            value: value
        }
    }
}

impl RawObject for Class {
    fn value(&self) -> Value {
        self.value
    }
}

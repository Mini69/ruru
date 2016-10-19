use ruby_sys::gc;

use types::Value;

pub fn mark(value: Value) {
    unsafe { gc::rb_gc_mark(value) };
}

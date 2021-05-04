use std::any::Any;
use std::mem;
use crate::vm::obj::Gc;

impl super::VM {
    /// Allocate a garbage-collected value on the heap.
    ///
    /// This method is how to obtain a `Gc` pointer (not exported from this crate and has no public
    /// constructor). Values allocated with this method will be owned (and eventually freed) by the
    /// VM. If the value lives until the VM goes out of scope, it will be freed in the VM's `Drop`
    /// implementation.
    ///
    /// For a usage example, see [`NativeFun`](./type.NativeFun.html).
    pub fn alloc<T: Any>(&mut self, obj: T) -> Gc<T> {
        // if self.should_collect() {
        //     self.collect_garbage();
        // }
        //
        // let size = mem::size_of::<T>();
        // self.total_allocations += size;
        //
        let ptr = Gc::new(obj);
        // self.objects.push(ptr.as_any());
        //
        // #[cfg(feature = "trace-gc")]
        // log::debug!(
        //     "{:p} allocate {} bytes for {}",
        //     ptr,
        //     size,
        //     std::any::type_name::<T>()
        // ); FIXME TODO

        ptr
    }
}
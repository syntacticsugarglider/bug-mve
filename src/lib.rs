use std::marker::PhantomData;

fn drop_send<F: Send>(_: F) {}

trait Object: Send {}

pub trait Bound {
    type Type;
}

impl Bound for dyn Object {
    type Type = ();
}

trait Contains<T> {}

struct IsBound<T: Bound + ?Sized>(PhantomData<dyn Contains<T::Type> + Send>);

impl<T: Bound + ?Sized> IsBound<T> {
    pub fn new() -> Self {
        IsBound(PhantomData)
    }
}

fn fails() {
    drop_send(async {
        let collection: IsBound<dyn Object> = IsBound::new();
        async {}.await;
    })
}

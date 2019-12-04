use std::marker::PhantomData;

fn run<F: Send>(_: F) {}

trait Object: Send {}

pub trait Trait {
    type Type;
}

impl Trait for dyn Object {
    type Type = ();
}

trait Contains<T> {}

struct IsTrait<T: Trait + ?Sized>(PhantomData<dyn Contains<T::Type> + Send>);

impl<T: Trait + ?Sized> IsTrait<T> {
    pub fn new() -> Self {
        IsTrait(PhantomData)
    }
}

fn fails() {
    run(async {
        let collection: IsTrait<dyn Object> = IsTrait::new();
        async {}.await;
    })
}

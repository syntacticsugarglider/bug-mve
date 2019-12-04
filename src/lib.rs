use std::{future::Future, pin::Pin};
use futures::Stream;

fn run<F: Sync + Send + Future<Output = ()> + 'static>(_: F) {
    unimplemented!()
}

trait Object: Sync + Send {}

struct Struct;

impl Object for Struct {}

pub trait Trait {
    type Type;
}

impl Trait for Box<dyn Object> {
    type Type = ();
}

struct IsTrait<T: Trait>(Box<dyn Stream<Item = T::Type> + Sync + Send>);

impl<T: Trait> IsTrait<T> {
    pub fn new(item: T) -> Self {
        IsTrait(panic!())
    }
}

fn fails() {
    run(async move {
        let collection = IsTrait::new(Box::new(Struct) as Box<dyn Object>);
        async {}.await;
    })
}

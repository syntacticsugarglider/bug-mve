use std::future::Future;
use vessels::{object, Kind};

fn run<F: Future<Output = ()> + 'static>(_: F) {
    unimplemented!()
}

pub trait Trait: Sized {
    type Type;

    fn consume(self) -> Self;
}

#[object]
pub trait Object<T: Kind> {}

pub struct Struct;

impl Object<()> for Struct {}

impl<T> Trait for Box<dyn Object<T>> {
    type Type = ();

    fn consume(self) -> Self {
        self
    }
}

pub struct IsTrait<T: Trait>(T);

fn fails() {
    run(async move {
        let a = IsTrait(Box::new(Struct) as Box<dyn Object<()>>);
        async {}.await;
    })
}

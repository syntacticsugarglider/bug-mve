use futures::Stream;
use std::future::Future;

fn run<F: Sync + Send + Future<Output = ()> + 'static>(_: F) {
    unimplemented!()
}

trait Object: Sync + Send {}

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
        let a: Box<dyn Object> = panic!();
        let collection = IsTrait::new(a);
        async {}.await;
    })
}

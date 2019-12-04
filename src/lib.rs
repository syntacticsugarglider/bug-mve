use std::future::Future;
use vessels::{
    object,
    replicate::{collections::List, React},
    Kind,
};

fn run<F: Sync + Send + Future<Output = ()> + 'static>(_: F) {
    unimplemented!()
}

trait Trait: Sized {
    type Type;

    fn consume(self) -> Self;
}

#[object]
trait Object<T: Kind> {}

#[derive(Kind)]
struct Struct;

impl Object<()> for Struct {}

impl<T: Kind> Trait for Box<dyn Object<T>> {
    type Type = ();

    fn consume(self) -> Self {
        self
    }
}

struct IsTrait<T: Trait + Kind>(T);

fn fails() {
    run(async move {
        //let a = IsTrait(Box::new(Struct) as Box<dyn Object<()>>);
        let collection = React::new(Box::new(vec![]) as Box<dyn List<String>>);
        async {}.await;
    })
}

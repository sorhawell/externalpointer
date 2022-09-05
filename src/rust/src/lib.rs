use extendr_api::prelude::*;

#[derive(Debug, Clone)]
#[extendr]
struct MyStruct(String); //could be any type

#[extendr]
impl MyStruct {
    pub fn new() -> Self {
        MyStruct("applepie".into())
    }

    //restore_from_robj must take Robj, not MyStruct as input
    pub fn restore_from_robj(robj: Robj) -> Self {
        let res: Result<ExternalPtr<MyStruct>> = robj.try_into(); // this fails
        let x = res.unwrap().0.clone();
        MyStruct(x)
    }
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod helloextendr;
    impl MyStruct;
}

#[cfg(test)]
mod tests {
    use super::*;
    use extendr_api::prelude::extendr_engine::{end_r, start_r};

    #[test]

    //build and install package to access it from rust side
    fn it_works() {
        start_r();
        let robj = R!("helloextendr:::MyStruct$new()").unwrap();
        dbg!(&robj);
        let mystruct = MyStruct::restore_from_robj(robj);
        assert_eq!(mystruct.0, "applpie".to_string());
        end_r();
    }
}

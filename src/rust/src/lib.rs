use extendr_api::prelude::*;

//Part 1: one intended struct to downcast to from Any
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

//Part 2: one WRONG struct to downcast to from Any
#[derive(Debug, Clone)]
#[extendr]
struct SomethingElse(String); //could be any type
#[extendr]
impl SomethingElse {
    pub fn new() -> Self {
        SomethingElse("blueberrypie".into())
    }

    pub fn try_restore_from_robj(robj: Robj) -> Self {
        let res: Result<ExternalPtr<MyStruct>> = robj.try_into(); // this fails
        let x = res.unwrap().0.clone();
        SomethingElse(x)
    }
}

//Part 3: one WORSTCASE struct to downcast to from Any
mod worstcasemodule {
    use super::*;

    #[derive(Debug, Clone)]
    #[extendr] //NOTE will comile with and without this macro
    pub struct MyStruct(pub i32); //could be any type

    //NOTE only MyStruct can be exported. Will not compile if uncomment line below
    //#[extendr]
    impl MyStruct {
        #[allow(unused)]
        pub fn new() -> Self {
            MyStruct(42)
        }

        #[allow(unused)]
        pub fn restore_from_robj(robj: Robj) -> Self {
            let res: Result<ExternalPtr<MyStruct>> = robj.try_into(); // this fails
            let x = res.unwrap().0.clone();
            MyStruct(x)
        }
    }
}

extendr_module! {
    mod helloextendr;
    impl MyStruct;
    impl SomethingElse;
}

#[cfg(test)]
mod tests {
    use super::*;
    use extendr_engine::{end_r, start_r};

    //NOTE yeah normal use works!
    #[test]
    fn get_back_the_right_struct() {
        start_r();
        let robj = R!("helloextendr:::MyStruct$new()").unwrap();
        dbg!(&robj);
        let mystruct = MyStruct::restore_from_robj(robj);
        assert_eq!(mystruct.0, "applepie".to_string());
        end_r();
    }

    //NOTE do stop any wrong typename conversion
    #[test]
    #[should_panic]
    fn try_the_wrong_struct() {
        start_r();
        let robj = R!("helloextendr:::SomethingElse$new()").unwrap();
        dbg!(&robj);
        let mystruct = MyStruct::restore_from_robj(robj);
        assert_eq!(mystruct.0, "applepie".to_string());
        end_r();
    }

    //NOTE ...unless same typename but from to different modules
    //Undefined behaviour!! A String is interpreted as a i32
    #[test]
    #[should_panic]
    fn try_the_worst_case_module() {
        start_r();
        let robj = R!("helloextendr:::MyStruct$new()").unwrap();
        dbg!(&robj);
        let mystruct = worstcasemodule::MyStruct::restore_from_robj(robj);
        println!("mystruct is NOT 42 yikes, it is: {:?}", mystruct);
        end_r();
    }
}

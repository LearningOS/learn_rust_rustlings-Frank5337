// structs1.rs
// Address all the TODOs to make the tests pass!

//

struct ColorClassicStruct {
    // TODO: Something goes here
    name: &'static str,
    hex: &'static str,
}

// 面向C 过程式
fn new(name: &'static str, hex: &'static str) -> ColorClassicStruct {
    ColorClassicStruct{name, hex}
}


impl ColorClassicStruct {
    fn new(name: &'static str, hex: &'static str) -> Self {
        Self{name, hex}
    }
}

struct ColorTupleStruct(/* TODO: Something goes here */
    &'static str, &'static str
);

#[derive(Debug)]
struct UnitStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // TODO: Instantiate a classic c struct!
        // 结构体
        // let green = ColorClassicStruct {
        //     name: "green",
        //     hex: "#00FF00",
        // };
        let green = ColorClassicStruct::new("green", "#00FF00");

        assert_eq!(green.name, "green");
        assert_eq!(green.hex, "#00FF00");
    }

    #[test]
    fn tuple_structs() {
        // TODO: Instantiate a tuple struct!
        //元组结构体
        let green = ColorTupleStruct("green", "#00FF00");

        assert_eq!(green.0, "green");
        assert_eq!(green.1, "#00FF00");
    }

    #[test]
    fn unit_structs() {
        // TODO: Instantiate a unit struct!
        // 单元结构体
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitStructs are fun!");
    }
}

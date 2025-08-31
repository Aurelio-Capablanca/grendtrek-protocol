use crate::common::grend_trek_error::StopTrek;

macro_rules! hashmap {
    ($ ($key : expr => $val : expr), *) => {{
        let mut map = ::std::collections::HashMap::new();
        $(map.insert($key, $val); )*
        map
    }};
}

pub fn translate_ddl() -> Result<(), StopTrek>{

    Ok(())
 }
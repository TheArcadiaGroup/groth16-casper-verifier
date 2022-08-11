use types::{CLType, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter};

pub fn endpoint(name: &str, param: Vec<Parameter>, ret: CLType) -> EntryPoint {
    EntryPoint::new(
        String::from(name),
        param,
        ret,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn gamma_miller_loop() -> EntryPoint {
    endpoint(
        "gamma_miller_loop",
        vec![
            Parameter::new("i", CLType::U8),
            Parameter::new("j", CLType::U8),
            Parameter::new("input", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
    )
}

pub fn delta_miller_loop() -> EntryPoint {
    endpoint(
        "delta_miller_loop",
        vec![
            Parameter::new("i", CLType::U8),
            Parameter::new("j", CLType::U8),
            Parameter::new("input", CLType::List(Box::new(CLType::U8))),
        ],
        CLType::Unit,
    )
}

pub fn final_exponentiation() -> EntryPoint {
    endpoint(
        "final_exponentiation",
        vec![
            Parameter::new("i", CLType::U8),
            Parameter::new("j", CLType::U8),
            Parameter::new("input", CLType::List(Box::new(CLType::U8))),
            Parameter::new("keys", CLType::List(Box::new(CLType::Key))),
        ],
        CLType::Unit,
    )
}

pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(gamma_miller_loop());
    entry_points.add_entry_point(delta_miller_loop());
    entry_points.add_entry_point(final_exponentiation());
    entry_points
}

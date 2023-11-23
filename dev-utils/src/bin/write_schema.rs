use schemars::schema_for;

pub fn main() {
    let schema = schema_for!(glance_app::AppData);
    println!(
        "{}",
        serde_json::to_string_pretty(&schema).expect("Serializing schema")
    );
}

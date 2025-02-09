use sellershut_core::users::User;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User::default();

    let user_json = serde_json::to_string(&user)?;

    println!("{user_json}");
    Ok(())
}

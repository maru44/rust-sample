use clean::infra::persistence::pg::new_pg;

#[tokio::main]
async fn main() {
    let pg = new_pg().await;

    match pg {
        Err(e) => {
            eprintln!("failed to start postgres: {}", e);
            return;
        }
        Ok(p) => {}
    }
}

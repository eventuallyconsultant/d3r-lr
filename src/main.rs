use tide::prelude::*;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    app.at("/boite-aux-lettres").get(get_boite_aux_lettres);
    app.at("/boite-aux-lettres").post(post_boite_aux_lettres);

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_boite_aux_lettres(_req: Request<()>) -> tide::Result {
    Ok(format!("You won't get anything here !").into())
}

async fn post_boite_aux_lettres(mut req: Request<()>) -> tide::Result {
    let body: String = req.body_string().await?;

    dbg!(&body);

    if body.eq("") {
        Ok(format!("Didn't you forget something ?").into())
    } else {
        Ok(format!("Sending invitation").into())
    }
}

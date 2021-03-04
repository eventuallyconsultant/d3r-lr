use askama::Template;
use askama_tide::into_response;
use tide::Request;

use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// #[structopt(name = "env")]
// struct Opt {
//     #[structopt(long, env = "SLACK_API_TOKEN")]
//     slack_api_token: String,
// }

// lazy_static::lazy_static! {
//   static ref SERVER_OPTIONS: Opt = {
//     Opt::from_args()
//   };
// }

#[async_std::main]
async fn main() -> tide::Result<()> {
    // dbg!(&SERVER_OPTIONS.slack_api_token);

    let mut app = tide::new();

    app.at("/").get(get_index);
    app.at("/").serve_dir("public")?;
    app.at("/boite-aux-lettres").get(get_boite_aux_lettres);
    app.at("/boite-aux-lettres").post(post_boite_aux_lettres);

    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    name: &'a str,
}

async fn get_index(_req: Request<()>) -> tide::Result {
    let index = IndexTemplate { name: "world" };
    Ok(into_response(&index, "html").into())
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

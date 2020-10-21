use actix_web::{web, get, App, HttpRequest, HttpServer};
use matecito::Matecito;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Matecito::<String, String>::new(10_000));
    HttpServer::new(move || {
        App::new()
            // .route("/", web::get().to(greet))
            // .route("/{name}", web::get().to(greet))
            .app_data(data.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn index(req: HttpRequest, data: web::Data<Matecito<String, String>>) -> String {
    let asd = req.query_string().to_string();
    let asd: Vec<_> = asd.split("=").collect();
    if asd.len() == 2 {
        let (key, value) = (asd[0].to_owned(), asd[1].to_owned());
        let app_name = data.get(key.clone()).unwrap_or("not found".to_string()); // <- get app_name
        data.put(key, value);
        return format!("Hello {}!\n", app_name) // <- response with app_name
    }
    format!("Hello!\n") // <- response with app_name

}
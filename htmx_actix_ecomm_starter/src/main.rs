use actix_htmx::{Htmx, HtmxMiddleware, TriggerType};
use actix_web::{web::{self, Form}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct BannerForm {
    banner_color: String,
    banner_title: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ProductForm {
    product_image: String,
    product_title: String,
    product_price: String,
    product_description: String,
}

async fn index(htmx: Htmx) -> impl Responder {
    if htmx.is_htmx {
        // build a partial view
    } else {
        // build a full view
    }

    htmx.trigger_event(
        "my_event".to_string(),
        Some(r#"{"level": "info", "message": "my event message!"}"#.to_string()),
        Some(TriggerType::Standard)
    );
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("index.html")) // replace this with constructed view?
}

async fn dashboard() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("dashboard.html")) 
}

async fn update_banner(Info: Form<BannerForm>) -> impl Responder {
    let data = BannerForm{
        banner_color: Info.banner_color.clone(),
        banner_title: Info.banner_title.clone(),
    };
    println!("BNNER\n {} \n {}", &data.banner_color, &data.banner_title);
    // Update the banner color and title here
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8") //("text/html; charset=utf-8")
        .body(format!("<div style='background-color: {};'>{}</div>", data.banner_color, data.banner_title)) // replace with actual banner HTML
}

async fn update_products(Info: Form<ProductForm>) -> impl Responder {
    let data = ProductForm{
        product_image: Info.product_image.clone(), 
        product_title: Info.product_title.clone(), 
        product_price: Info.product_price.clone(), 
        product_description: Info.product_description.clone()
    };
    println!("product\n {} \n {} \n {} \n {}", &data.product_image, &data.product_title, &data.product_price, &data.product_description);
    // Update the products here
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8") //("text/html; charset=utf-8")
        .body(format!("<div><img src='{}'><h2>{}</h2><p>Price: {}</p><p>{}</p></div>", data.product_image, data.product_title, data.product_price, data.product_description)) // replace with actual product HTML
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("Server is running on port 8080");
        App::new()
            .wrap(HtmxMiddleware)
            .service(web::resource("/").to(index))
            .route("/dashboard", web::get().to(dashboard))
            .route("/update_banner", web::post().to(update_banner))
            .route("/update_products", web::post().to(update_products))
    })
    .bind("127.0.0.1:8080")?
    .workers(4)
    .run()
    .await
}

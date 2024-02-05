use std::sync::Mutex;
use actix_web::{web::{self, Form}, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use tera::{Tera, Context};
use futures::stream::once;
use futures::Stream;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
struct Banner {
    banner_color: String,
    banner_title: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Product {
    product_image: String,
    product_title: String,
    product_price: String,
    product_description: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct AppState {
    form_data: Mutex<HashMap<String, String>>,
    banner: Mutex<Banner>,
    products: Mutex<Product>,
    index: Mutex<String>,
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_state = data.get_ref();
    let banner = app_state.banner.lock().unwrap();
    let products = app_state.products.lock().unwrap();

    // Create a Tera instance and add your template
    let mut tera = match Tera::new("templates/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let footer_visible = true; //toggle cms dashboard tests
    // Create a context and add your variables
    let mut context = Context::new();
    context.insert("banner", &banner.banner_title);
    context.insert("products", &products.product_title);
    context.insert("footer_class", if footer_visible { "show-footer" } else { "" });

    // Render your template with the context
    let index = tera.render("index.html", &context).unwrap();
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(index)
}
     
async fn dashboard() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("dashboard.html")) 
}

async fn update_banner(data: web::Data<AppState>, form: web::Form<HashMap<String, String>>) -> impl Responder {
    let data = Banner{
        banner_color: form.get("banner_color").unwrap().clone(),
        banner_title: form.get("banner_title").unwrap().clone(),
    };
    println!("BNNER\n {} \n {}", &data.banner_color, &data.banner_title);
    // Update the banner color and title here
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(once(async move { Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: <div id='banner' style='background-color: {}'>{}</div>\n\n", data.banner_color, data.banner_title))) }))
}

async fn update_products(data: web::Data<AppState>, form: web::Form<HashMap<String, String>>) -> impl Responder {
    let data = Product{
        product_image: form.get("product_image").unwrap().clone(), 
        product_title: form.get("product_title").unwrap().clone(), 
        product_price: form.get("product_price").unwrap().clone(), 
        product_description: form.get("product_description").unwrap().clone()
    };
    println!("product\n {} \n {} \n {} \n {}", &data.product_image, &data.product_title, &data.product_price, &data.product_description);
    // Update the products here
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(once(async move { Ok::<_, actix_web::Error>(web::Bytes::from(format!("data: <div id='product-grid'><img src='{}'><h2>{}</h2><p>Price: {}</p><p>{}</p></div>\n\n", data.product_image, data.product_title, data.product_price, data.product_description))) }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        form_data: Mutex::new(HashMap::new()),
        banner: Mutex::new(Banner {
            banner_color: "#c9430b".to_string(),
            banner_title: "default".to_string(),
        }),
        products: Mutex::new(Product {
            product_image: "default".to_string(), 
            product_title: "default".to_string(), 
            product_price: "default".to_string(), 
            product_description: "default".to_string()
        }),
        index: Mutex::new(String::new()),
    });

    HttpServer::new(move || {
        println!("Server is running on port 8080");
        App::new()
            .app_data(app_state.clone())
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

use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::NamedFile;
use fft::FastFourierTransform; 
use num::complex::Complex;
use itertools::{Itertools, Either}; 

extern crate num;

async fn index() -> Result<NamedFile, actix_web::Error> {
    let file = NamedFile::open("static/index.html")?;
    Ok(file)
}

async fn calculate(path: web::Path<Vec<f64>>) -> impl Responder {
    let input: Vec<f64> = path.into_inner();
    let transform = FastFourierTransform::new(input.clone());
    // let output = transform.fft(input);
    let mut vec: Vec<Complex<f64>> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    transform.fft_rec(&mut vec); 
    let result: Vec<(f64, f64)> = vec.iter().map(|c| (c.re, c.im)).collect(); 
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calculate/{values}", web::get().to(calculate))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



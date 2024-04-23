use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::NamedFile;
use fft::FastFourierTransform; 
use num::complex::Complex;
extern crate num;


async fn index() -> Result<NamedFile, actix_web::Error> {
    let file = NamedFile::open("static/index.html")?;
    Ok(file)
}

async fn calculatefft(path: web::Path<String>) -> impl Responder {

    let input_string = path.into_inner();

    //println!("{}", input_string); 

    let input: Vec<f64> = input_string
        .split(',')
        .map(|s| s.trim().parse::<f64>())
        .filter_map(Result::ok)
        .collect();

    //println!("{:?}", input); 


    let transform = FastFourierTransform::new(input.clone());
    let mut vec: Vec<Complex<f64>> = input
        .iter()
        .map(|&x| Complex::new(x, 0.0))
        .collect();
    transform.fft_rec(&mut vec); 
    let result: Vec<(f64, f64)> = vec.iter().map(|c| (c.re, c.im)).collect(); 

    //println!("{:?}", result); 

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/calculate/{value}", web::get().to(calculatefft))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}






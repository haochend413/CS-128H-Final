use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_files::NamedFile;
use fft::{simd_fft, FastFourierTransform}; 
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


    let vec = if input.len() >= 8 {
        simd_fft(input)
    } else {
        let mut vec = input.iter().map(|&x| Complex::new(x, 0.0)).collect();
        let transform = FastFourierTransform::new(input);
        transform.fft_rec(&mut vec);
        vec
    };

    let result: Vec<(f64, f64)> = vec.iter().map(|c| (c.re, c.im)).collect(); 

    //println!("{:?}", result); 

    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(actix_files::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/calculate/{value}", web::get().to(calculatefft))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

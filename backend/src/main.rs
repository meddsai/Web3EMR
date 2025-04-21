use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user: User,
}

#[derive(Serialize, Clone)]
struct User {
    email: String,
}

const SECRET: &str = "supersecretkey";

// In-memory user for demo
const DEMO_USER_EMAIL: &str = "demo@rustemr.com";
const DEMO_USER_PASSWORD: &str = "password123";

#[post("/api/auth/login")]
async fn login(req: web::Json<LoginRequest>) -> impl Responder {
    if req.email == DEMO_USER_EMAIL && req.password == DEMO_USER_PASSWORD {
        let user = User {
            email: req.email.clone(),
        };
        let claims = serde_json::json!({
            "email": user.email,
            "exp": chrono::Utc::now().timestamp() + 24 * 3600
        });
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .unwrap();
        HttpResponse::Ok().json(LoginResponse { token, user })
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(login))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

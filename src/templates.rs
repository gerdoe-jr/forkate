use askama::Template;

use std::collections::HashMap;

use actix_web::{web, HttpResponse, Result};

// u can do it as macro_derive but no homo only boilerplate
macro_rules! lowkey_include_md {
    ($A:expr) => {
        include_str!(concat!("../templates/pages/", $A, ".md"))
    };
}

pub trait MarkdownContent {
    fn md() -> &'static str;
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct Home;

pub async fn home(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Home {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

impl MarkdownContent for Home {
    fn md() -> &'static str {
        lowkey_include_md!("home")
    }
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct Blog;

pub async fn blog(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Blog {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

impl MarkdownContent for Blog {
    fn md() -> &'static str {
        lowkey_include_md!("blog")
    }
}

#[derive(Template)]
#[template(path = "articles.html")]
pub struct Articles;

pub async fn articles(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Articles {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

impl MarkdownContent for Articles {
    fn md() -> &'static str {
        lowkey_include_md!("articles")
    }
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct Contacts;

pub async fn contacts(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = Contacts {}.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

impl MarkdownContent for Contacts {
    fn md() -> &'static str {
        lowkey_include_md!("contacts")
    }
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct NotFound;

pub async fn not_found(_query: web::Query<HashMap<String, String>>) -> Result<HttpResponse> {
    let s = NotFound {}.render().unwrap();
    Ok(HttpResponse::NotFound().content_type("text/html").body(s))
}

impl MarkdownContent for NotFound {
    fn md() -> &'static str {
        lowkey_include_md!("not_found")
    }
}

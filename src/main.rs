extern crate iron;
extern crate tera;
extern crate time;
extern crate lazy_static;

use std::path::Path;
use lazy_static::lazy_static;
use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::middleware::Handler;
use iron::modifiers::Header;
use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use time::precise_time_ns;
use tera::{Tera, Context, compile_templates};

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }

impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(tera: &'static Tera) -> impl Handler {
    move |_req: &mut Request| -> IronResult<Response> {
        // Using the tera Context struct
        let mut context = Context::new();
        context.insert("product", "Product");
        let html: String = tera.render("index.html", &context).unwrap();

        let content_type = Header(ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![])));
        Ok(Response::with((iron::status::Ok, html, content_type)))
    }
}

lazy_static! {
    pub static ref TERA: Tera = {
        // Use globbing
        let mut tpl_path = String::from(Path::new(file!()).parent().unwrap().to_str().unwrap());
        tpl_path.push_str("/../template/**/*");
        let mut tera = compile_templates!(tpl_path.as_str());

        // and we can add more things to our instance if we want to
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}

fn main() {
    let mut chain = Chain::new(hello_world(&TERA));
    chain.link_before(ResponseTime);
    chain.link_after(ResponseTime);
    Iron::new(chain).http("localhost:3000").unwrap();
}

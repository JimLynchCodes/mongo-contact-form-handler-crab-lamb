use chrono::{TimeZone, Utc};
use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use mongodb::bson::doc;
use serde_json::json;
use std::env;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(hello)).await?;
    Ok(())
}

async fn hello(req: Request, _: Context) -> Result<impl IntoResponse, Error> {

    println!("Connecting to MongoDB...");
    
    let connection_string = env::var("MONGO_CONNECTION_STRING").is_err();
    
    println!("Connection string: {}", connection_string);
    
    let client_options = ClientOptions::parse(
        "mongodb+srv://<username>:<password>@cluster0.hx67j.mongodb.net/myFirstDatabase?retryWrites=true&w=majority",
    )
    .await?;
    let client = Client::with_options(client_options)?;
    let database = client.database("testDB");

    let new_doc = doc! {
       "title": req.body["title"],
       "year": 2020,
       "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
       "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
    };

    let insert_result = movies.insert_one(new_doc.clone(), None).await?;

    println!("New document ID: {}", insert_result.inserted_id);

    Ok(json!({
        "success": true
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn hello_handles() {
        let request = Request::default();
        let expected = json!({
            "message": "Go Serverless v1.0! Your function executed successfully!"
        })
        .into_response();
        let response = hello(request, Context::default())
            .await
            .expect("expected Ok(_) value")
            .into_response();
        assert_eq!(response.body(), expected.body())
    }
}

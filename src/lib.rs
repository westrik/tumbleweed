pub struct App {
    pub name: String,
    pub frontend_url: String,
    pub backend_url: String,
}

// pub enum RequestMethod {
//     DELETE,
//     GET,
//     HEAD,
//     PATCH,
//     POST,
//     PUT,
// }
//
// type ApiResult<A> = Result<A, String>;
//
// #[derive(Response)]
// struct HelloResponse {
//     pub message: String,
// }
//
// #[endpoint("/", POST)]
// async fn hello_world() -> ApiResult<HelloResponse> {
//
// }
//
// #[derive(Entity)]
// struct Note {
//     pub name: String,
// }
//
// #[derive(Request)]
// struct CreateNoteRequest {
//     pub name: String,
// }
// #[derive(Response)]
// struct CreateNoteResponse {
//     pub note: Note,
// }
// #[endpoint("/note", [PUT, POST])]
// async fn create_note(_request: CreateNoteRequest) -> ApiResult<CreateNoteResponse> {
//     Ok(CreateNoteResponse {
//         note: Note {
//             name: "my note".to_string()
//         }
//     })
// }
//
#[cfg(test)]
mod test_configuration {
    use crate::*;

    #[test]
    fn app_creation() {
        let app = App {
            name: "Tumbleweed Test App".to_string(),
            frontend_url: "https://www.example.com".to_string(),
            backend_url: "https://api.example.com".to_string(),
        };
    }
}

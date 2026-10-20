use chrono::{ DateTime, Local };


#[derive(Debug)]
struct Bookmark {
    name: String,
    target: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>
}

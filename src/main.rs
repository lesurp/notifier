use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Default)]
struct Notifications {
    counter: usize,
    lines: Vec<(usize, String)>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    const MAX_NOTIF: usize = 10;

    let notifs = Arc::new(Mutex::new(Notifications::default()));

    let n = notifs.clone();
    let show_notifs = warp::get().map(move || {
        n.lock()
            .unwrap()
            .lines
            .iter()
            .fold(String::new(), |all, (count, line)| {
                format!("{}{} - {}\n", all, count, line).to_owned()
            })
    });

    let n = notifs.clone();
    let clear_notifs = warp::get().and(warp::path("clear")).map(move || {
        let n = n.lock();
        let mut n = n.unwrap();
        n.lines.clear();
        n.counter = 0;
        "OK CLEAR"
    });

    let n = notifs.clone();
    let add_notif = warp::post()
        .and(warp::path("add"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(move |new_notif| {
            let n = n.lock();
            let mut n = n.unwrap();
            let idx = n.counter % MAX_NOTIF;
            let new_elem = (n.counter + 1, new_notif);
            if n.lines.len() < MAX_NOTIF {
                n.lines.push(new_elem);
            } else {
                n.lines[idx] = new_elem;
            }
            n.counter += 1;
            "OK ADD"
        });

    warp::serve(clear_notifs.or(show_notifs).or(add_notif))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

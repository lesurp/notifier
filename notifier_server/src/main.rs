use clap::{App, Arg};
use log::error;
use notifier_common::{
    CreateNotification, CreateNotificationResult, DeleteNotification, DeleteNotificationResult,
    GetNotifications, GetNotificationsResult, NotificationContent,
};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use warp::{body, reply, Filter};

#[derive(Default)]
struct NotificationByTag(usize, HashSet<NotificationContent>);
#[derive(Default)]
struct Notifications(HashMap<String, NotificationByTag>);

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("Notifier server")
        .version("0.1")
        .author("Paul Lesur <git@lesurpaul.fr>")
        .about("Notify you of whatever you want")
        .arg(
            Arg::with_name("interface")
                .short("b")
                .long("bind")
                .value_name("INTERFACE")
                .help("Interface to bind against e.g. 127.0.0.1:8080")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let interface_input = matches.value_of("interface").unwrap();
    let interface = match interface_input.parse::<std::net::SocketAddr>() {
        Ok(interface) => interface,
        Err(_) => {
            error!(
                "Error: 'interface' argument ({}) is not a valid interface to bind to",
                interface_input
            );
            return;
        }
    };

    let notifs = Arc::new(Mutex::new(Notifications::default()));

    let n = notifs.clone();
    let delete_notif = warp::delete()
        .and(body::content_length_limit(1024 * 16))
        .and(body::json())
        .map(move |delete_notif: DeleteNotification| {
            let n = n.lock();
            let mut n = n.unwrap();
            let mut notifs_for_this_tag = n.0.get_mut(&delete_notif.tag);
            reply::json(&match &mut notifs_for_this_tag {
                None => DeleteNotificationResult::TagDoesNotExist,
                Some(notifs_for_this_tag) => {
                    if notifs_for_this_tag.1.remove(&delete_notif.id) {
                        DeleteNotificationResult::Ok
                    } else {
                        DeleteNotificationResult::IdDoesNotExist
                    }
                }
            })
        });

    let n = notifs.clone();
    let get_notifs = warp::get()
        .and(body::content_length_limit(1024 * 16))
        .and(body::json())
        .map(move |get_notifs: GetNotifications| {
            let n = n.lock();
            let mut n = n.unwrap();
            let notifs_for_this_tag = n.0.get_mut(&get_notifs.tag);
            reply::json(&match &notifs_for_this_tag {
                None => GetNotificationsResult::TagDoesNotExist,
                Some(notifs_for_this_tag) => {
                    GetNotificationsResult::Ok(notifs_for_this_tag.1.iter().cloned().collect())
                }
            })
        });

    let n = notifs.clone();
    let add_notif = warp::put()
        .and(body::content_length_limit(1024 * 16))
        .and(body::json())
        .map(move |notification_push: CreateNotification| {
            let n = n.lock();
            let mut n = n.unwrap();
            let tag = notification_push.tag;
            let content = notification_push.content;
            let notif_for_this_tag = n.0.entry(tag).or_default();
            let id = notif_for_this_tag.0;
            notif_for_this_tag.0 += 1;
            notif_for_this_tag
                .1
                .insert(NotificationContent { id, content });
            reply::json(&CreateNotificationResult::Ok(id))
        });

    warp::serve(delete_notif.or(get_notifs).or(add_notif))
        .run(interface)
        .await;
}

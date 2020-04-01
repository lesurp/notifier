use clap::{App, Arg, SubCommand};
use notifier_common::{
    CreateNotification, CreateNotificationResult, DeleteNotification, DeleteNotificationResult,
    GetNotifications, GetNotificationsResult,
};

#[derive(Debug)]
enum Error {
    Reqwest(reqwest::Error),
    TagDoesNotExist,
    IdDoesNotExist,
    Unknown,
}

fn get<RefS: AsRef<str>, S: Into<String>>(url: RefS, tag: S) -> Result<(), Error> {
    let req = GetNotifications { tag: tag.into() };

    let notifs: GetNotificationsResult = reqwest::blocking::Client::new()
        .get(url.as_ref())
        .json(&req)
        .send()
        .map_err(Error::Reqwest)?
        .json()
        .map_err(Error::Reqwest)?;

    match notifs {
        GetNotificationsResult::Ok(notifs) => {
            for notif in notifs {
                println!("{}: {}", notif.id, notif.content);
            }
            Ok(())
        }
        GetNotificationsResult::TagDoesNotExist => Err(Error::TagDoesNotExist),
    }
}

fn push<RefS: AsRef<str>, S: Into<String>>(url: RefS, tag: S, content: S) -> Result<usize, Error> {
    let req = CreateNotification {
        tag: tag.into(),
        content: content.into(),
    };

    let pushed_id: CreateNotificationResult = reqwest::blocking::Client::new()
        .put(url.as_ref())
        .json(&req)
        .send()
        .map_err(Error::Reqwest)?
        .json()
        .map_err(Error::Reqwest)?;

    match pushed_id {
        CreateNotificationResult::Ok(id) => Ok(id),
        CreateNotificationResult::Err => Err(Error::Unknown),
    }
}

fn delete<RefS: AsRef<str>, S: Into<String>>(url: RefS, tag: S, id: usize) -> Result<(), Error> {
    let req = DeleteNotification {
        tag: tag.into(),
        id,
    };

    let success: DeleteNotificationResult = reqwest::blocking::Client::new()
        .delete(url.as_ref())
        .json(&req)
        .send()
        .map_err(Error::Reqwest)?
        .json()
        .map_err(Error::Reqwest)?;

    match success {
        DeleteNotificationResult::Ok => Ok(()),
        DeleteNotificationResult::TagDoesNotExist => Err(Error::TagDoesNotExist),
        DeleteNotificationResult::IdDoesNotExist => Err(Error::IdDoesNotExist),
    }
}

fn main() {
    let matches = App::new("Notifier client")
        .version("0.1")
        .author("Paul Lesur <git@lesurpaul.fr>")
        .about("Notify you of whatever you want")
        .arg(
            Arg::with_name("url")
                .help("URL of the server")
                .required(true),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("retrieve notifications for a given tag from the server")
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag that should be retrieved from the server")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("push")
                .about("push notification to the server")
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag that should be retrieved from the server")
                        .required(true),
                )
                .arg(
                    Arg::with_name("CONTENT")
                        .help("Actual notification")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("delete")
                .about("push notification to the server")
                .arg(
                    Arg::with_name("TAG")
                        .help("Tag that should be retrieved from the server")
                        .required(true),
                )
                .arg(
                    Arg::with_name("ID")
                        .help("Id of the notification to delete")
                        .required(true),
                ),
        )
        .get_matches();

    let url = matches.value_of("url").unwrap();

    if let Some(matches) = matches.subcommand_matches("get") {
        let res = get(url, matches.value_of("TAG").unwrap());
        if let Err(e) = res {
            println!("Error getting notifications: {:?}", e)
        }
    } else if let Some(matches) = matches.subcommand_matches("push") {
        let res = push(
            url,
            matches.value_of("TAG").unwrap(),
            matches.value_of("CONTENT").unwrap(),
        );
        match res {
            Ok(id) => println!("Created notification with id: {}", id),
            Err(e) => println!("Error getting notifications: {:?}", e),
        }
    } else if let Some(matches) = matches.subcommand_matches("delete") {
        let id = matches.value_of("ID").unwrap().parse::<usize>();

        match id {
            Ok(id) => {
                let res = delete(url, matches.value_of("TAG").unwrap(), id);
                match res {
                    Ok(_) => println!("Notification deleted"),
                    Err(e) => println!("Error getting notifications: {:?}", e),
                }
            }
            Err(e) => {
                println!("Input id is not a correct id: {}", e);
            }
        }
    } else {
        println!("{}", matches.usage());
    }
}

//! Thearesia a tool to maintain GitHub Repos and Organizations
#![feature(plugin)]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate futures;
extern crate github_rs;
extern crate hyper;
extern crate serde;
extern crate serde_json;

use futures::{Future, Stream};
use futures::future;
use hyper::server::{Service, Http};
use hyper::header::*;
use hyper::server;
use hyper::error;
use hyper::Method::Post;
use hyper::status::StatusCode::{BadRequest, MethodNotAllowed};
use serde_json::Value;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::borrow::Cow;

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let _ = Http::new()
        .bind(&socket, || Ok(Webhook))
        .map(|server| server.run())
        .map_err(|e| println!("Server failed to setup: {}", e));
}

struct Webhook;

impl Service for Webhook {
    type Request = server::Request;
    type Response = server::Response;
    type Error = error::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;
    fn call(&self, req: server::Request) -> Self::Future {
        let (method, _, _, headers, body) = req.deconstruct();

        // Make sure we only recieve POST requests from Github
        if method == Post {

            // Because this is a custom field we need to look at the Raw
            let raw_event = headers
                .get_raw("X-Github-Event")
                .and_then(|raw| raw.one())
                .and_then(|raw| Some(String::from_utf8_lossy(raw)));

            // If the UserAgent header exists and is from GitHub-Hookshot
            // then true else false because the request is invalid
            let agent = headers.get::<UserAgent>()
                .map_or(false, |user_agent| {
                    match user_agent {
                        &UserAgent(ref raw) => raw.starts_with("GitHub-Hookshot"),
                    }
                });

            let event_type: Event;

            // If the headers are good try to assign an Event value
            // if that fails for some reason send a 400 to GitHub
            if let (true, Some(event_cow)) = (agent, raw_event) {
                match parse_event(event_cow) {
                    Ok(event) => event_type = event,
                    Err(bad) => return bad,
                }
            } else {
                return bad_request();
            }

            // Get all of the chunks streamed to us in our request
            // GitHub gives us a lot of data so there might be
            // more than one Chunk
            body.fold(Vec::new(), |mut v, chunk| {
                    v.extend(&chunk[..]);
                    future::ok::<_, error::Error>(v)
                })
                // If there is JSON do things with it
                // Send to the server that we got the data
                .map(move |buffer| {
                    if !buffer.is_empty() {
                        // For now this is generic but when each event type is implemented
                        // deserialize to specific types
                        let deserialized = serde_json::from_slice::<Value>(&buffer).unwrap();
                        use self::Event::*;
                        match event_type {
                            WildCard => println!("{:#?}", deserialized),
                            CommitComment => println!("{:#?}", deserialized),
                            Create => println!("{:#?}", deserialized),
                            Delete => println!("{:#?}", deserialized),
                            Deployment => println!("{:#?}", deserialized),
                            DeploymentStatus => println!("{:#?}", deserialized),
                            Fork => println!("{:#?}", deserialized),
                            Gollum => println!("{:#?}", deserialized),
                            IssueComment => println!("{:#?}", deserialized),
                            Issues => println!("{:#?}", deserialized),
                            Label => println!("{:#?}", deserialized),
                            Member => println!("{:#?}", deserialized),
                            Membership => println!("{:#?}", deserialized),
                            Milestone => println!("{:#?}", deserialized),
                            Organization => println!("{:#?}", deserialized),
                            PageBuild => println!("{:#?}", deserialized),
                            ProjectCard => println!("{:#?}", deserialized),
                            ProjectColumn => println!("{:#?}", deserialized),
                            Project => println!("{:#?}", deserialized),
                            Public => println!("{:#?}", deserialized),
                            PullRequestReviewComment => println!("{:#?}", deserialized),
                            PullRequestReview => println!("{:#?}", deserialized),
                            PullRequest => println!("{:#?}", deserialized),
                            Push => println!("{:#?}", deserialized),
                            Repository => println!("{:#?}", deserialized),
                            Release => println!("{:#?}", deserialized),
                            Status => println!("{:#?}", deserialized),
                            Team => println!("{:#?}", deserialized),
                            TeamAdd => println!("{:#?}", deserialized),
                            Watch => println!("{:#?}", deserialized),
                        }
                    }

                    server::Response::new()
                }).boxed()

        } else {

            let mut res = server::Response::new();
            res.set_status(MethodNotAllowed);
            future::ok(res).boxed()

        }
    }
}

fn bad_request() -> Box<Future<Error = error::Error, Item = server::Response>> {
    let mut res = server::Response::new();
    res.set_status(BadRequest);
    future::ok(res).boxed()
}

#[derive(Debug, PartialEq, Eq)]
enum Event {
    WildCard,
    CommitComment,
    Create,
    Delete,
    Deployment,
    DeploymentStatus,
    Fork,
    Gollum,
    IssueComment,
    Issues,
    Label,
    Member,
    Membership,
    Milestone,
    Organization,
    PageBuild,
    ProjectCard,
    ProjectColumn,
    Project,
    Public,
    PullRequestReviewComment,
    PullRequestReview,
    PullRequest,
    Push,
    Repository,
    Release,
    Status,
    Team,
    TeamAdd,
    Watch,
}

// Parse the string taken from the header and turn it into an Event if possible.
// If that's not possible return a 400 Future for GitHub
fn parse_event(event_cow: Cow<str>) -> Result<Event, Box<Future<Error = error::Error, Item = server::Response>>> {
        if let Cow::Borrowed(event) = event_cow {
            match event {
                "*" => Ok(Event::WildCard),
                "commit_comment" => Ok(Event::CommitComment),
                "create" => Ok(Event::Create),
                "delete" => Ok(Event::Delete),
                "deployment" => Ok(Event::Deployment),
                "deployment_status" => Ok(Event::DeploymentStatus),
                "fork" => Ok(Event::Fork),
                "gollum" => Ok(Event::Gollum),
                "issue_comment" => Ok(Event::IssueComment),
                "issues" => Ok(Event::Issues),
                "label" => Ok(Event::Label),
                "member" => Ok(Event::Member),
                "membership" => Ok(Event::Membership),
                "milestone" => Ok(Event::Milestone),
                "organization" => Ok(Event::Organization),
                "page_build" => Ok(Event::PageBuild),
                "project_card" => Ok(Event::ProjectCard),
                "project_column" => Ok(Event::ProjectColumn),
                "project" => Ok(Event::Project),
                "public" => Ok(Event::Public),
                "pull_request_review_comment" => Ok(Event::PullRequestReviewComment),
                "pull_request_review" => Ok(Event::PullRequestReview),
                "pull_request" => Ok(Event::PullRequest),
                "push" => Ok(Event::Push),
                "repository" => Ok(Event::Repository),
                "release" => Ok(Event::Release),
                "status" => Ok(Event::Status),
                "team" => Ok(Event::Team),
                "team_add" => Ok(Event::TeamAdd),
                "watch" => Ok(Event::Watch),
                _ => Err(bad_request())
            }
        } else {
            Err(bad_request())
        }
}

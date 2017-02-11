extern crate serde_json;
use github_rs::github::{IssueComment, CommitComment};
use github_rs::github::{Assignees, Issues};
use nom::IResult::Done;
use self::Command::*;
use client::gen_client;

pub fn commit_comment(raw_json: Vec<u8>) {
    let commit_comment = serde_json::from_slice::<CommitComment>(&raw_json).unwrap();
    println!("{}", commit_comment.comment.body);
}

pub fn issue_comment(raw_json: Vec<u8>) {
    let issue_comment = serde_json::from_slice::<IssueComment>(&raw_json).unwrap();

    // We don't care if this was a comment that was changed or deleted. We want to check
    // for new comments and deal with them.
    if &issue_comment.action == "created" {
        if let Some(comment) = issue_comment.comment.clone() {
            // r? @mgattozzi - Assign mgattozzi as reviewer
            // @thearesia r+ - Assigned reviewer approves the PR
            // @thearesia r- - Assigned reviewer rejects the PR
            // @thearesia r+ rollup - Assigned reviewer approves the PR and wants it in a rollup commit
            // @thearesia close - Close PR
            match parse_command(&comment.body.as_bytes()) {
                Done(_, ChangeReviewer(s)) => change_rev(issue_comment, s),
                Done(_, AcceptPr) => println!("Accept"),
                Done(_, RejectPr) => println!("Reject"),
                Done(_, Rollup) => println!("Rollup"),
                Done(_, Close) => println!("Close"),
                _ => println!("Ignore"),
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Command {
    ChangeReviewer(String),
    AcceptPr,
    RejectPr,
    Rollup,
    Close,
}

named!(parse_command <Command>, alt!(
      change_reviewer => { |res: &[u8]| {
        ChangeReviewer(String::from_utf8(res.to_vec()).unwrap()) }}
    | accept_pr => { |res: &[u8]|
        if res == b"rollup" {
            Rollup
        } else {
            AcceptPr
        }
    }
    | reject_pr => { |_|  RejectPr}
    | close_pr => { |_|  Close}
));

named!(change_reviewer <&[u8]> , do_parse!(
    take_until!("r?") >>
    take_until!("@") >>
    take!(1) >>
    res: take_while!(not_whitespace) >>
    (res)
));

fn not_whitespace(val: u8) -> bool {
    let c = val as char;
    if c == '\r' || c == '\n' || c == ' ' || c == '\t' {
        false
    } else {
        true
    }
}

named!(accept_pr, do_parse!(
    take_until!("@thearesia r+") >>
    rollup: opt!(take_until!("rollup")) >>
    // We found the rollup word so it's a some
    // otherwise pass along an empty slice
    (match rollup {
        Some(_) => b"rollup",
        None => b""
    })
));

named!(reject_pr, take_until!("@thearesia r-"));
named!(close_pr, take_until!("@thearesia close"));

fn change_rev(issue_comment: IssueComment, reviewer: String) {
    let client = gen_client();
    let user = Assignees {
        assignees: vec![reviewer],
    };

    println!("{:#?}", issue_comment.clone());
    let issue = issue_comment.issue;
    let issue_num = issue.number;
    let mut url_split = issue.repository_url.split('/');

    // Avoid all the extra bits from the repo url
    for _ in 0..4 {
        let _ = url_split.next();
    }
    // Extract the owner and repo for the patch
    if let (Some(owner), Some(repo)) = (url_split.next(), url_split.next()) {

        if let Some(remove) = issue.assignees {

            let rem_assignees = Assignees {
                assignees: remove.into_iter()
                                 .map(|x| x.login)
                                 .collect(),
            };

            let _ = client.remove_assignees(&owner, &repo, issue_num, rem_assignees);
        }

        match client.add_assignees(&owner, &repo, issue_num, user) {
            Ok(_) => println!("Succesfully added an assignee to {}/{} Issue {}", owner, repo, issue_num),
            Err(e) => println!("{:#?}", e),
        }
    }
}

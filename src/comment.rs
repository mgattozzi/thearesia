extern crate serde_json;
use github_rs::github::{IssueComment, CommitComment, MakeReview};
use github_rs::github::{Assignees, Issues, Reviews, Collaborators};
use nom::IResult::Done;
use self::Command::*;
use client::*;

pub fn commit_comment(raw_json: Vec<u8>) {
    let commit_comment =
        serde_json::from_slice::<CommitComment>(&raw_json)
                    .unwrap();
    println!("{}", commit_comment.comment.body);

}

pub fn issue_comment(raw_json: Vec<u8>) {
    let issue_comment =
        serde_json::from_slice::<IssueComment>(&raw_json)
                    .unwrap();

    // We don't care if this was a comment that was changed or deleted. We want
    // to check for new comments and deal with them.
    if_let_chain! {[
        &issue_comment.action == "created",
        let Some(comment) = issue_comment.comment.clone(),
    ],{
        // r? @mgattozzi - Assign mgattozzi as reviewer
        // @thearesia r+ - Assigned reviewer approves the PR
        // @thearesia r- - Assigned reviewer rejects the PR
        // @thearesia r+ rollup - Assigned reviewer approves the PR and wants it
        // in a rollup commit
        // @thearesia close - Close PR
        use self::ReviewType::*;
        match parse_command(&comment.body.as_bytes()) {
            Done(_, ChangeReviewer(s)) => change_rev(issue_comment, s),
            Done(_, AcceptPr) => review_pr(issue_comment, Approve),
            Done(_, RejectPr) => review_pr(issue_comment, RequestChanges),
            Done(_, Rollup) => println!("Rollup"),
            Done(_, Close) => println!("Close"),
            _ => println!("Ignore"),
        }
    }}
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
    let user = Assignees { assignees: vec![reviewer], };
    let issue = issue_comment.issue;
    let issue_num = issue.number;
    let mut url_split = issue.repository_url.split('/');

    // Avoid all the extra bits from the repo url
    for _ in 0..4 {
        let _ = url_split.next();
    }

    // Extract the owner and repo for the patch
    if_let_chain! {[
        let (Some(owner), Some(repo)) = (url_split.next(), url_split.next()),
        let Some(remove) = issue.assignees,
    ],{

        let rem_assignees = Assignees {
            assignees: remove.into_iter()
                             .map(|x| x.login)
                             .collect(),
        };

        let _ = client.remove_assignees(&owner, &repo, issue_num, rem_assignees);

        match client.add_assignees(&owner, &repo, issue_num, user) {

            Ok(_) => println!("Succesfully added an assignee to {}/{} Issue {}", 
                              owner, repo, issue_num),

            Err(e) => println!("{:#?}", e),
        }
    }}
}


#[derive(Debug, PartialEq, Eq)]
enum ReviewType{
    Approve,
    RequestChanges,
    // No need for them in the workflow.
    // Maybe activate them later.
    // Comment,
    // Pending
}

fn review_pr(issue_comment: IssueComment, rev: ReviewType) {

    let client = gen_collaborator_client();
    let issue = issue_comment.issue;
    let issue_num = issue.number;
    let mut url_split = issue.repository_url.split('/');

    // Avoid all the extra bits from the repo url
    for _ in 0..4 {
        let _ = url_split.next();
    }

    if_let_chain! {[
        // Extract the owner and repo for the patch
        let (Some(owner), Some(repo)) = (url_split.next(), url_split.next()),
        let Some(comment) = issue_comment.comment,
        let Some(user) = comment.user.login,
        client.get_permissions(&owner, &repo, &user)
              .map(|x| x.permission ==  "admin" || x.permission == "write")
              .unwrap_or(false)
    ],{
        let client = gen_review_client();
        let review_id =
            // Get the review_id that the bot is assigned to
            client.get_reviews(&owner, &repo, issue_num)
                  .ok()
                  .and_then(|rev|
                    rev.into_iter()
                       .find(|x| x.user.login
                          == Some("thearesia".to_string()))
                   )
                  .map(|y| y.id);

        use self::ReviewType::*;
        match rev {

            // If there was an r+ have the bot approve the review and then
            // merge the code if possible
            Approve => {
                if let Some(id) = review_id {
                    let _ = client.delete_review(&owner,
                                                 &repo,
                                                 issue_num,
                                                 id);

                    let mk_review = MakeReview {
                        body: format!("PR has been approved by `{}`", user),
                        event: "APPROVE".to_string(),
                        comments: vec![],
                    };

                    let _ = client.post_review(&owner,
                                               &repo,
                                               issue_num,
                                               mk_review);
                }
            },

            // If there was an r- have the bot deny the review
            RequestChanges => {
                if let Some(id) = review_id {
                    let _ = client.delete_review(&owner,
                                                 &repo,
                                                 issue_num,
                                                 id);

                    let mk_review = MakeReview {
                        body: format!("PR has been denied by `{}`.", user),
                        event: "REQUEST_CHANGES".to_string(),
                        comments: vec![],
                    };

                    let _ = client.post_review(&owner,
                                               &repo,
                                               issue_num,
                                               mk_review);
                }
            },

        }
    }}
}

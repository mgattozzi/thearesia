//! Methods to sync and update data between GitHub and Airtable
use airtable::AirtableClient;
use error::*;
use github_rs::client::{ Github, Executor };
use hyper::StatusCode;
use json::AssignedIssuesRecord;
use serde_json;

use std::collections::HashSet;
use std::{ thread, time };

pub fn sync_new_assigned_issues(gclient: &Github, aclient: &mut AirtableClient) -> Result<()> {
    let issues = gclient.get()
                        .issues()
                        .filter("assigned")
                        .state("open")
                        .paginated_execute::<serde_json::Value>()?;


    let open_github_issues = issues.into_iter()
                   .filter_map(|(_,_,i)| {
                        // We can unwrap because we know these are all indeed Strings for these
                        // fields
                        let status = "Assigned".to_string();
                        let issue  = i["html_url"].as_str().unwrap().to_string();
                        let opened = i["created_at"].as_str().unwrap().to_string();
                        let closed = None;
                        let repo   = i["repository"]["full_name"].as_str().unwrap().to_string();
                        let title  = i["title"].as_str().unwrap().to_string();

                        if issue.contains("/pull") {
                            return None;
                        }

                        Some(AssignedIssuesRecord {
                            status,
                            issue,
                            opened,
                            closed,
                            repo,
                            issue_title: title,
                        })
                   }).collect::<HashSet<AssignedIssuesRecord>>();


    let current_open_issues = aclient.get_assigned_issues()?
                                     .records
                                     .into_iter()
                                     .map(|i| i.fields)
                                     .filter(|i| i.status != "Completed")
                                     .collect::<HashSet<AssignedIssuesRecord>>();

    let not_inserted_issues = open_github_issues.difference(&current_open_issues);

    for i in not_inserted_issues {
        loop {
            match aclient.create_assigned_issue(i)? {
                (StatusCode::TooManyRequests, _) => thread::sleep(time::Duration::new(5,0)),
                (StatusCode::Ok, _) => break,
                (a@_, err_json) => return Err(ThearesiaFailure::StatusCodeFail{
                    error: a.to_string() + " " + &err_json.to_string()
                }),
            }
        }
    }

    Ok(())
}

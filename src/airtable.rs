use error::ThearesiaFailure;
use hyper::{ Client, Method, Uri, Request };
use hyper::header::{ Authorization, Bearer, ContentType };
use hyper::StatusCode;
use hyper_rustls::HttpsConnector;
use tokio_core::reactor::Core;
use std::str::FromStr;
use { AIRTABLE_KEY, ASSIGNED_ISSUES_URL };
use json::*;
use serde_json::{ self, Value };
use futures::future::Future;
use futures::Stream;

fn auth() -> Authorization<Bearer> {
    Authorization(Bearer {token: AIRTABLE_KEY.to_string()})
}

pub struct AirtableClient {
    client: Client<HttpsConnector>,
    core: Core
}

impl AirtableClient {
    pub fn new() -> Result<Self, ThearesiaFailure> {
        let core = Core::new()?;
        let h = core.handle();
        Ok(Self{
            client: Client::configure()
                           .connector(HttpsConnector::new(4, &h))
                           .build(&h),
            core
        })
    }

    pub fn get_assigned_issues(&mut self)
        -> Result<AssignedIssuesRecordResponse, ThearesiaFailure>
    {
        // If we go over 100 Open issues we'll need to paginate
        let mut request = Request::new(
            Method::Get,
            Uri::from_str(&ASSIGNED_ISSUES_URL)?
        );
        request.headers_mut().set(auth());
        let work = self.client
                       .request(request)
                       .and_then(|res|
                            res.body().concat2().map(move |chunks| {
                                Ok(serde_json::from_slice(&chunks)?)
                            })
                       );
        self.core.run(work)?
    }

    pub fn create_assigned_issue(&mut self, data: &AssignedIssuesRecord)
        -> Result<(StatusCode, Value), ThearesiaFailure>
    {
        let mut request = Request::new(
            Method::Post,
            Uri::from_str(&ASSIGNED_ISSUES_URL)?
        );
        request.set_body(serde_json::to_vec(
            &CreateAssignedIssuesRecord {
                fields: data.clone()
            }
        )?);
        request.headers_mut().set(auth());
        request.headers_mut().set(ContentType::json());
        let work = self.client
                       .request(request)
                       .and_then(|res| {
                            let status = res.status();
                            res.body().concat2().map(move |chunks| {
                                Ok((status, serde_json::from_slice(&chunks)?))
                            })
                       });
        self.core.run(work)?
    }
}

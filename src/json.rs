use std::fmt;
use std::hash::{ Hash, Hasher };
use serde::de::{self, Visitor, Deserializer};
use serde_json::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Event {
    #[serde(deserialize_with = "parse_event", rename = "type")]
    pub _type: EventType,
    pub public: bool,
    pub payload: Value,
    pub actor: Actor,
    pub repo: Repo,
    pub org: Option<Org>,
    pub created_at: String,
    pub id: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Org {
    pub id: u64,
    pub login: String,
    pub gravatar_id: String,
    pub avatar_url: String,
    pub url: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    CommitCommentEvent,
    CreateEvent,
    DeleteEvent,
    DeploymentEvent,
    DeploymentStatusEvent,
    DownloadEvent,
    FollowEvent,
    ForkEvent,
    ForkApplyEvent,
    GistEvent,
    GollumEvent,
    InstallationEvent,
    InstallationRepositoriesEvent,
    IssueCommentEvent,
    IssuesEvent,
    LabelEvent,
    MarketplacePurchaseEvent,
    MemberEvent,
    MembershipEvent,
    MilestoneEvent,
    OrganizationEvent,
    OrgBlockEvent,
    PageBuildEvent,
    ProjectCardEvent,
    ProjectColumnEvent,
    ProjectEvent,
    PublicEvent,
    PullRequestEvent,
    PullRequestReviewEvent,
    PullRequestReviewCommentEvent,
    PushEvent,
    ReleaseEvent,
    RepositoryEvent,
    StatusEvent,
    TeamEvent,
    TeamAddEvent,
    WatchEvent,
}


fn parse_event<'de, D>(event: D) -> Result<EventType, D::Error>
    where D: Deserializer<'de>
{
    use json::EventType::*;
    match event.deserialize_str(EventTypeVisitor)? {
        "CommitCommentEvent" => Ok(CommitCommentEvent),
        "CreateEvent" => Ok(CreateEvent),
        "DeleteEvent" => Ok(DeleteEvent),
        "DeploymentEvent" => Ok(DeploymentEvent),
        "DeploymentStatusEvent" => Ok(DeploymentStatusEvent),
        "DownloadEvent" => Ok(DownloadEvent),
        "FollowEvent" => Ok(FollowEvent),
        "ForkEvent" => Ok(ForkEvent),
        "ForkApplyEvent" => Ok(ForkApplyEvent),
        "GistEvent" => Ok(GistEvent),
        "GollumEvent" => Ok(GollumEvent),
        "InstallationEvent" => Ok(InstallationEvent),
        "InstallationRepositoriesEvent" => Ok(InstallationRepositoriesEvent),
        "IssueCommentEvent" => Ok(IssueCommentEvent),
        "IssuesEvent" => Ok(IssuesEvent),
        "LabelEvent" => Ok(LabelEvent),
        "MarketplacePurchaseEvent" => Ok(MarketplacePurchaseEvent),
        "MemberEvent" => Ok(MemberEvent),
        "MembershipEvent" => Ok(MembershipEvent),
        "MilestoneEvent" => Ok(MilestoneEvent),
        "OrganizationEvent" => Ok(OrganizationEvent),
        "OrgBlockEvent" => Ok(OrgBlockEvent),
        "PageBuildEvent" => Ok(PageBuildEvent),
        "ProjectCardEvent" => Ok(ProjectCardEvent),
        "ProjectColumnEvent" => Ok(ProjectColumnEvent),
        "ProjectEvent" => Ok(ProjectEvent),
        "PublicEvent" => Ok(PublicEvent),
        "PullRequestEvent" => Ok(PullRequestEvent),
        "PullRequestReviewEvent" => Ok(PullRequestReviewEvent),
        "PullRequestReviewCommentEvent" => Ok(PullRequestReviewCommentEvent),
        "PushEvent" => Ok(PushEvent),
        "ReleaseEvent" => Ok(ReleaseEvent),
        "RepositoryEvent" => Ok(RepositoryEvent),
        "StatusEvent" => Ok(StatusEvent),
        "TeamEvent" => Ok(TeamEvent),
        "TeamAddEvent" => Ok(TeamAddEvent),
        "WatchEvent" => Ok(WatchEvent),
        _ => panic!("Malformed payload"),
    }
}

struct EventTypeVisitor;

impl<'de> Visitor<'de> for EventTypeVisitor {
    type Value = &'de str;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("Expecting an &str for the event type")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(v)
    }

}

#[derive(Debug, PartialOrd, Ord, Serialize, Deserialize, Clone)]
pub struct AssignedIssuesRecord {
    #[serde(rename="Status")]
    pub status: String,
    #[serde(rename="Issue")]
    pub issue: String,
    #[serde(rename="Opened")]
    pub opened: String,
    #[serde(rename="Closed")]
    pub closed: Option<String>,
    #[serde(rename="Repo")]
    pub repo: String,
    #[serde(rename="Issue Title")]
    pub issue_title: String,
}

impl Hash for AssignedIssuesRecord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.issue.hash(state);
    }
}

impl PartialEq for AssignedIssuesRecord {
    fn eq(&self, rhs: &Self) -> bool {
        self.issue.eq(&rhs.issue)
    }
}

impl Eq for AssignedIssuesRecord {}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssignedIssuesRecordResponseItems {
    pub id: String,
    pub fields: AssignedIssuesRecord,
    #[serde(rename="createdTime")]
    pub created_time: String
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct AssignedIssuesRecordResponse {
    pub records: Vec<AssignedIssuesRecordResponseItems>,
    pub offset: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateAssignedIssuesRecord {
    pub fields: AssignedIssuesRecord
}

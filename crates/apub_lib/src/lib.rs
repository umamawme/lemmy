use activitystreams::error::DomainError;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;
use std::marker::PhantomData;
use url::Url;

// for now, limit it to activity routing only, no http sigs, parsing or any of that
// need to route in this order:
// 1. recipient actor
// 2. activity type
// 3. inner object (recursively until object is empty or an url)

// TODO: turn this into a trait in which app has to implement the following functions:
// .checkIdValid() - for unique, instance block etc
// .checkHttpSig::<RequestType>()
// .fetchObject() - for custom http client
// .checkActivity() - for common validity checks
pub struct InboxConfig {
  //actors: Vec<ActorConfig>,
}

impl InboxConfig {
  pub fn shared_inbox_handler() {
    todo!()
  }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum PublicUrl {
  #[serde(rename = "https://www.w3.org/ns/activitystreams#Public")]
  Public,
}

#[async_trait::async_trait(?Send)]
pub trait ActivityHandler {
  type Actor;

  // TODO: also need to check for instance/community blocks in here
  async fn verify(&self, context: &LemmyContext) -> Result<(), LemmyError>;

  // todo: later handle request_counter completely inside library
  async fn receive(
    &self,
    actor: Self::Actor,
    context: &LemmyContext,
    request_counter: &mut i32,
  ) -> Result<(), LemmyError>;
}

pub fn verify_domains_match(a: &Url, b: &Url) -> Result<(), LemmyError> {
  if a.domain() != b.domain() {
    return Err(DomainError.into());
  }
  Ok(())
}

// todo: instead of phantomdata, might use option<kind> to cache the fetched object (or just fetch on construction)
pub struct ObjectId<'a, Kind>(Url, &'a PhantomData<Kind>);

impl<Kind> ObjectId<'_, Kind> {
  pub fn url(self) -> Url {
    self.0
  }
  pub fn dereference(self) -> Result<Kind, LemmyError> {
    // todo: fetch object from http or database
    todo!()
  }
}
use activitystreams::unparsed::UnparsedMutExt;
use activitystreams_ext::UnparsedExtension;
use lemmy_db_schema::DbLanguage;
use lemmy_utils::LemmyError;
use serde::{Deserialize, Serialize};

/// Activitystreams extension to allow (de)serializing additional Post fields
/// `comemnts_enabled` (called 'locked' in Lemmy),
/// `sensitive` (called 'nsfw') and `stickied`.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageExtension {
  pub comments_enabled: Option<bool>,
  pub sensitive: Option<bool>,
  pub stickied: Option<bool>,
  pub language: Option<DbLanguage>,
}

impl PageExtension {
  pub fn new(
    comments_enabled: bool,
    sensitive: bool,
    stickied: bool,
    language: DbLanguage,
  ) -> Result<PageExtension, LemmyError> {
    Ok(PageExtension {
      comments_enabled: Some(comments_enabled),
      sensitive: Some(sensitive),
      stickied: Some(stickied),
      language: Some(language),
    })
  }
}

impl<U> UnparsedExtension<U> for PageExtension
where
  U: UnparsedMutExt,
{
  type Error = serde_json::Error;

  fn try_from_unparsed(unparsed_mut: &mut U) -> Result<Self, Self::Error> {
    Ok(PageExtension {
      comments_enabled: unparsed_mut.remove("commentsEnabled")?,
      sensitive: unparsed_mut.remove("sensitive")?,
      stickied: unparsed_mut.remove("stickied")?,
      language: unparsed_mut.remove("language")?,
    })
  }

  fn try_into_unparsed(self, unparsed_mut: &mut U) -> Result<(), Self::Error> {
    unparsed_mut.insert("commentsEnabled", self.comments_enabled)?;
    unparsed_mut.insert("sensitive", self.sensitive)?;
    unparsed_mut.insert("stickied", self.stickied)?;
    unparsed_mut.insert("language", self.language)?;
    Ok(())
  }
}

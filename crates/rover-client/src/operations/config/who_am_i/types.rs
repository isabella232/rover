use super::runner::config_who_am_i_query;

use houston::CredentialOrigin;

pub(crate) type QueryResponseData = config_who_am_i_query::ResponseData;
pub(crate) type QueryVariables = config_who_am_i_query::Variables;
pub(crate) type QueryActorType = config_who_am_i_query::ActorType;

#[derive(Debug, PartialEq)]
pub struct RegistryIdentity {
    pub id: String,
    pub graph_title: Option<String>,
    pub key_actor_type: Actor,
    pub credential_origin: CredentialOrigin,
}

#[derive(Debug, PartialEq)]
pub enum Actor {
    GRAPH,
    USER,
    OTHER,
}

#[derive(Debug, PartialEq)]
pub struct ConfigWhoAmIInput {}

impl From<ConfigWhoAmIInput> for QueryVariables {
    fn from(_input: ConfigWhoAmIInput) -> Self {
        Self {}
    }
}

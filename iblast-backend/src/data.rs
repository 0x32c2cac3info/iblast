use anyhow::{Error, Result, self as _ah, bail, anyhow};
use async_graphql::{
	self,
	Context,
	dynamic::{
		self,
		Enum as DynEnum, EnumItem as DynEnumItem, 
		Field as DynField, FieldItem as DynFieldItem, 
		FieldFuture as DynFieldFut, 
		FieldValue as DynFieldVal, 
		FieldResult as DynFieldResult, 
		Object as DynObj,
		Schema as DynSchema, SchemaError as DynSchemaErr,
		SimpleObject as DynSimpleObj,
	}, 
	http::GraphiQLSource, 
	EmptyMutation, 
	EmptySubscription,
	Enum, EnumItem,
	Field, FieldItem,
	FieldValue, FieldResult, 
	ID,
	Object, 
	SimpleObject,
};
use async_graphql_poem::*;
use sqlx::{self, FromRow, PgPool};

mod model {
	use super::*;
	/// Identity is basically a foreign key on an API token
	/// 
	/// Specifically it is a surjection (a one-way mapping between an ApiKey
	/// and a Client's Identity).
	/// On delete operations for a given client, ApiKey's must also be deleted.
	/// There exists a global guest identity for initial mobile app setup.
	struct Identity(pub ID);

	/// Each mobile app installation comes with a guest ApiKey. 
	/// Guest clients esp. on mobile get a shared API key 
	struct ApiKey;

	/// A Client is a User. 	
	struct Client {
		id: Identity,
		kind: ClientKind,
		// services is the set of  
		subscriptions: Option<Vec<ServiceId>>,
	}

	/// Unique u64 ID
	struct ServiceId(pub ID);

	#[Object(extends)]
	impl Client {
		#[graphql(external)]
		async fn id(&self) -> &ID {
			&self.id.0
		}

		#[graphql(external)]
		async fn kind(&self) -> &ClientKind {
			&self.kind
		}

		async fn subscriptions(&self, ctx: &Context<'_>) -> &[Subscription] {
			let sub = ctx.data_unchecked::<Service>().lock().await;
			&self.unwrap_or(vec![])
				.into_iter()
				.map(|sid: ServiceId| {
					let sid = ServiceId.0;
					sub.clone().iter().filter(|s: Service| s.id().await == sid)
				})
		}
	}

	#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Debug)]	
	enum ClientKind {
		#[graphql(name="guest", description="not signed in")]
		GUEST,
		#[graphql(name="subscriber", description="has account")]
		SUBSCRIBER,
		#[graphql(name="premium", description="pays for account")]
		PREMIUM,
	}
	
	/// Differentiates between mobile client, web, CLI/API user
	#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq, Debug)]
	enum UserAgentish {
		
	}
	struct Service;
	
}
pub use model::*;

struct Query;

#[Object(extends)]
impl Query {
	async fn setup<'a>(&self, ctx: &'a Context<'_>) -> FieldResult<
}

#[handler]
async fn graphiql() -> impl IntoResponse {
	Html(GraphiQLSource::build().finish())
}


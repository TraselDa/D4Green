use cdrs::{
    authenticators::NoneAuthenticator,
    cluster::{
        session::{
            new as new_session,
            // other option: new_lz4 as new_lz4_session,
            // other option: new_snappy as new_snappy_session
            Session,
        },
        ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool,
    },
    load_balancing::SingleNode,
    query::*,
    Result as CDRSResult,
};
use crate::departement::Departement;
use cdrs::frame::frame_error::CDRSError;
use cdrs::error::*;
use crate::cdrs::frame::TryFromRow;

static CREATE_KEYSPACE_QUERY: &'static str = r#"
  CREATE KEYSPACE IF NOT EXISTS tutorial
    WITH REPLICATION = {
      'class': 'SimpleStrategy',
      'replication_factor': 1
    };
"#;

static ADD_DEPARTEMENT_QUERY: &'static str = r#"
  INSERT INTO eclipsed4g.Departement (nom, code_postal)
    VALUES (?, ?);
"#;
static SELECT_DEPARTEMENT_BY_NAME_QUERY: &'static str = r#"
  SELECT * FROM eclipsed4g.Departement
    WHERE nom = ?;
"#;


pub type CurrentSession = Session<SingleNode<TcpConnectionPool<NoneAuthenticator>>>;

pub fn create_keyspace(session: &mut CurrentSession) -> CDRSResult<()> {
    session.query(CREATE_KEYSPACE_QUERY).map(|_| (()))
}

pub fn create_db_session() -> CDRSResult<CurrentSession> {
    let auth = NoneAuthenticator;
    let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", auth).build();
    let cluster_config = ClusterTcpConfig(vec![node]);
    new_session(&cluster_config, SingleNode::new())
}
pub fn add_departement(
    session: &mut CurrentSession,
    departement: Departement,
) -> CDRSResult<()> {
    session
        .query_with_values(ADD_DEPARTEMENT_QUERY, departement.into_query_values())
        .map(|_| (()))
}
pub fn select_departementByName(
    session: &mut CurrentSession,
    name: String
) -> CDRSResult<Vec<Departement>> {
    let values = query_values!(name);
    session
        .query_with_values(SELECT_DEPARTEMENT_BY_NAME_QUERY, values)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut departements: Vec<Departement> = Vec::with_capacity(rows.len());

            for row in rows {
                departements.push(Departement::try_from_row(row)?);
            }

            Ok(departements)
        })
}
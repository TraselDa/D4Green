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
use crate::departement::*;
use crate::commune::Commune;
use crate::region::Region;
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
  INSERT INTO eclipse_base.Departements (nomdep, code_postal)
    VALUES (?, ?);
"#;
static SELECT_DEPARTEMENT_BY_NAME_QUERY: &'static str = r#"
  SELECT nomdep,nomr FROM eclipse_base.Departements
    WHERE nomdep LIKE %?% LIMIT ?;
"#;
static SELECT_ALL_DEPARTEMENT_QUERY: &'static str = r#"
  SELECT nomdep,nomr FROM eclipse_base.Departements LIMIT ?;
"#;
static SELECT_ALL_DEPARTEMENTS_NAME_QUERY: &'static str = r#"
  SELECT nomdep FROM eclipse_base.Departements;
"#;



static SELECT_ALL_REGION_NAME_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.Region;
"#;
static SELECT_ALL_REGION_BY_NAME_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.Region WHERE nomr LIKE %?%;
"#;
static SELECT_ALL_COMMUNE_QUERY: &'static str = r#"
  SELECT nom_com ,
    code_iris ,
    acces_a_linformation ,
    acces_aux_interfaces_numerique ,
    classement ,
    competences_administratives ,
    competences_numeriques ,
    global_acces ,
    global_competences ,
    nom_iris ,
    nomdep ,
    nomr ,
    population,
    score_global FROM eclipse_base.communes LIMIT 20;
"#;
static SELECT_ALL_COMMUNE_NAME_QUERY: &'static str = r#"
  SELECT nom_com FROM eclipse_base.communes;
"#;
static SELECT_ALL_COMMUNE_BY_NAME_QUERY: &'static str = r#"
  SELECT nom_com,nomdep,nomr FROM eclipse_base.communes WHERE nom_com LIKE %?% LIMIT 20;
"#;
static SELECT_ALL_COMMUNE_BY_DEPARTEMENT_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.communes WHERE nomdep LIKE %?% LIMIT 20;
"#;
static SELECT_ALL_COMMUNE_BY_REGION_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.communes WHERE nomr LIKE %?% LIMIT 20;
"#;


pub type CurrentSession = Session<SingleNode<TcpConnectionPool<NoneAuthenticator>>>;

pub fn create_keyspace(session: &mut CurrentSession) -> CDRSResult<()> {
    session.query(CREATE_KEYSPACE_QUERY).map(|_| (()))
}
pub fn create_db_session() -> CDRSResult<CurrentSession> {
    let auth = NoneAuthenticator;
    let node = NodeTcpConfigBuilder::new("146.59.196.39:9042", auth).build();
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
pub fn select_departements_by_name(session: &mut CurrentSession, name: String,limit:i32) -> CDRSResult<Vec<Departement>> {
    let values = query_values!(name,limit);
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
pub fn select_departements(
    session: &mut CurrentSession,limit:i32) -> CDRSResult<Vec<Departement>> {
    let values = query_values!(limit);
    session.query_with_values(SELECT_ALL_DEPARTEMENT_QUERY,values)
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
pub fn select_departements_by_code_postal(session: &mut CurrentSession, code_postal: &String,limit:i32)->CDRSResult<Vec<Departement>>{
    let values = query_values!(limit);
    session.query_with_values(SELECT_ALL_DEPARTEMENT_QUERY,values)
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
pub fn select_departements_name(session: &mut CurrentSession) -> CDRSResult<Vec<DepartementName>> {
    session.query(SELECT_ALL_DEPARTEMENTS_NAME_QUERY)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut departements: Vec<DepartementName> = Vec::with_capacity(rows.len());

            for row in rows {
                departements.push(DepartementName::try_from_row(row)?);
            }

            Ok(departements)
        })
}

pub fn select_regions_name(session: &mut CurrentSession)->CDRSResult<Vec<Region>>{
    session.query(SELECT_ALL_REGION_NAME_QUERY)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut regions: Vec<Region> = Vec::with_capacity(rows.len());

            for row in rows {
                regions.push(Region::try_from_row(row)?);
            }

            Ok(regions)
        })
}

pub fn select_all_communes(session: &mut CurrentSession)->CDRSResult<Vec<Commune>>{
    session.query(SELECT_ALL_COMMUNE_QUERY)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut communes: Vec<Commune> = Vec::with_capacity(rows.len());

            for row in rows {
                communes.push(Commune::try_from_row(row)?);
            }

            Ok(communes)
        })
}
pub fn select_commune_by_name(session: &mut CurrentSession,name:String,limit:i32)->CDRSResult<Vec<Commune>>{
    let values=query_values!(name);
    session.query_with_values(SELECT_ALL_COMMUNE_NAME_QUERY,values)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut communes: Vec<Commune> = Vec::with_capacity(rows.len());

            for row in rows {
                communes.push(Commune::try_from_row(row)?);
            }

            Ok(communes)
        })
}
pub fn select_commune_by_departement(session: &mut CurrentSession,name:String,limit:i32)->CDRSResult<Vec<Commune>>{
    let values=query_values!(name);
    session.query_with_values(SELECT_ALL_COMMUNE_BY_DEPARTEMENT_QUERY,values)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut communes: Vec<Commune> = Vec::with_capacity(rows.len());

            for row in rows {
                communes.push(Commune::try_from_row(row)?);
            }

            Ok(communes)
        })
}
pub fn select_commune_by_region(session: &mut CurrentSession,name:String)->CDRSResult<Vec<Commune>>{
    let values=query_values!(name);
    session.query_with_values(SELECT_ALL_COMMUNE_BY_REGION_QUERY,values)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut communes: Vec<Commune> = Vec::with_capacity(rows.len());

            for row in rows {
                communes.push(Commune::try_from_row(row)?);
            }

            Ok(communes)
        })
}

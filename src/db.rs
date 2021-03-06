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
use crate::CommunePaging;
use crate::departement::*;
use crate::commune::Commune;
use crate::commune::CommuneName;
use crate::region::Region;
use cdrs::error::*;
use crate::cdrs::frame::TryFromRow;
use serde::{Serialize, Deserialize};


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
static SELECT_DEPARTEMENT_BY_NAME: &'static str = r#"
  SELECT nomdep,nomr,indice FROM eclipse_base.Departements;
"#;


static REGION_FIELDS :&'static str=r#"id,
  nom_com ,
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
    score_global"#;

static SELECT_ALL_REGION_NAME_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.Region;
"#;
static SELECT_ALL_REGION_BY_NAME_QUERY: &'static str = r#"
  SELECT nomr FROM eclipse_base.Region WHERE nomr LIKE %?%;
"#;

static SELECT_ALL_COMMUNE_QUERY: &'static str = r#"
  SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes_temp LIMIT 100 ALLOW FILTERING;
"#;
static SELECT_COMMUNE_BY_IDS: &'static str = r#"
  SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes_temp WHERE id= ? LIMIT 1 ALLOW FILTERING;
"#;
static SELECT_ALL_COMMUNE_NAME_QUERY: &'static str = r#"
  SELECT id,nom_com FROM eclipse_base.communes_temp LIMIT 20 ALLOW FILTERING;
"#;
static SELECT_ALL_COMMUNE_BY_NAME_QUERY: &'static str = r#"
  SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes WHERE nom_com=? LIMIT 20 ALLOW FILTERING;
"#;
static SELECT_ALL_COMMUNE_BY_DEPARTEMENT_QUERY: &'static str = r#"SELECT
id,
  nom_com ,
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
    score_global
 FROM eclipse_base.communes_temp WHERE nomdep = ? LIMIT 20 ALLOW FILTERING"#;

static SELECT_ALL_COMMUNE_BY_REGION_QUERY: &'static str = r#"
  SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes_temp WHERE nomr = ? LIMIT 20 ALLOW FILTERING;
"#;
static SELECT_ALL_COMMUNE_PAGING_QUERY: &'static str = r#"
   SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes_temp WHERE id > ? LIMIT 20 ALLOW FILTERING;
"#;
static SELECT_ALL_COMMUNE_ALL_PARAMS_QUERY:&'static str = r#"
   SELECT id,
  nom_com ,
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
    score_global FROM eclipse_base.communes_temp WHERE id > ? LIMIT 20 ALLOW FILTERING;
"#;

pub type CurrentSession = Session<SingleNode<TcpConnectionPool<NoneAuthenticator>>>;


#[derive(Clone, Debug,Serialize, Deserialize)]
pub struct Statistic {
    total_departements:i32,
    total_communes:i32,
    total_regions:i32,

}

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
    let values = query_values!(format!("{}",limit));
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

pub fn select_all_communes(session: &mut CurrentSession,page:i32,last_commune:Option<i32>)->CDRSResult<Vec<Commune>>{
    let mut select:String;
    let mut params;
    match last_commune {
            Some(last_id)=> {
                select = SELECT_ALL_COMMUNE_PAGING_QUERY.to_string();
                let q=format!("{}",last_id);
                params = query_values!(q);
                session.query_with_values(select,params)
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
            None=>{
                select=SELECT_ALL_COMMUNE_QUERY.to_string();
                session.query(select)
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
        }



}
pub fn select_commune_by_name(session: &mut CurrentSession,name:String,last_id:Option<i32>)->CDRSResult<Vec<Commune>>{
    let values=query_values!(name);
    session.query_with_values(SELECT_ALL_COMMUNE_BY_NAME_QUERY,values)
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
pub fn select_commune_by_departement(session: &mut CurrentSession,name:String,last_id:Option<i32>)->CDRSResult<Vec<Commune>>{
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
pub fn select_commune_by_region(session: &mut CurrentSession,name:String,last_id:Option<i32>)->CDRSResult<Vec<Commune>>{

    let values=match last_id{
        Some(id)=>query_values!(name,id),
        None=>query_values!(name)
    };
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
pub fn select_commune_by_paging(session: &mut CurrentSession,paging: CommunePaging)->CDRSResult<Vec<Commune>>{


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
 pub fn select_communes_name(session: &mut CurrentSession)->CDRSResult<Vec<CommuneName>>{
    session.query(SELECT_ALL_COMMUNE_NAME_QUERY)
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let mut communes: Vec<CommuneName> = Vec::with_capacity(rows.len());
            for row in rows {
                communes.push(CommuneName::try_from_row(row)?);
            }

            Ok(communes)
        })
}
pub fn get_statistics(session: &mut CurrentSession) ->CDRSResult<Statistic>{
    let statistic=Statistic{
        total_departements: get_total_departement(session),
        total_communes: get_total_communes(session),
        total_regions: get_total_regions(session)
    };
    Ok(statistic)

}
fn get_total_communes(session: &mut CurrentSession)->i32{
    match session.query("SELECT count(nom_com) FROM eclipse_base.communes_temp")
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {

            return Ok(30);
        }).ok(){
        Some(x)=>x,
        None=>0
    }
}
fn get_total_departement(session: &mut CurrentSession)->i32{
    match session.query("SELECT count(nomr) FROM eclipse_base.Regions")
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {

            return Ok(30);
        }).ok(){
        Some(x)=>x,
        None=>0
    }
}
fn get_total_regions(session: &mut CurrentSession)->i32{
    match session.query("SELECT count(nomr) FROM eclipse_base.Regions")
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {

            return Ok(30);
        }).ok(){
        Some(w)=>w,
        None=>0
    }
}
/*
pub fn get_commune(session: &mut CurrentSession,ids:String)->CDRSResult<Commune>{
    session.query_with_values(SELECT_COMMUNE_BY_IDS,query_values!(ids))
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
                let commune=Commune::try_from_row(rows[0])?;
            Ok(commune)
        })
}
pub fn get_departement(session: &mut CurrentSession,name:String)->CDRSResult<Departement>{
    session.query_with_values(SELECT_DEPARTEMENT_BY_NAME,query_values!(name))
        .and_then(|res| res.get_body())
        .and_then(|body| {
            body
                .into_rows()
                .ok_or(Error::General("cannot get rows from a response body".to_string()))
        })
        .and_then(|rows| {
            let commune=Departement::try_from_row(rows[0])?;
            Ok(commune)
        })
}*/

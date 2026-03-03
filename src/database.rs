use crate::interact::Interactive;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::sqlite::SqlitePoolOptions;
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

impl Interactive {
    pub async fn database(&self) -> Result<String, Box<dyn Error>> {
        let database = Interactive {
            interactive: self.interactive.clone(),
        };
        let valueunpack = database.intercative().unwrap();
        let connection = SqliteConnectOptions::new()
            .filename("diversity.db")
            .create_if_missing(true);
        let connect = SqlitePoolOptions::new()
            .max_connections(10)
            .connect_with(connection)
            .await
            .unwrap();
        sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS DIVERSITY(
        sourcetaxonid text not null,
        sourcetaxontds text not null,
        sourcetaxonname text not null,
        sourcetaxonpathnames text not null,
        sourcetaxonpathids text not null,
        sourcetaxonsubgenusid text not null,
        sourcetaxongenusname text not null,
        sourcetaxonfamilyid text not null,
        sourcetaxonfamilyname text not null,
        reststring text not null)"#,
        )
        .execute(&connect)
        .await
        .unwrap();

        for i in valueunpack.iter() {
            sqlx::query(
                "INSERT INTO DIVERSITY(sourcetaxonid, sourcetaxontds, sourcetaxonname, sourcetaxonpathnames,sourcetaxonpathids, sourcetaxonsubgenusids, sourcetaxongenusname,  sourcetaxonfamilyid, sourcetaxonfamilyname, reststring) values ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
            ).bind(i.sourcetaxonid.clone())
            .bind(i.sourcetaxontds.clone())
            .bind(i.sourcetaxonname.clone())
            .bind(i.sourcetaxonpathnames.clone())
            .bind(i.sourcetaxonpathids.clone())
            .bind(i.sourcetaxonsubgenusid.clone())
            .bind(i.sourcetaxongenusname.clone())
            .bind(i.sourcetaxonfamilyid.clone())
            .bind(i.sourcetaxonfamilyname.clone())
            .bind(i.reststring.clone())
            .execute(&connect)
            .await.unwrap();
        }
        Ok("The database has been written".to_string())
    }
}

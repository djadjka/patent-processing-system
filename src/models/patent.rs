use cdrs::frame::IntoBytes;
use cdrs::query::QueryExecutor;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")]
#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq, Serialize, Deserialize)]
pub struct Patent {
    serial_number: String,
    registration_date: i64,
    expire_date: i64,
    company: String,
    img: String,
    info: String,
}

impl Patent {
    pub fn insert(
        &self,
        session: crate::services::scylla::CurrentSession,
    ) -> Result<cdrs::frame::Frame> {
        let query = "INSERT INTO test.patent (serial_number, registration_date, expire_date, company, img, info) VALUES (?, ?, ?, ?, ?, ?)";
        let values = query_values!(
            self.serial_number.clone(),
            self.registration_date,
            self.expire_date,
            self.company.clone(),
            self.img.clone(),
            self.info.clone()
        );
        session.query_with_values(query, values)
    }

    pub fn get_by_serial_number(
        serial_number: String,
        session: crate::services::scylla::CurrentSession,
    ) -> std::result::Result<Patent, failure::Error> {
        let query = "SELECT * FROM test.patent WHERE serial_number = ? ";
        let values = query_values!(serial_number);
        let rows = session
            .query_with_values(query, values)?
            .get_body()?
            .into_rows()
            .expect("data");
        if rows.len() == 0 {
            Err(format_err!("patent not found"))
        } else {
            let patent: Patent = Patent::try_from_row(rows[0].clone())?;
            Ok(patent)
        }
    }
}

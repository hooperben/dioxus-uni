use alloy::primitives::{Address, Uint};
use axum::{extract::Query, response::Json};
use serde::{Deserialize, Serialize};

use crate::helpers::uni_v2::get_output_amount;

// input parameters for the /estimate route
#[derive(Deserialize)]
pub struct Params {
    pub pool: Option<String>,
    pub src: Option<String>,
    pub dst: Option<String>,
    pub src_amount: Option<String>,
}

// Define a struct for our JSON response
#[derive(Serialize)]
pub struct Response {
    pub pool: Option<String>,
    pub src: Option<String>,
    pub dst: Option<String>,
    pub amount_out: Option<String>,
    pub error: Option<String>,
}

// Handler that extracts query parameters and returns JSON
pub async fn estimate_handler(Query(params): Query<Params>) -> Json<Response> {
    // Check if required parameters are provided
    if params.pool.is_none()
        || params.src.is_none()
        || params.dst.is_none()
        || params.src_amount.is_none()
    {
        return Json(Response {
            pool: params.pool,
            src: params.src,
            dst: params.dst,
            amount_out: None,
            error: Some("Missing required parameters".to_string()),
        });
    }

    // Try to parse addresses and amount
    let parse_result = || -> Result<(Address, Address, Address, Uint<256, 4>), String> {
        let pool_address = params
            .pool
            .as_ref()
            .unwrap()
            .parse::<Address>()
            .map_err(|e| format!("Invalid pool address: {}", e))?;

        let src_address = params
            .src
            .as_ref()
            .unwrap()
            .parse::<Address>()
            .map_err(|e| format!("Invalid source address: {}", e))?;

        let dst_address = params
            .dst
            .as_ref()
            .unwrap()
            .parse::<Address>()
            .map_err(|e| format!("Invalid destination address: {}", e))?;

        let amount_in = Uint::<256, 4>::from_str_radix(params.src_amount.as_ref().unwrap(), 10)
            .map_err(|e| format!("Invalid amount: {}", e))?;

        Ok((pool_address, src_address, dst_address, amount_in))
    };

    match parse_result() {
        Ok((pool_address, src_address, dst_address, amount_in)) => {
            // Call get_output_amount function
            match get_output_amount(pool_address, src_address, dst_address, amount_in).await {
                Ok(output) => Json(Response {
                    pool: Some(output.pool.to_string()),
                    src: Some(output.src.to_string()),
                    dst: Some(output.dst.to_string()),
                    amount_out: Some(output.amount_out.to_string()),
                    error: None,
                }),
                Err(e) => Json(Response {
                    pool: params.pool,
                    src: params.src,
                    dst: params.dst,
                    amount_out: None,
                    error: Some(e.to_string()),
                }),
            }
        }
        Err(e) => Json(Response {
            pool: params.pool,
            src: params.src,
            dst: params.dst,
            amount_out: None,
            error: Some(e),
        }),
    }
}

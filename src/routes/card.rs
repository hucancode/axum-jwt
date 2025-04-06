use crate::{
    models::{
        card::{TravelCard, TravelCardWithPasses, TravelPass},
        dto::{CardCreateInfo, TravelCardRechargeInfo, TravelPassCreateInfo},
        Error,
    },
    AppState,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use surrealdb::RecordId;

pub async fn get_all_cards_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            balance,
            last_stop_id,
            created_at,
            updated_at
        FROM travel_card;"
    );
    let cards: Vec<TravelCard> = state.db.query(query).await?.take(0)?;
    Ok(Json(cards))
}

pub async fn get_card_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            balance,
            last_stop_id,
            created_at,
            updated_at,
            (SELECT meta::id(out.id) AS id,
                out.route_id AS route_id,
                out.expiry_date AS expiry_date
            FROM ->has_pass
            WHERE out.expiry_date > time::now()) AS passes
        FROM ONLY travel_card:{id};"
    );
    let card: Option<TravelCardWithPasses> = state.db.query(query).await?.take(0)?;
    Ok(Json(card))
}

pub async fn create_card_handler(
    State(state): State<Arc<AppState>>,
    Json(CardCreateInfo { balance }): Json<CardCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "CREATE ONLY travel_card
        SET balance = {balance},
            created_at = time::now(),
            updated_at = time::now()"
    );
    let result: Option<RecordId> = state.db.query(query).await?.take(0)?;
    Ok(Json(result))
}

pub async fn recharge_card_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(TravelCardRechargeInfo { amount }): Json<TravelCardRechargeInfo>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "UPDATE ONLY travel_card:{id}
        SET balance = balance + {amount},
            updated_at = time::now()"
    );
    let result: Option<TravelCard> = state.db.query(query).await?.take(0)?;
    Ok(Json(result))
}

pub async fn get_all_passes_handler(
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            route_id,
            card_id,
            expiry_date,
            created_at,
            updated_at
        FROM travel_pass;"
    );
    let passes: Vec<TravelPass> = state.db.query(query).await?.take(0)?;
    Ok(Json(passes))
}

pub async fn get_pass_handler(
    Path(id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    let query = format!(
        "SELECT meta::id(id) AS id,
            route_id,
            card_id,
            expiry_date,
            created_at,
            updated_at
        FROM ONLY travel_pass:{id};"
    );
    let pass: Option<TravelPass> = state.db.query(query).await?.take(0)?;
    Ok(Json(pass))
}

pub async fn create_pass_handler(
    State(state): State<Arc<AppState>>,
    Json(TravelPassCreateInfo { card_id, route_id, expiry_date }): Json<TravelPassCreateInfo>,
) -> Result<impl IntoResponse, Error> {
    let mut queries = vec![format!(
        "LET $pass = CREATE ONLY travel_pass
        SET route_id = '{route_id}',
            card_id = '{card_id}',
            expiry_date = '{expiry_date}',
            created_at = time::now(),
            updated_at = time::now()"
    )];

    queries.push(format!(
        "RELATE travel_card:{card_id} ->has_pass-> $pass"
    ));

    let result: Option<RecordId> = state.db.query(queries.join(";")).await?.take(0)?;
    Ok(Json(result))
}

pub async fn validate_card_at_stop_handler(
    Path((card_id, stop_id)): Path<(String, String)>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, Error> {
    // First check if the card has a valid pass for a route that includes this stop
    let validation_query = format!(
        "LET $card = travel_card:{card_id};
         LET $stop = bus_stop:{stop_id};

         LET $has_valid_pass = (
            SELECT count() > 0 AS valid FROM (
                SELECT * FROM $card->has_pass->travel_pass
                WHERE expiry_date > time::now()
                AND route_id IN (
                    SELECT meta::id(id) FROM bus_route
                    WHERE id IN (
                        SELECT in.id FROM $stop<-contain<-bus_route
                    )
                )
            )
         )[0].valid;

         RETURN $has_valid_pass;"
    );

    let has_valid_pass: Option<bool> = state.db.query(validation_query).await?.take(0)?;

    // If no valid pass, check if we can deduct from balance
    if has_valid_pass.unwrap_or(false) {
        // Update the last stop
        let update_query = format!(
            "UPDATE travel_card:{card_id}
             SET last_stop_id = '{stop_id}',
                 updated_at = time::now()
             RETURN {{'valid': true, 'deducted': 0, 'reason': 'Valid pass found'}}"
        );
        let result: Option<RecordId> = state.db.query(update_query).await?.take(0)?;
        Ok(Json(result))
    } else {
        // Deduct from balance if sufficient
        let deduct_query = format!(
            "LET $card = travel_card:{card_id};
             LET $fare = 2.50; -- Standard fare amount

             IF $card.balance >= $fare THEN (
                 UPDATE $card
                 SET balance = $card.balance - $fare,
                     last_stop_id = '{stop_id}',
                     updated_at = time::now()
                 RETURN {{'valid': true, 'deducted': $fare, 'reason': 'Fare deducted from balance'}}
             ) ELSE (
                 RETURN {{'valid': false, 'deducted': 0, 'reason': 'Insufficient balance'}}
             ) END;"
        );
        let result: Option<RecordId> = state.db.query(deduct_query).await?.take(0)?;
        Ok(Json(result))
    }
}

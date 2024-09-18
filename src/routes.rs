use axum::{routing::get, Router};

use crate::handlers::{analytics, transactions};

pub fn get_routes(app: Router) -> Router {
    app.route("/transactions", get(transactions::get_transactions))
        .route("/analytics", get(analytics::get_analytics))
}

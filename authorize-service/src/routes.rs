// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the Aleo SDK library.

// The Aleo SDK library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Aleo SDK library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Aleo SDK library. If not, see <https://www.gnu.org/licenses/>.

use super::*;

use warp::{Filter, Rejection, Reply};

// GET /keygen
pub fn keygen_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::get()
        .and(warp::path("keygen"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(|seed: String| async move {
            let response =
                match tokio_rayon::spawn_fifo(move || keygen::<CurrentNetwork>(&seed)).await {
                    Ok(response) => response,
                    Err(_) => return Err(warp::reject()),
                };
            Ok(warp::reply::json(&response))
        })
}

// POST /authorize
pub fn authorize_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("authorize"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024)) // 1 kilobyte
        .and(warp::body::json())
        .and_then(|request: AuthorizeRequest| async move {
            let response = match tokio_rayon::spawn_fifo(|| authorize(request)).await {
                Ok(response) => response,
                Err(_) => return Err(warp::reject()),
            };
            Ok(warp::reply::json(&response))
        })
}

// POST /sign
pub fn sign_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("sign"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024)) // 1 kilobyte
        .and(warp::body::json())
        .and_then(|request: SignRequest| async move {
            let response = match tokio_rayon::spawn_fifo(|| sign(request)).await {
                Ok(response) => response,
                Err(_) => return Err(warp::reject()),
            };
            Ok(warp::reply::json(&response))
        })
}

// POST /verify
pub fn verify_route() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::post()
        .and(warp::path("verify"))
        .and(warp::path::end())
        .and(warp::body::content_length_limit(1024)) // 1 kilobyte
        .and(warp::body::json())
        .and_then(|request: VerifyRequest| async move {
            let response = match tokio_rayon::spawn_fifo(|| verify(request)).await {
                Ok(response) => response,
                Err(_) => return Err(warp::reject()),
            };
            Ok(warp::reply::json(&response))
        })
}

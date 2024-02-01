use deadpool_redis::redis::AsyncCommands;

use super::Cache;

// Accept 1 request per 3 seconds
const MAX_REQUESTS: u32 = 1;
const RATE_INTERVAL: i64 = 3;

/// Returns `true` if `ip` made more than `MAX_REQUESTS` requests in the last
/// `RATE_INTERVAL` seconds.
pub async fn check_ip(ip: &String) -> Result<bool, Box<dyn std::error::Error>> {
    let mut con = Cache::get_connection().await?;

    let requests_n: u32 = con.incr(ip, 1).await?;

    if requests_n == 1 {
        // IP did not make a request in the last `RATE_INTERVAL` secs
        // Setup expiration for this interval limit
        con.expire(ip, RATE_INTERVAL).await?;
    } else if requests_n > MAX_REQUESTS {
        // IP sent more than `MAX_REQUESTS`
        return Ok(true);
    }

    Ok(false)
}

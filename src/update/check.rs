use diesel::prelude::*;
use tokio::time::{self, Duration, Instant};
use uuid::Uuid;

use crate::{
    db::{ConnectionPool, DbConnection},
    schema,
};

use super::web::get_instagram_user_data;

pub async fn run(pool: &ConnectionPool, update_interval: Duration) {
    println!(
        "Update check set to start every {} seconds...",
        update_interval.as_secs()
    );
    let mut connection: DbConnection = pool.get().expect("Unable to get DB connection"); //TODO error handling

    let mut loop_start_time = Instant::now();
    loop {
        let user_ids = schema::instagram_users::table
            .select(schema::instagram_users::username)
            .load::<String>(&mut connection)
            .expect("Unable to retrieve user IDs from Database"); //TODO error handling

        print!("Checking for updates ({} users)...", &user_ids.len());

        let total_requests: u32 = 0; //TODO get user count

        let request_rate = total_requests as f64 / update_interval.as_secs_f64(); // requests per second
        let rate_limit: u32 = 3; // max requests allowed in a given time frame
        let rate_limit_time_frame = Duration::from_secs(1); // time frame for the rate limit

        let mut requests_made: u32 = 0;
        let mut start_time = Instant::now();

        for ig_user_id in user_ids {
            // check if the rate limit has been reached

            if check_rate_limit(requests_made, rate_limit, start_time, rate_limit_time_frame).await
            {
                // reset the requests made and start time
                requests_made = 0;
                start_time = Instant::now();
            }

            update_ig_user(&mut connection, &ig_user_id).await;

            // increment the requests made
            requests_made += 1;

            // calculate the time to wait before making the next request
            let time_between_requests = Duration::from_secs_f64(1.0 / request_rate as f64);
            time::sleep(time_between_requests).await;
        }

        // sleep for rest of duration
        let loop_elapsed = loop_start_time.elapsed();
        println!("({} seconds)", loop_elapsed.as_secs());
        if loop_elapsed < update_interval {
            let delay_time = update_interval - loop_elapsed;
            time::sleep(delay_time).await;
        }
        loop_start_time = Instant::now();
    }
}

async fn update_ig_user(connection: &mut DbConnection, user_id: &String) {
    // make the request
    let instagram_user = get_instagram_user_data(&user_id)
        .await
        .expect("Instagram request error"); //TODO error handling

    // update *current* data
    let user_uuid: Uuid = diesel::update(schema::instagram_users::table)
        .filter(schema::instagram_users::username.eq(&user_id))
        .set((
            schema::instagram_users::follower_count.eq(instagram_user.follower_count),
            schema::instagram_users::following_count.eq(instagram_user.following_count),
            schema::instagram_users::posts_count.eq(instagram_user.posts_count),
        ))
        .returning(schema::instagram_users::id)
        .get_result(connection)
        .expect("unable to update instagram data"); //TODO error handling

    // add historical datapoint
    diesel::insert_into(schema::instagram_datapoints::table)
        .values((
            schema::instagram_datapoints::user_id.eq(&user_uuid),
            schema::instagram_datapoints::follower_count.eq(instagram_user.follower_count),
            schema::instagram_datapoints::following_count.eq(instagram_user.following_count),
            schema::instagram_datapoints::posts_count.eq(instagram_user.posts_count),
        ))
        .execute(connection)
        .expect("unable to insert instagram datapoint"); //TODO error handling
}

async fn check_rate_limit(
    requests_made: u32,
    rate_limit: u32,
    start_time: Instant,
    time_frame: Duration,
) -> bool {
    if requests_made >= rate_limit {
        let elapsed_time = start_time.elapsed();
        if elapsed_time < time_frame {
            // if the elapsed time is less than the time frame, delay the request
            let delay_time = time_frame - elapsed_time;

            time::sleep(delay_time).await;
            return true;
        }
    }
    false
}

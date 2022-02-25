use aot_backend::api;
use aot_backend::constants::K_FACTOR;
use aot_backend::schema::{map_layout, user};
use aot_backend::util;
use diesel::QueryDsl;
use diesel::{prelude::*, update};

fn main() {
    let pool = util::get_connection_pool();
    let conn = &*pool.get().expect("Could not retrieve connection from pool");

    let level_id = api::util::get_current_levels_fixture(conn)
        .expect("Could not get level id")
        .id;
    let invalid_users = user::table
        .left_join(
            map_layout::table.on(map_layout::player
                .eq(user::id)
                .and(map_layout::level_id.eq(level_id))
                .and(map_layout::is_valid.eq(true))),
        )
        .select(user::id)
        .filter(map_layout::is_valid.is_null())
        .load::<i32>(conn)
        .expect("Could not get invalid users");

    update(user::table)
        .filter(user::id.eq_any(invalid_users))
        .set(user::overall_rating.eq(user::overall_rating - 2.0 * K_FACTOR))
        .execute(conn)
        .expect("Could not update user ratings");

    println!("Ratings have been updated");
}
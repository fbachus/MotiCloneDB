// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 30]
        username -> Varchar,
        #[max_length = 60]
        email -> Varchar,
        #[max_length = 82]
        password -> Varchar,
        weekly_workout_goal -> Nullable<Int4>,
    }
}

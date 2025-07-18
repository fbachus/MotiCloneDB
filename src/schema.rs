// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 15]
        referral_code -> Nullable<Varchar>,
        admin -> Int4,
        #[max_length = 255]
        name -> Varchar,
        challenge_deadline -> Nullable<Timestamp>,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        author_id -> Nullable<Int4>,
        groups_id -> Nullable<Int4>,
        content -> Text,
        time -> Timestamp,
        proof -> Nullable<Bool>,
    }
}

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

diesel::joinable!(groups -> users (admin));
diesel::joinable!(messages -> groups (groups_id));
diesel::joinable!(messages -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    messages,
    users,
);

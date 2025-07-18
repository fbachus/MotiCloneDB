// @generated automatically by Diesel CLI.

diesel::table! {
    commitments (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        groups_id -> Nullable<Int4>,
        amount -> Nullable<Int4>,
    }
}

diesel::table! {
    contributions (id) {
        id -> Int4,
        commitment -> Nullable<Int4>,
        time -> Timestamp,
        amount -> Int4,
    }
}

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
    pools (id) {
        id -> Int4,
        manager -> Nullable<Int4>,
        #[max_length = 255]
        purpose -> Nullable<Varchar>,
        amount -> Int4,
    }
}

diesel::table! {
    reactions (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
        #[max_length = 8]
        emoji -> Varchar,
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

diesel::table! {
    workouts (id) {
        id -> Int4,
        user_id -> Int4,
        proof_path -> Int4,
        verified -> Nullable<Bool>,
    }
}

diesel::joinable!(commitments -> groups (groups_id));
diesel::joinable!(commitments -> users (user_id));
diesel::joinable!(contributions -> commitments (commitment));
diesel::joinable!(groups -> users (admin));
diesel::joinable!(messages -> groups (groups_id));
diesel::joinable!(messages -> users (author_id));
diesel::joinable!(pools -> users (manager));
diesel::joinable!(reactions -> messages (message_id));
diesel::joinable!(reactions -> users (user_id));
diesel::joinable!(workouts -> messages (proof_path));
diesel::joinable!(workouts -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    commitments,
    contributions,
    groups,
    messages,
    pools,
    reactions,
    users,
    workouts,
);

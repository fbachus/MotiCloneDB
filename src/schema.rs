// @generated automatically by Diesel CLI.

diesel::table! {
    Group (id) {
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
    GroupChallenge (challenge_id) {
        challenge_id -> Int4,
        group_id -> Int4,
        #[max_length = 1000]
        description -> Varchar,
        start_date -> Date,
        end_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    GroupMember (id) {
        id -> Int4,
        group_id -> Int4,
        user_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    Message (id) {
        id -> Int4,
        author_id -> Nullable<Int4>,
        groups_id -> Nullable<Int4>,
        content -> Text,
        time -> Timestamp,
        proof -> Bool,
    }
}

diesel::table! {
    Reaction (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        message_id -> Nullable<Int4>,
        #[max_length = 8]
        emoji -> Varchar,
    }
}

diesel::table! {
    User (id) {
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
    UserWeeklyTarget (target_id) {
        target_id -> Int4,
        user_id -> Int4,
        groupchallenge_id -> Nullable<Int4>,
        target_count -> Int4,
        achieved_count -> Int4,
        #[max_length = 255]
        current_progress -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(Group -> User (admin));
diesel::joinable!(GroupChallenge -> Group (group_id));
diesel::joinable!(GroupMember -> Group (group_id));
diesel::joinable!(GroupMember -> User (user_id));
diesel::joinable!(Message -> Group (groups_id));
diesel::joinable!(Message -> User (author_id));
diesel::joinable!(Reaction -> Message (message_id));
diesel::joinable!(Reaction -> User (user_id));
diesel::joinable!(UserWeeklyTarget -> GroupChallenge (groupchallenge_id));
diesel::joinable!(UserWeeklyTarget -> User (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    Group,
    GroupChallenge,
    GroupMember,
    Message,
    Reaction,
    User,
    UserWeeklyTarget,
);

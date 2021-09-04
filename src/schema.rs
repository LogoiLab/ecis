table! {
    areas (id) {
        id -> Int4,
        name -> Text,
        location -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    components (id) {
        id -> Int4,
        manufacturer -> Nullable<Int4>,
        purchase_date -> Nullable<Int4>,
        part_number -> Text,
        category -> Nullable<Int4>,
        sub_category -> Nullable<Int4>,
        description -> Nullable<Text>,
        datasheet -> Nullable<Text>,
        cvalues -> Nullable<Text>,
        mount_style -> Nullable<Int4>,
        package -> Nullable<Int4>,
    }
}

table! {
    locations (id) {
        id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
    }
}

table! {
    manufacturers (id) {
        id -> Int4,
        name -> Text,
        location -> Nullable<Text>,
        customer_id -> Nullable<Text>,
    }
}

table! {
    mount_styles (id) {
        id -> Int4,
        name -> Nullable<Text>,
    }
}

table! {
    packages (id) {
        id -> Int4,
        pvalue -> Nullable<Text>,
    }
}

table! {
    subcategories (id) {
        id -> Int4,
        name -> Text,
        sub_of -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
    }
}

joinable!(areas -> locations (location));
joinable!(components -> categories (category));
joinable!(components -> manufacturers (manufacturer));
joinable!(components -> mount_styles (mount_style));
joinable!(components -> packages (package));
joinable!(components -> subcategories (sub_category));
joinable!(subcategories -> categories (sub_of));

allow_tables_to_appear_in_same_query!(
    areas,
    categories,
    components,
    locations,
    manufacturers,
    mount_styles,
    packages,
    subcategories,
    users,
);

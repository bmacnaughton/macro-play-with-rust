const get_query_type_ITEMS_3: [(&'static str, QueryType); 6usize] = [
    ("$gt", QueryType::Gt),
    ("$lt", QueryType::Lt),
    ("$eq", QueryType::Eq),
    ("$ne", QueryType::Ne),
    ("$in", QueryType::In),
    ("$or", QueryType::Or)
];
const get_query_type_ITEMS_4: [(&'static str, QueryType); 7usize] = [
    ("$gte", QueryType::Gte),
    ("$lte", QueryType::Lte),
    ("$nin", QueryType::Nin),
    ("$mod", QueryType::Mod),
    ("$all", QueryType::All),
    ("body", QueryType::Body),
    ("init", QueryType::Init)
];
const get_query_type_ITEMS_5: [(&'static str, QueryType); 4usize] = [
    ("$size", QueryType::Size),
    ("$type", QueryType::Type),
    ("$keyf", QueryType::KeyF),
    ("merge", QueryType::Merge)
];
const get_query_type_ITEMS_6: [(&'static str, QueryType); 2usize] = [
    ("$slice", QueryType::Slice),
    ("$where", QueryType::Where)
];
const get_query_type_ITEMS_7: [(&'static str, QueryType); 2usize] = [
    ("$exists", QueryType::Exists),
    ("$reduce", QueryType::Reduce)
];
const get_query_type_ITEMS_9: [(&'static str, QueryType); 2usize] = [
    ("mapReduce", QueryType::MapReduce),
    ("$finalize", QueryType::Finalize)
];
const get_query_type_ITEMS_10: [(&'static str, QueryType); 1usize] = [
    ("accumulate", QueryType::Accumulate)
];
const get_query_type_ITEMS_11: [(&'static str, QueryType); 1usize] = [
    ("accumulator", QueryType::Accumulator)
];

fn get_query_type(s: &str) -> u32 {
    match s.len() {
        3usize => {
            for i in 0..get_query_type_ITEMS_3.len() {
                if s == get_query_type_ITEMS_3[i].0 {
                        return get_query_type_ITEMS_3[i].1 as u32;
                    }
            }
            return 0;
        }
        4usize => {
            for i in 0..get_query_type_ITEMS_4.len() {
                if s == get_query_type_ITEMS_4[i].0 {
                        return get_query_type_ITEMS_4[i].1 as u32;
                    }
            }
            return 0;
        }
        5usize => {
            for i in 0..get_query_type_ITEMS_5.len() {
                if s == get_query_type_ITEMS_5[i].0 {
                        return get_query_type_ITEMS_5[i].1 as u32;
                    }
            }
            return 0;
        }
        6usize => {
            for i in 0..get_query_type_ITEMS_6.len() {
                if s == get_query_type_ITEMS_6[i].0 {
                        return get_query_type_ITEMS_6[i].1 as u32;
                    }
            }
            return 0;
        }
        7usize => {
            for i in 0..get_query_type_ITEMS_7.len() {
                if s == get_query_type_ITEMS_7[i].0 {
                        return get_query_type_ITEMS_7[i].1 as u32;
                    }
            }
            return 0;
        }
        9usize => {
            for i in 0..get_query_type_ITEMS_9.len() {
                if s == get_query_type_ITEMS_9[i].0 {
                        return get_query_type_ITEMS_9[i].1 as u32;
                    }
            }
            return 0;
        }
        10usize => {
            for i in 0..get_query_type_ITEMS_10.len() {
                if s == get_query_type_ITEMS_10[i].0 {
                        return get_query_type_ITEMS_10[i].1 as u32;
                    }
            }
            return 0;
        }
        11usize => {
            for i in 0..get_query_type_ITEMS_11.len() {
                if s == get_query_type_ITEMS_11[i].0 {
                        return get_query_type_ITEMS_11[i].1 as u32;
                    }
            }
            return 0;
        }
        _ => 0,
    }
}

use crate::query::Query;
use juniper::{
    EmptyMutation,
    EmptySubscription,
    RootNode,
};

pub type Schema = RootNode<'static, Query, EmptyMutation<()>, EmptySubscription<()>>;

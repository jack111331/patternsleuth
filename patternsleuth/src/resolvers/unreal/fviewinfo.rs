use std::fmt::Debug;

use futures::future::join_all;

use crate::resolvers::{ensure_one, impl_resolver_singleton, unreal::util};

use patternsleuth_scanner::Pattern;

#[derive(Debug, PartialEq)]
#[cfg_attr(
    feature = "serde-resolvers",
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct FViewInfo(pub u64);
impl_resolver_singleton!(all, FViewInfo, |ctx| async {
    /*
    let strings = ctx.scan(util::utf8_pattern("TemporalJitterSequenceLength")).await;
    let refs = util::scan_xrefs(ctx, &strings).await;
    let fns = util::root_functions(ctx, &refs)?;
    eprintln!("Found all strings {:?} {:?} {:?}", &strings, &refs, &fns);
//    write!("Found all strings {:?} {:?} {:?}", &strings, &refs, &fns);
    Ok(Self(ensure_one(fns)?))
    */

    let patterns = [
        "48 ?? ?? 48 ?? ?? ?? 48 ?? ?? ?? 48 ?? ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? 0F 29 ?? ?? 0F 57 ??"
    ];

    let res = join_all(patterns.iter().map(|p| ctx.scan(Pattern::new(p).unwrap()))).await;
    Ok(Self(ensure_one(res.into_iter().flatten())?))
});

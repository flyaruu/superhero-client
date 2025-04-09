// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub(super) struct AddArgs {
    pub name: String,
}

impl From<AddArgs> for super::Reducer {
    fn from(args: AddArgs) -> Self {
        Self::Add { name: args.name }
    }
}

impl __sdk::InModule for AddArgs {
    type Module = super::RemoteModule;
}

pub struct AddCallbackId(__sdk::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `add`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait add {
    /// Request that the remote module invoke the reducer `add` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_add`] callbacks.
    fn add(&self, name: String) -> __sdk::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `add`.
    ///
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::ReducerEventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`AddCallbackId`] can be passed to [`Self::remove_on_add`]
    /// to cancel the callback.
    fn on_add(
        &self,
        callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> AddCallbackId;
    /// Cancel a callback previously registered by [`Self::on_add`],
    /// causing it not to run in the future.
    fn remove_on_add(&self, callback: AddCallbackId);
}

impl add for super::RemoteReducers {
    fn add(&self, name: String) -> __sdk::Result<()> {
        self.imp.call_reducer("add", AddArgs { name })
    }
    fn on_add(
        &self,
        mut callback: impl FnMut(&super::ReducerEventContext, &String) + Send + 'static,
    ) -> AddCallbackId {
        AddCallbackId(self.imp.on_reducer(
            "add",
            Box::new(move |ctx: &super::ReducerEventContext| {
                let super::ReducerEventContext {
                    event:
                        __sdk::ReducerEvent {
                            reducer: super::Reducer::Add { name },
                            ..
                        },
                    ..
                } = ctx
                else {
                    unreachable!()
                };
                callback(ctx, name)
            }),
        ))
    }
    fn remove_on_add(&self, callback: AddCallbackId) {
        self.imp.remove_on_reducer("add", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `add`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_add {
    /// Set the call-reducer flags for the reducer `add` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn add(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_add for super::SetReducerFlags {
    fn add(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("add", flags);
    }
}

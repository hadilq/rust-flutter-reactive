/// Copyright 2020 Hadi Lashkari Ghouchani
///
/// Licensed under the Apache License, Version 2.0 (the "License");
/// you may not use this file except in compliance with the License.
/// You may obtain a copy of the License at
///
/// http://www.apache.org/licenses/LICENSE-2.0
///
/// Unless required by applicable law or agreed to in writing, software
/// distributed under the License is distributed on an "AS IS" BASIS,
/// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
/// See the License for the specific language governing permissions and
/// limitations under the License.
///
/// # Acknowledgment
/// This class is borrowed from `redux-rs = "0.1.0"` crate, but changed the types to match this
/// project's need.
///
///
/// Function signature for a reducer.
pub trait Reducer<State, Action> {
    fn reduce(self: &Self, state: &State, action: &Action) -> State;
}

/// Function signature for a subscription.
///
/// A Subscription will be called, whenever an action is dispatched (and reaches the reducer).
/// It receives a reference to the current state, which might or might not be used.
pub type Subscription<State> = fn(&State);

/// A container holding a state and providing the possibility to dispatch actions.
///
/// A store is defined by the state is holds and the actions it can dispatch.
pub struct Store<State, Action, ReducerType> where ReducerType: Reducer<State, Action> {
    reducer: ReducerType,
    state: State,
    subscriptions: Vec<Subscription<State>>,
    _marker: std::marker::PhantomData<Action>,
}

impl<State, Action, ReducerType: Reducer<State, Action>> Store<State, Action, ReducerType> {
    /// Creates a new store.
    pub fn new(reducer: ReducerType, initial_state: State) -> Self {
        Self {
            reducer,
            state: initial_state,
            subscriptions: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns the current state.
    pub fn state(&self) -> &State {
        &self.state
    }

    /// Dispatches an action which is handles by the reducer, after the store got passed through the middleware.
    /// This can modify the state within the store.
    pub fn dispatch(&mut self, action: Action) {
        self.dispatch_reducer(&action);
    }

    /// Runs the reducer.
    fn dispatch_reducer(&mut self, action: &Action) {
        self.state = self.reducer.reduce(self.state(), action);
        self.dispatch_subscriptions();
    }

    /// Runs all subscriptions.
    pub fn dispatch_subscriptions(&self) {
        for subscription in &self.subscriptions {
            subscription(self.state());
        }
    }

    /// Subscribes a callback to any change of the state.
    ///
    /// Subscriptions will be called, whenever an action is dispatched.
    ///
    /// See [`Subscription`](type.Subscription.html).
    pub fn subscribe(&mut self, callback: Subscription<State>) {
        self.subscriptions.push(callback);
    }
}

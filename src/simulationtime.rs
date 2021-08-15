use actix::{Actor, Addr, Context};

#[derive(Debug, Default)]
struct ChaoticActorPool<A> where A: Actor<Context = Context<A>> {
    actors: Vec<Addr<A>>,
}

impl<A: Actor<Context = Context<A>> + Default> ChaoticActorPool<A> {
    fn inject_defaults(self: &mut Self, count: u32) {
        for _ in 0..count {
            let actor = A::start_default();
            self.actor_setup(actor);
            self.actors.push(actor);
        }    
    }
    
    fn simulate_fluctuation(self: &mut Self, join_rate: f32, leave_rate: f32, ticks: u32) {
        let new_joins = join_rate*(ticks as f32);
        let new_leaves = leave_rate*(ticks as f32);
        self.inject_defaults(unsafe{ new_joins.floor().to_int_unchecked() });
        self.relieve_leaving(unsafe{ new_leaves.floor().to_int_unchecked() });
    }
}

impl<A: Actor<Context = Context<A>>> ChaoticActorPool<A> {
    fn relieve_leaving(self: &mut Self, count: u32) {
        use rand::prelude::*;
        let total_count = self.actors.len();
        let mut selection = (0..total_count).choose_multiple(&mut thread_rng(), std::convert::TryInto::try_into(count).unwrap());
        selection.sort();
        selection.iter().map(|i| self.actors.remove(total_count - i));
    }

    fn actor_setup(self: &mut Self, actor: Addr<A>) {
        unimplemented!();
    }
}
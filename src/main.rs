mod simulationtime;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, Arc, Condvar},
    time::Duration,
    vec,
};

#[macro_use]
extern crate lazy_static;

use actix::prelude::*;
use actix_web::rt::System;
use futures::future::join_all;
use lazy_static::__Deref;
use rand::prelude::*;
use simulationtime::ChaoticActorPool;
use tokio::join;

/// Represents physical place (intersections) in the working zone
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Location {
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl Default for Location {
    fn default() -> Self {
        Location::Unknown
    }
}

#[actix_rt::main]
async fn main() {
    // Initialize actor subsystem
    let system = System::new("fun");

    // Start the real-time clock
    let world_clock = AtomicU32::new(0);
    let tick_tock = std::sync::Condvar::new();

    let clock_pair = Arc::new((world_clock, tick_tock));
    let clock_duplicate = Arc::<(AtomicU32, Condvar)>::clone(&clock_pair);

    let tick_tock_thread = move || loop {
        let (world_clock, tick_tock) = clock_duplicate.deref();
        std::thread::sleep(Duration::from_secs(1));
        world_clock.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        tick_tock.notify_all();
    };
    std::thread::spawn(tick_tock_thread);

    // Create the population
    let mediator = Mediator::start_default();

    let mut courier_pool = simulationtime::ChaoticActorPool::<DeliveryGuy>::default();

    let pool_initial = 2 + thread_rng().next_u32() % 10;
    let (join_rate, defect_rate) = (0.15, 0.15);

    courier_pool.inject_defaults(pool_initial);
    eprintln!("Initialized with {:} couriers.", pool_initial);

    let results = courier_pool
        .actors
        .iter()
        .map(|courier| courier.send(Invitation(mediator.clone())));
    for response in results {
        eprintln!("{:?}", response.await);
    }

    // Start the simulation
    system.run().unwrap();
}

/// The courier - shuttles cargo from restaurants to clients
#[derive(Debug, PartialEq, Eq)]
struct DeliveryGuy {
    /// Amount of packages carried right now
    cargo: u32,
    /// Total packages picked up this shift
    cargo_counter: u32,
    /// Current location
    location: Location,
    /// Current destination
    destination: Option<Location>,
    /// How far from current destination
    remaining_distance: u32,



    /// Management contact
    boss: Option<Addr<Overseer>>,
}

impl Default for DeliveryGuy {
    fn default() -> Self {
        Self {
            cargo: 0,
            cargo_counter: 0,
            location: Location::Unknown,
            destination: None,
            remaining_distance: thread_rng().next_u32() % 100,
            boss: None,
        }
    }
}

impl Actor for DeliveryGuy {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        eprintln!("{:?}", self);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        if self.cargo == 0 {
            return Running::Stop;
        }
        // TODO: notify overseer we're about to defect
        return Running::Continue;
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<Invitation> for DeliveryGuy {
    type Result = ();

    fn handle(
        &mut self,
        Invitation(mediator): Invitation,
        ctx: &mut Self::Context,
    ) -> Self::Result {
        eprintln!("Delivery guy {:?} got an invitation", self);
        mediator.do_send(RequestAssignment);
        eprintln!("Delivery guy {:?} requests assignment", self);
    }
}
impl Handler<Addr<Overseer>> for DeliveryGuy {
    type Result = ();

    fn handle(&mut self, msg: Addr<Overseer>, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
impl Message for Addr<Overseer> {
    type Result = ();
}
impl Handler<DeliveryWorkOrder> for DeliveryGuy {
    type Result = ();

    fn handle(&mut self, msg: DeliveryWorkOrder, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            DeliveryWorkOrder::GoToPickUp(location) => {
                if self.destination.is_none() {
                    self.destination = location.into();
                    self.remaining_distance = thread_rng().next_u32() % 100 + 10;
                }
            }
            DeliveryWorkOrder::GoToDeliver(location) => {
                if self.destination.is_none() && !self.report_details().is_unburdened() {
                    self.destination = location.into();
                    self.remaining_distance = thread_rng().next_u32() % 100 + 10;
                }
            }
            DeliveryWorkOrder::Report => {
                if let Some(ref boss) = self.boss {
                    boss.do_send(DeliveryGuyReport::CheckIn(
                        ctx.address(),
                        self.report_details(),
                    ))
                }
            }
            DeliveryWorkOrder::Relieve => ctx.stop(),
        }
    }
}

impl DeliveryGuy {
    fn report_details(self: &Self) -> ReportDetails {
        ReportDetails {
            picked_up: 0,
            dropped_off: 0,
            cargo: self.cargo,
            cargo_counter: self.cargo,
            location: self.location,
        }
    }
}

/// Overseer - keeps track of couriers and assigns them tasks
#[derive(Default, Debug, Clone)]
struct Overseer {
    delivery_guys: Vec<Addr<DeliveryGuy>>,
    orders_queue: Vec<DeliveryOrder>,
    assignments: HashMap<Addr<DeliveryGuy>, Vec<DeliveryOrder>>,
}
impl Overseer {
    fn has_work(self: &Self) -> bool {
        !self.orders_queue.is_empty()
    }
}
impl Actor for Overseer {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        eprintln!("Overseer starts {:?}", &self);
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}
impl Handler<DeliveryGuyReport> for Overseer {
    type Result = ();

    fn handle(&mut self, msg: DeliveryGuyReport, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            DeliveryGuyReport::EnteredZone(courier) => self.delivery_guys.push(courier),
            DeliveryGuyReport::LeftZone(courier) => {
                if let Some(index) = self.delivery_guys.iter().position(|i| *i == courier) {
                    self.delivery_guys.remove(index);
                }
            }
            DeliveryGuyReport::CheckIn(courier, report) => {
                if report.is_unburdened() && self.has_work() {
                    let order = self.orders_queue.pop().unwrap();

                    courier.do_send(DeliveryWorkOrder::GoToPickUp(order.restaurant));

                    // here goes the brain of the overseer
                    // implement assignments
                }
            }
        }
    }
}

/// Manages Overseers and gets couriers and overseers in touch
#[derive(Debug)]
struct Mediator {
    overseers: Vec<Overseer>,
    overseers_addr: Vec<Addr<Overseer>>,
}

impl Default for Mediator {
    fn default() -> Self {
        let mut overseers = vec![];
        for _ in [..2 + rand::thread_rng().next_u32() % 10] {
            overseers.push(Overseer::default());
        }

        Self {
            overseers: overseers,
            overseers_addr: vec![],
        }
    }
}

impl Actor for Mediator {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        eprintln!(
            "Mediator {:?} starts himself and his {:} overseers!",
            self,
            self.overseers.len()
        );

        for ref overseer in self.overseers.as_slice() {
            let overseed = overseer.clone();
            let addr = overseed.clone().start();
            self.overseers_addr.push(addr);
            eprintln!("Overseer kicked off!");
        }
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

impl Handler<RequestAssignment> for Mediator {
    type Result = Addr<Overseer>;

    fn handle(&mut self, msg: RequestAssignment, ctx: &mut Self::Context) -> Self::Result {
        eprintln!("Mediator got assignment request!");
        let overseer = self.overseers_addr.choose(&mut thread_rng()).unwrap();
        eprintln!("Picked {:?}", overseer);
        overseer.clone()
    }
}

/// Places orders and recieves cargo
#[derive(Default)]
struct Client {
    clock_ref: AtomicU32,

    started_on: u32,
    initial_delay: u32,
    order_complexity: u32,
    home_location: Location,
}
impl Actor for Client {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

/// Responds to orders, creates cargo
struct Restaurant;
impl Actor for Restaurant {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {}

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {}
}

#[derive(Debug, Clone, Copy)]
struct DeliveryOrder {
    order_complexity: u32,
    restaurant: Location,
    creation_time: u32,
}

/// Courier joins the job for the first time, gets introduced to the mediator
struct Invitation(Addr<Mediator>);
impl Message for Invitation {
    type Result = ();
}

struct RequestAssignment;
impl Message for RequestAssignment {
    type Result = Addr<Overseer>;
}

/// Task orders for a courier
enum DeliveryWorkOrder {
    GoToPickUp(Location),
    GoToDeliver(Location),
    Report,
    Relieve,
}

impl Message for DeliveryWorkOrder {
    type Result = ();
}

/// Status reports from a courier to an overseer
enum DeliveryGuyReport {
    EnteredZone(Addr<DeliveryGuy>),
    LeftZone(Addr<DeliveryGuy>),
    CheckIn(Addr<DeliveryGuy>, ReportDetails),
}
struct ReportDetails {
    picked_up: u32,
    dropped_off: u32,
    cargo: u32,
    cargo_counter: u32,
    location: Location,
}
impl ReportDetails {
    fn is_unburdened(self: &Self) -> bool {
        self.cargo == 0
    }
}

impl Message for DeliveryGuyReport {
    type Result = ();
}

use amethyst::{
    core::bundle::{Result, SystemBundle},
    core::ArcThreadPool,
    ecs::{prelude::*, Dispatcher, DispatcherBuilder, System},
    prelude::*,
    Error,
};

pub struct CustomGameData<'a, 'b> {
    core_dispatcher: Dispatcher<'a, 'b>,
    running_dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> CustomGameData<'a, 'b> {
    pub fn update(&mut self, world: &World, running: bool) {
        if running {
            self.running_dispatcher.dispatch(&world.res);
        }
        self.core_dispatcher.dispatch(&world.res);
    }
}

pub struct CustomGameDataBuilder<'a, 'b> {
    pub core: DispatcherBuilder<'a, 'b>,
    pub running: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> Default for CustomGameDataBuilder<'a, 'b> {
    fn default() -> Self {
        CustomGameDataBuilder::new()
    }
}

impl<'a, 'b> CustomGameDataBuilder<'a, 'b> {
    pub fn new() -> CustomGameDataBuilder<'a, 'b> {
        CustomGameDataBuilder {
            core: DispatcherBuilder::new(),
            running: DispatcherBuilder::new(),
        }
    }

    pub fn with_base_bundle<B: SystemBundle<'a, 'b>>(mut self, bundle: B) -> Result<Self> {
        bundle
            .build(&mut self.core)
            .map_err(Error::Core)
            .expect("Cannot bundle");
        Ok(self)
    }

    pub fn with_running<S>(mut self, system: S, name: &str, dependencies: &[&str]) -> Self
    where
        for<'c> S: System<'c> + Send + 'a,
    {
        self.running.add(system, name, dependencies);
        self
    }
}

impl<'a, 'b> DataInit<CustomGameData<'a, 'b>> for CustomGameDataBuilder<'a, 'b> {
    fn build(self, world: &mut World) -> CustomGameData<'a, 'b> {
        let pool = world.read_resource::<ArcThreadPool>().clone();

        let mut core_dispatcher = self.core.with_pool(pool.clone()).build();
        let mut running_dispatcher = self.running.with_pool(pool.clone()).build();

        core_dispatcher.setup(&mut world.res);
        running_dispatcher.setup(&mut world.res);

        CustomGameData {
            core_dispatcher,
            running_dispatcher,
        }
    }
}

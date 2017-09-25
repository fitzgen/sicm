// TODO: should the paths be `ExactSizeIterator` or something? Should they have
// a sense of their resolution?

// TODO: make the paths implement `rayon::iter::ParallelIterator` and make
// `Action` use `rayon`'s parrallel iteration?

/// TODO: how to represent time?
pub struct Time;

/// Maybe this should be a big decimal?
pub type Real = f64;

/// Little gamma
pub trait ConfigurationPath {
    type ConfigurationSpacePoint;

    fn configuration_point_at(&self, time: Time) -> Self::ConfigurationSpacePoint;
}

/// X
pub trait CoordinateFunction {
    type Coordinates;
    type ConfigurationSpacePoint;

    fn coordinates_for_config(&self, point: Self::ConfigurationSpacePoint) -> Self::Coordinates;
}

/// q = X o little gamma
pub struct CoordinatePath<ConfPath, CoordFn>
where
    ConfPath: ConfigurationPath,
    CoordFn: CoordinateFunction,
{
    config_path: ConfPath,
    coordinate_fn: CoordFn,
}

impl<ConfPath, CoordFn> CoordinatePath<ConfPath, CoordFn>
where
    ConfPath: ConfigurationPath,
    CoordFn: CoordinateFunction<ConfigurationSpacePoint = ConfPath::ConfigurationSpacePoint>,
{
    pub fn coordinates_at(&self, time: Time) -> CoordFn::Coordinates {
        self.coordinate_fn
            .coordinates_for_config(self.config_path.configuration_point_at(time))
    }
}

/// Big gamma[q]
///
/// Wouldn't `-> impl Trait` with generic methods be great right now, so we
/// didn't even have to define this trait? Sigh...
pub trait TimeToLocalTuple {
    type LocalTuple;

    fn time_to_local_tuple(&self, time: Time) -> Self::LocalTuple;
}

/// Big gamma
pub trait Gamma {
    type LocalTuple;
    type TimeToLocalTuple: TimeToLocalTuple<LocalTuple = Self::LocalTuple>;

    fn make<ConfPath, CoordFn>(
        &self,
        path: CoordinatePath<ConfPath, CoordFn>,
    ) -> Self::TimeToLocalTuple
    where
        ConfPath: ConfigurationPath,
        CoordFn: CoordinateFunction<ConfigurationSpacePoint = ConfPath::ConfigurationSpacePoint>;
}

/// L
pub trait Lagrangian {
    type LocalTuple;

    fn measure(&self, local_tuple: Self::LocalTuple) -> Real;
}

/// S
pub trait S<ConfPath, CoordFn>
where
    ConfPath: ConfigurationPath,
    CoordFn: CoordinateFunction<ConfigurationSpacePoint = ConfPath::ConfigurationSpacePoint>,
{
    type Lagrangian: Lagrangian;
    type GammaQ: TimeToLocalTuple<LocalTuple = <Self::Lagrangian as Lagrangian>::LocalTuple>;

    fn action(
        &self,
        path: CoordinatePath<ConfPath, CoordFn>,
    ) -> Action<Self::Lagrangian, Self::GammaQ>;
}

/// S[q]
pub struct Action<L, G> {
    lagrangian: L,
    gamma_q: G,
}

impl<L, G> Action<L, G>
where
    L: Lagrangian,
    G: TimeToLocalTuple<LocalTuple = L::LocalTuple>,
{
    pub fn action(&self, from: Time, to: Time) -> Real {
        let _ = (from, to);
        let _ = (&self.lagrangian, &self.gamma_q);

        unimplemented!(
            "how to iterate all moments between `from` and `to`? how much resolution to use? how \
             much resolution is available?"
        )
    }
}
